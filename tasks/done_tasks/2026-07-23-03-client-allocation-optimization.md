# Task 3: Optimize Client Configuration & Memory Allocations

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Medium
**Depends on:** tasks/2026-07-23-02-rate-limiter-precision.md

---

## Objective

Reduce memory allocations per API call by caching middleware client pipelines, optimizing header parsing, and refining Cargo feature tree isolation.

---

## Tasks & Checklist

- [x] **Cache `CustomClient` in `AmazonAdClient`**:
  - Location: `src/client/client.rs` (`AmazonAdClient`), `src/apis/configuration.rs`
  - Cached pre-configured `CustomClient` inside `AmazonAdClient` and reused `custom_client.inner` in `create_configuration()`.

- [x] **Optimize Response Header Extraction**:
  - Location: `src/apis/helpers.rs` (`execute_request`)
  - Selective filter for relevant API headers (`x-*`, `nexttoken`, `location`, `content-type`).

- [x] **Make `async-stream` and `futures-core` Optional in `Cargo.toml`**:
  - Location: `Cargo.toml`
  - Moved `async-stream` and `futures-core` to optional dependencies under the `client` feature flag.

- [x] **Verification**:
  - Run `cargo build --no-default-features --features=sp,sb,sd,dsp,cross` (models build succeeded)
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
