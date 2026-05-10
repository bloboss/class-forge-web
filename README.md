# class-forge-web

Leptos (Rust → WebAssembly) frontend for **ClassForge**, a forge-agnostic
classroom-management system. Backed by the Go REST API at
[bloboss/class-forge](https://github.com/bloboss/class-forge).

This implements the "Hi-Fi v1" screen spec from the design handoff:

- **Onboarding · multi-forge connect** — Forgejo + GitLab live, others queued.
- **Classes dashboard** with a "new classroom" modal.
- **Classroom shell** (persistent sidebar) with:
  - Assignments (default tab) + assignment drill-down
  - Roster (filters, status, grade, last push)
  - New assignment (4-step wizard with live preview)
  - CI / CD & tests (runners, templates, secrets, recent runs)
  - Analytics (KPIs + sparkline)
  - Settings (general, forge binding, danger zone)
- **Theme tweaks** — Warm cream / Studio dark / Cool slate.
- **Hash router** + bottom-left "Jump to…" menu mirroring the prototype.

The frontend currently runs against in-memory mock data that mirrors the
backend's resource models (Classroom, Assignment, Roster, Forge); wiring it to
the real API is a follow-up.

## Develop

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk            # one-time
trunk serve                    # http://localhost:8080
```

`cargo check --target wasm32-unknown-unknown` is a fast validation step.
