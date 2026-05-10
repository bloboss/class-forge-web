//! HTTP client core: [`Client`], [`ApiError`], [`Method`], and the
//! [`Transport`] trait that lets tests substitute a mock.

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt;

/// HTTP verbs we use. `gloo_net` accepts a string method, so this enum
/// stays small and avoids pulling in `http::Method`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Method {
    pub fn as_str(self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Patch => "PATCH",
            Method::Delete => "DELETE",
        }
    }

    /// Mutating methods need the CSRF header.
    pub fn is_mutating(self) -> bool {
        !matches!(self, Method::Get)
    }
}

/// Every API call returns this on failure.
///
/// Status mapping mirrors the contract documented in
/// `book/src/architecture/api.md`: a session layer can match on
/// [`ApiError::Unauthorized`] to flip `Session::Anonymous`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError {
    /// 401 — caller is unauthenticated. Session layer listens for this.
    Unauthorized,
    /// Any other 4xx with the response body as a message.
    Client(u16, String),
    /// 5xx — server reported a status code only.
    Server(u16),
    /// Network or transport-level failure (DNS, CORS, abort).
    Network(String),
    /// Body was not valid JSON for the target type.
    Decode(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Unauthorized => f.write_str("unauthorized"),
            ApiError::Client(code, msg) => write!(f, "client error {code}: {msg}"),
            ApiError::Server(code) => write!(f, "server error {code}"),
            ApiError::Network(msg) => write!(f, "network error: {msg}"),
            ApiError::Decode(msg) => write!(f, "decode error: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {}

/// Raw HTTP response handed back by a [`Transport`]. The client decodes
/// `body` based on the caller's target type and `status`.
#[derive(Clone, Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

/// Pluggable HTTP transport. Production uses [`GlooTransport`]; tests
/// (track B3) supply a `MockTransport` that returns canned fixtures.
///
/// `?Send` because the WASM target is single-threaded and `gloo-net`
/// futures are `!Send`.
#[async_trait(?Send)]
pub trait Transport {
    async fn send(
        &self,
        method: Method,
        url: &str,
        csrf: Option<&str>,
        body: Option<String>,
    ) -> Result<Response, ApiError>;
}

/// Default transport, backed by `gloo_net::http::Request`.
pub struct GlooTransport;

#[async_trait(?Send)]
impl Transport for GlooTransport {
    #[cfg(target_arch = "wasm32")]
    async fn send(
        &self,
        method: Method,
        url: &str,
        csrf: Option<&str>,
        body: Option<String>,
    ) -> Result<Response, ApiError> {
        use gloo_net::http::RequestBuilder;
        use web_sys::RequestCredentials;

        let http_method = match method {
            Method::Get => gloo_net::http::Method::GET,
            Method::Post => gloo_net::http::Method::POST,
            Method::Put => gloo_net::http::Method::PUT,
            Method::Patch => gloo_net::http::Method::PATCH,
            Method::Delete => gloo_net::http::Method::DELETE,
        };

        let mut builder = RequestBuilder::new(url)
            .method(http_method)
            .credentials(RequestCredentials::Include);

        if method.is_mutating() {
            if let Some(token) = csrf {
                builder = builder.header("X-CSRF-Token", token);
            }
        }

        let request = if let Some(payload) = body {
            builder
                .header("Content-Type", "application/json")
                .body(payload)
                .map_err(|e| ApiError::Network(e.to_string()))?
        } else {
            builder
                .build()
                .map_err(|e| ApiError::Network(e.to_string()))?
        };

        let response = request
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;
        Ok(Response { status, body })
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn send(
        &self,
        _method: Method,
        _url: &str,
        _csrf: Option<&str>,
        _body: Option<String>,
    ) -> Result<Response, ApiError> {
        Err(ApiError::Network(
            "GlooTransport is only available on wasm32".into(),
        ))
    }
}

/// API client. One instance is created at boot and shared via Leptos
/// context.
pub struct Client {
    base: String,
    csrf: Option<String>,
    transport: Box<dyn Transport>,
}

impl Client {
    /// Build a client with the default [`GlooTransport`]. Reads the
    /// `csrf_token` cookie via `web_sys` so subsequent mutating
    /// requests carry the header.
    pub fn new(base: &str) -> Self {
        Self::with_transport(base, Box::new(GlooTransport), read_csrf_cookie())
    }

    /// Build a client with a custom transport. Used by tests and by
    /// the future [`crate::api::auth`] layer if it ever needs to fan
    /// out to a mock backend.
    pub fn with_transport(base: &str, transport: Box<dyn Transport>, csrf: Option<String>) -> Self {
        Self {
            base: normalize_base(base),
            csrf,
            transport,
        }
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn csrf(&self) -> Option<&str> {
        self.csrf.as_deref()
    }

    /// Compose the final URL for a path. The path may or may not start
    /// with `/`; either form yields exactly one separator.
    pub fn url(&self, path: &str) -> String {
        let trimmed = path.trim_start_matches('/');
        if trimmed.is_empty() {
            self.base.clone()
        } else {
            format!("{}/{}", self.base, trimmed)
        }
    }

    /// Send a JSON request and decode the JSON response.
    pub async fn request<T, B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let payload = match body {
            Some(b) => Some(serde_json::to_string(b).map_err(|e| ApiError::Decode(e.to_string()))?),
            None => None,
        };
        let url = self.url(path);
        let response = self
            .transport
            .send(method, &url, self.csrf.as_deref(), payload)
            .await?;
        decode::<T>(response)
    }

    /// Convenience: GET that decodes JSON. `body` parameter elided.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        self.request::<T, ()>(Method::Get, path, None).await
    }

    /// Convenience: POST with a JSON body that decodes a JSON response.
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.request(Method::Post, path, Some(body)).await
    }
}

fn decode<T: DeserializeOwned>(resp: Response) -> Result<T, ApiError> {
    match resp.status {
        // 204 No Content → caller must use () (or Option) as the target.
        204 => serde_json::from_str("null").map_err(|e| ApiError::Decode(e.to_string())),
        200..=299 => {
            if resp.body.is_empty() {
                serde_json::from_str("null").map_err(|e| ApiError::Decode(e.to_string()))
            } else {
                serde_json::from_str(&resp.body).map_err(|e| ApiError::Decode(e.to_string()))
            }
        }
        401 => Err(ApiError::Unauthorized),
        400..=499 => Err(ApiError::Client(resp.status, resp.body)),
        500..=599 => Err(ApiError::Server(resp.status)),
        other => Err(ApiError::Network(format!("unexpected status {other}"))),
    }
}

fn normalize_base(base: &str) -> String {
    let trimmed = base.trim_end_matches('/');
    trimmed.to_string()
}

#[cfg(target_arch = "wasm32")]
fn read_csrf_cookie() -> Option<String> {
    use wasm_bindgen::JsCast;
    let document = web_sys::window()?.document()?;
    let html_doc = document.dyn_into::<web_sys::HtmlDocument>().ok()?;
    let cookie = html_doc.cookie().ok()?;
    parse_csrf(&cookie)
}

#[cfg(not(target_arch = "wasm32"))]
fn read_csrf_cookie() -> Option<String> {
    None
}

fn parse_csrf(cookie: &str) -> Option<String> {
    for part in cookie.split(';') {
        let entry = part.trim();
        if let Some(rest) = entry.strip_prefix("csrf_token=") {
            return Some(rest.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    //! Unit tests for the API client. These run under the
    //! [`wasm-bindgen-test`] harness wired up by track D1 and exercised
    //! in CI by track G1's `wasm-pack test --headless --firefox` job.
    //!
    //! On `wasm32-unknown-unknown` the `#[wasm_bindgen_test]` attribute
    //! enrolls each function with the runner; on non-wasm hosts the
    //! attribute compiles to a no-op and `cargo test` simply skips
    //! these — that is fine because the gate is wasm-pack.

    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    type Captured = (Method, String, Option<String>, Option<String>);

    /// In-memory transport used solely for client-level unit tests.
    /// The full mock + fixture pipeline lives in track B3.
    struct RecordingTransport {
        last: Rc<RefCell<Option<Captured>>>,
        canned: Response,
    }

    #[async_trait(?Send)]
    impl Transport for RecordingTransport {
        async fn send(
            &self,
            method: Method,
            url: &str,
            csrf: Option<&str>,
            body: Option<String>,
        ) -> Result<Response, ApiError> {
            *self.last.borrow_mut() =
                Some((method, url.to_string(), csrf.map(str::to_string), body));
            Ok(self.canned.clone())
        }
    }

    fn make_client(base: &str, status: u16, body: &str) -> (Client, Rc<RefCell<Option<Captured>>>) {
        let last = Rc::new(RefCell::new(None));
        let transport = RecordingTransport {
            last: last.clone(),
            canned: Response {
                status,
                body: body.to_string(),
            },
        };
        let client = Client::with_transport(base, Box::new(transport), Some("tok-123".into()));
        (client, last)
    }

    #[wasm_bindgen_test]
    fn url_joins_base_and_path() {
        let (client, _) = make_client("/api", 200, "{}");
        assert_eq!(client.url("/classrooms"), "/api/classrooms");
        assert_eq!(client.url("classrooms"), "/api/classrooms");
        assert_eq!(client.url("/"), "/api");
        assert_eq!(client.url(""), "/api");
    }

    #[wasm_bindgen_test]
    fn url_strips_trailing_slash_from_base() {
        let (client, _) = make_client("/api/", 200, "{}");
        assert_eq!(client.url("/healthz"), "/api/healthz");
    }

    #[wasm_bindgen_test]
    fn url_works_with_absolute_base() {
        let (client, _) = make_client("http://localhost:8080/api", 200, "{}");
        assert_eq!(client.url("/auth/me"), "http://localhost:8080/api/auth/me");
    }

    #[wasm_bindgen_test]
    fn parse_csrf_reads_cookie_jar() {
        let jar = "session=abc; csrf_token=xyz-42; theme=dark";
        assert_eq!(parse_csrf(jar).as_deref(), Some("xyz-42"));
    }

    #[wasm_bindgen_test]
    fn parse_csrf_returns_none_when_missing() {
        assert!(parse_csrf("session=abc").is_none());
    }

    #[wasm_bindgen_test]
    fn decode_maps_unauthorized() {
        let r = Response {
            status: 401,
            body: String::new(),
        };
        assert_eq!(decode::<serde_json::Value>(r), Err(ApiError::Unauthorized));
    }

    #[wasm_bindgen_test]
    fn decode_maps_client_4xx() {
        let r = Response {
            status: 422,
            body: "bad".into(),
        };
        assert_eq!(
            decode::<serde_json::Value>(r),
            Err(ApiError::Client(422, "bad".into()))
        );
    }

    #[wasm_bindgen_test]
    fn decode_maps_server_5xx() {
        let r = Response {
            status: 503,
            body: "down".into(),
        };
        assert_eq!(decode::<serde_json::Value>(r), Err(ApiError::Server(503)));
    }

    #[wasm_bindgen_test]
    fn decode_decodes_2xx_json() {
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct Hello {
            name: String,
        }
        let r = Response {
            status: 200,
            body: r#"{"name":"world"}"#.into(),
        };
        assert_eq!(
            decode::<Hello>(r),
            Ok(Hello {
                name: "world".into()
            })
        );
    }

    #[wasm_bindgen_test]
    fn decode_treats_204_as_null() {
        let r = Response {
            status: 204,
            body: String::new(),
        };
        assert_eq!(decode::<Option<serde_json::Value>>(r), Ok(None));
    }

    #[wasm_bindgen_test]
    async fn request_sends_csrf_for_post_only() {
        let (client, last) = make_client("/api", 200, "null");
        let _: () = client
            .request::<(), serde_json::Value>(
                Method::Post,
                "/auth/logout",
                Some(&serde_json::json!({})),
            )
            .await
            .unwrap();
        let captured = last.borrow().clone().unwrap();
        assert_eq!(captured.0, Method::Post);
        assert_eq!(captured.1, "/api/auth/logout");
        assert_eq!(captured.2.as_deref(), Some("tok-123"));
        assert!(captured.3.is_some());
    }

    #[wasm_bindgen_test]
    async fn request_sends_no_body_for_get() {
        let (client, last) = make_client("/api", 200, "null");
        let _: Option<serde_json::Value> = client.get("/auth/me").await.unwrap();
        let captured = last.borrow().clone().unwrap();
        assert_eq!(captured.0, Method::Get);
        assert_eq!(captured.1, "/api/auth/me");
        assert!(captured.3.is_none());
    }
}
