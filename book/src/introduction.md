# Introduction

`class-forge-web` is the Leptos (Rust → WebAssembly) frontend for **ClassForge**,
a forge-agnostic classroom-management system. The backend is a separate Go
service at [`bloboss/class-forge`](https://github.com/bloboss/class-forge) that
exposes a REST API.

This book is the design + execution log for the next phase of work. It is
deliberately split so that several agents (or humans) can pick up independent
tasks in parallel without stepping on each other.

## Where things live

| Area                | Path                                    |
| ------------------- | --------------------------------------- |
| App entry           | `src/main.rs`, `src/lib.rs`             |
| Top-level chrome    | `src/app.rs`                            |
| Hash router         | `src/router.rs`                         |
| Mock data layer     | `src/data.rs`                           |
| Screen components   | `src/screens/{onboarding,dashboard,classroom,assignment_detail}.rs` |
| Shared UI primitives| `src/components.rs`, `src/icons.rs`     |
| Styling             | `styles.css`                            |
| Build config        | `Trunk.toml`, `index.html`, `Cargo.toml`|

## Why we are staying on Leptos

Reviewed and decided: stay on Leptos. The team values Rust's compile-time
guarantees over JS-ecosystem reach, and the codebase is small enough
(~2.3k LOC) that the supposed JS interactivity gap is mostly an *ecosystem*
gap, not a framework one. Where we need third-party widgets (charts, rich
editors, drag-and-drop), we'll wrap them through `wasm-bindgen` rather than
rewriting the app.

## What this book covers

1. **Architecture** — the auth model, how the WASM client talks to the Go
   backend, and the state layout we want before features pile up.
2. **Deployment** — Docker images for frontend + backend and a `docker
   compose` file that runs the whole stack locally for fast iteration.
3. **Roadmap** — a DAG of work items that can be parallelized, with one task
   card per agent including the test gate that closes the task out.
