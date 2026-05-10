# `docker compose` topology

Goal: one command (`docker compose up`) brings up the full stack on
`http://localhost:8080` so that we can iterate on the frontend with a real
backend, real session cookies, and real Forgejo OAuth.

## Services

```
                 ┌──────────────────┐
                 │ web   (Caddy)    │  ← :8080  (browser entry)
                 │  serves dist/    │
                 │  /api → backend  │
                 └─────┬────────────┘
                       │
       ┌───────────────┼────────────────┐
       │               │                │
┌──────▼─────┐  ┌──────▼─────┐  ┌───────▼──────┐
│ backend    │  │ forgejo    │  │ postgres     │
│ class-forge│  │ test SCM   │  │ backend DB   │
│ :8080      │  │ :3000      │  │ :5432        │
└────────────┘  └────────────┘  └──────────────┘
```

Forgejo is bundled so OAuth works end-to-end during local dev without
hitting the real internet. It seeds a test instructor + a couple of
students on first boot.

## `docker-compose.yml` (target)

```yaml
services:
  web:
    build: .
    image: class-forge-web:dev
    ports: ["8080:8080"]
    environment:
      BACKEND_URL: http://backend:8080
    depends_on: [backend]

  backend:
    image: ghcr.io/bloboss/class-forge:latest
    environment:
      DATABASE_URL: postgres://classforge:classforge@db:5432/classforge?sslmode=disable
      FORGEJO_BASE_URL: http://forgejo:3000
      FORGEJO_OAUTH_CLIENT_ID: ${FORGEJO_OAUTH_CLIENT_ID}
      FORGEJO_OAUTH_CLIENT_SECRET: ${FORGEJO_OAUTH_CLIENT_SECRET}
      SESSION_SECRET: dev-only-do-not-use-in-prod
    depends_on: [db, forgejo]

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: classforge
      POSTGRES_USER: classforge
      POSTGRES_PASSWORD: classforge
    volumes: ["dbdata:/var/lib/postgresql/data"]

  forgejo:
    image: codeberg.org/forgejo/forgejo:1.21
    environment:
      USER_UID: "1000"
      USER_GID: "1000"
    volumes: ["forgejo:/data"]
    ports: ["3000:3000"]   # only for inspecting test data

volumes:
  dbdata:
  forgejo:
```

## First-run UX

```
$ cp .env.example .env             # contains FORGEJO_OAUTH_* placeholders
$ docker compose up --build
$ open http://localhost:8080
```

The Forgejo OAuth client must exist before the backend can complete the
flow. We document this in `deploy/README.md` with a one-shot bootstrap
script (`./deploy/bootstrap-forgejo-oauth.sh`) that uses the Forgejo API to
create the OAuth app and writes the credentials into `.env`.

## Production vs. dev

`docker-compose.yml` is dev-only. Production deploys the same `web` and
`backend` images with:

- Real TLS (Caddy auto-issues; or fronted by an existing reverse proxy).
- A managed Postgres.
- A real Forgejo / GitLab.
- `SESSION_SECRET` from a secret store, never a literal.

That deployment is **out of scope** for this roadmap — the goal here is
"fast local iteration with a real backend".
