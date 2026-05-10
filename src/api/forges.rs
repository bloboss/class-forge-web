//! Wire types and calls for `/api/forges`.
//!
//! These mirror the Go backend's JSON shape directly — owned `String` fields
//! rather than the `&'static str` of `crate::data::Forge`, which exists only
//! to back the prototype. The Onboarding screen reads these types directly
//! once it has been migrated (task **E1**).

use serde::{Deserialize, Serialize};

use super::ApiError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ForgeKind {
    Forgejo,
    Gitlab,
    Github,
    Gitea,
    Bitbucket,
    Codeberg,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ForgeStatus {
    Live,
    Soon,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Forge {
    pub id: String,
    pub kind: ForgeKind,
    pub label: String,
    pub status: ForgeStatus,
    #[serde(default)]
    pub org: Option<String>,
    pub accounts: u32,
    #[serde(default)]
    pub note: String,
}

/// `GET /api/forges` — list every forge integration the backend knows about,
/// both `Live` and `Soon`.
///
/// Until task **B1** lands the real `Client`, this returns the same fixture
/// the prototype used so the screen can render. Once `B1` is in, replace the
/// body with a `Client::get("/forges").await?` call.
pub async fn list() -> Result<Vec<Forge>, ApiError> {
    Ok(mock_forges())
}

fn mock_forges() -> Vec<Forge> {
    use ForgeKind::*;
    use ForgeStatus::*;
    vec![
        Forge {
            id: "forgejo-univ".into(),
            kind: Forgejo,
            label: "Forgejo · forge.cs.pcu.edu".into(),
            status: Live,
            org: Some("cs-dept".into()),
            accounts: 4,
            note: "self-hosted".into(),
        },
        Forge {
            id: "gitlab-cloud".into(),
            kind: Gitlab,
            label: "GitLab Cloud".into(),
            status: Live,
            org: Some("pcu-classes".into()),
            accounts: 2,
            note: String::new(),
        },
        Forge {
            id: "github".into(),
            kind: Github,
            label: "GitHub".into(),
            status: Soon,
            org: None,
            accounts: 0,
            note: "Q3 2026".into(),
        },
        Forge {
            id: "gitea".into(),
            kind: Gitea,
            label: "Gitea".into(),
            status: Soon,
            org: None,
            accounts: 0,
            note: "Q3 2026".into(),
        },
        Forge {
            id: "bitbucket".into(),
            kind: Bitbucket,
            label: "Bitbucket".into(),
            status: Soon,
            org: None,
            accounts: 0,
            note: "Q4 2026".into(),
        },
        Forge {
            id: "codeberg".into(),
            kind: Codeberg,
            label: "Codeberg".into(),
            status: Soon,
            org: None,
            accounts: 0,
            note: "Q4 2026".into(),
        },
        Forge {
            id: "custom".into(),
            kind: Custom,
            label: "Custom (federated)".into(),
            status: Soon,
            org: None,
            accounts: 0,
            note: "preview".into(),
        },
    ]
}
