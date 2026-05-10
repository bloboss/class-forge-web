# Parallel task map

This page is the **single source of truth** for who-can-work-on-what right
now. Each node is a self-contained task that an agent (or human) can pick
up. Edges are *hard* dependencies — if A → B, B is blocked until A is
merged. Anything not connected by an edge can run in parallel.

Detailed task cards (acceptance criteria, file paths, exact commands) live
in [agent task cards](./agents.md). This page is the dependency graph.

## Tracks at a glance

| Track | Theme                       | Can start now? |
| ----- | --------------------------- | -------------- |
| **A** | Auth & session plumbing     | yes            |
| **B** | API client + state refactor | yes            |
| **C** | Docker + compose            | yes            |
| **D** | Test infrastructure         | yes            |
| **E** | Screen migrations to API    | after A1 + B1  |
| **F** | mdbook CI & polish          | yes            |

Tracks A, B, C, D, F have **no inter-track dependencies** at their starting
nodes — four agents can begin simultaneously. Track E gates on the
foundation pieces (`A1`, `B1`).

## Dependency graph (Graphviz DOT)

Render with `dot -Tsvg book/src/roadmap/graph.dot -o graph.svg` once
`graph.dot` is extracted, or paste this block into any DOT viewer.

```dot
digraph roadmap {
    rankdir=LR;
    node [shape=box, style="rounded,filled", fontname="Helvetica", fontsize=11];
    edge [fontname="Helvetica", fontsize=9, color="#555555"];

    subgraph cluster_A {
        label="Track A · Auth"; color="#4338ca"; fontcolor="#4338ca";
        A1 [label="A1\nLogin route + screen\n(stubbed backend)", fillcolor="#eef2ff"];
        A2 [label="A2\nSession context\n(/auth/me hydration)", fillcolor="#eef2ff"];
        A3 [label="A3\nauth_guard wrapper\nfor gated routes", fillcolor="#eef2ff"];
        A4 [label="A4\nLogout + topbar\nuser menu", fillcolor="#eef2ff"];
        A1 -> A2 -> A3 -> A4;
    }

    subgraph cluster_B {
        label="Track B · API & state"; color="#0891b2"; fontcolor="#0891b2";
        B1 [label="B1\napi::Client + ApiError\n(gloo-net)", fillcolor="#ecfeff"];
        B2 [label="B2\nResource<T> + state ctx", fillcolor="#ecfeff"];
        B3 [label="B3\nMockTransport for tests", fillcolor="#ecfeff"];
        B1 -> B2;
        B1 -> B3;
    }

    subgraph cluster_C {
        label="Track C · Deployment"; color="#a16207"; fontcolor="#a16207";
        C1 [label="C1\nDockerfile (multi-stage)\n+ Caddyfile + .dockerignore", fillcolor="#fefce8"];
        C2 [label="C2\ndocker-compose.yml\n(web + backend + db + forgejo)", fillcolor="#fefce8"];
        C3 [label="C3\nForgejo OAuth bootstrap\nscript + .env.example", fillcolor="#fefce8"];
        C1 -> C2 -> C3;
    }

    subgraph cluster_D {
        label="Track D · Tests"; color="#15803d"; fontcolor="#15803d";
        D1 [label="D1\nwasm-bindgen-test harness\n+ trunk-friendly cargo cfg", fillcolor="#f0fdf4"];
        D2 [label="D2\nrouter parse() unit tests", fillcolor="#f0fdf4"];
        D3 [label="D3\nPlaywright smoke against\ndocker compose stack", fillcolor="#f0fdf4"];
        D1 -> D2;
    }

    subgraph cluster_F {
        label="Track F · Docs & CI"; color="#7c3aed"; fontcolor="#7c3aed";
        F1 [label="F1\nmdbook CI build job\n(GitHub Actions)", fillcolor="#faf5ff"];
        F2 [label="F2\nADR template + first ADR\n(stay-on-Leptos)", fillcolor="#faf5ff"];
    }

    subgraph cluster_E {
        label="Track E · Screen migrations"; color="#be185d"; fontcolor="#be185d";
        E1 [label="E1\nOnboarding → forges API", fillcolor="#fdf2f8"];
        E2 [label="E2\nDashboard → classrooms API", fillcolor="#fdf2f8"];
        E3 [label="E3\nClassroomShell →\nassignments + roster API", fillcolor="#fdf2f8"];
        E4 [label="E4\nAssignmentDetail API", fillcolor="#fdf2f8"];
    }

    // Cross-track gates
    A1 -> A3 [style=dashed, label="(uses)"];
    B1 -> A1 [style=dashed, label="(login POST)"];
    B2 -> E1; B2 -> E2; B2 -> E3; B2 -> E4;
    A2 -> E1; A2 -> E2; A2 -> E3; A2 -> E4;
    C2 -> D3 [style=dashed, label="needs stack"];

    // Suggested parallel start points
    start [label="START", shape=ellipse, fillcolor="#fafafa"];
    start -> A1;
    start -> B1;
    start -> C1;
    start -> D1;
    start -> F1;
    start -> F2;
}
```

The DOT source is also available standalone at
[`book/src/roadmap/graph.dot`](./graph.dot) so it can be rendered into the
book or PRs without copy-paste.

## Parallel start plan (six agents, day one)

Spawn one agent per node below — they only touch disjoint files:

| Agent | Task | Files touched (exclusive)                      |
| ----- | ---- | ---------------------------------------------- |
| α     | A1   | `src/screens/login.rs`, `src/router.rs` (add variants), `src/app.rs` (route arm) |
| β     | B1   | `src/api/`, `Cargo.toml` (gloo-net dep)        |
| γ     | C1   | `Dockerfile`, `deploy/Caddyfile`, `.dockerignore` |
| δ     | D1 ✅ | `tests/`, `Cargo.toml` (dev-deps), `.cargo/config.toml` |
| ε     | F1   | `.github/workflows/book.yml`                   |
| ζ     | F2   | `book/src/adr/0001-stay-on-leptos.md`, `book/src/SUMMARY.md` |

`src/router.rs` and `src/app.rs` are touched only by α. Other agents must
not edit those files in their first PR; if they need a route, they ask α
to add it (cheap PR).

## Wave 2 (after wave-1 PRs land)

- **A2** unblocked by A1.
- **B2** unblocked by B1.
- **B3** unblocked by B1 (parallel to B2).
- **C2** unblocked by C1.
- **D2** unblocked by D1 ✅ (harness landed — `tests/smoke.rs` passes
  `cargo build --target wasm32-unknown-unknown --test smoke`).

## Wave 3

- **A3, A4** in serial after A2.
- **C3** after C2.
- **E1–E4** can all run in parallel once A2 + B2 are merged. Each touches
  exactly one screen file, so four agents can run concurrently.
- **D3** after C2.

## Test gate per task

Every task card in [agents.md](./agents.md) ends with a **Test gate**: the
exact command(s) that must pass before the PR is mergeable. The gate is
the contract — if it passes, the task is done; if it fails, the task is
not.

Cross-cutting CI on every PR (set up by F1):

```bash
cargo fmt --all -- --check
cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings
cargo check --target wasm32-unknown-unknown
trunk build --release
mdbook build
```

Tracks D1 and onwards add `wasm-pack test --headless --firefox` and (D3)
`npx playwright test` to the pipeline.
