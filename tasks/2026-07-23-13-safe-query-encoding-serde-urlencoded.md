# Task 13: Safe Query Parameter Encoding & RFC 3986 Compliance

**Date:** 2026-07-23
**Status:** Pending
**Priority:** Medium
**Depends on:** None

---

## Objective

Integrate `serde_urlencoded` into low-level API request builders to ensure 100% compliant RFC 3986 query parameter encoding and eliminate manual string concatenation bugs.

---

## Tasks & Checklist

- [ ] **Add `serde_urlencoded` Dependency**:
  - Location: `Cargo.toml`
  - Add `serde_urlencoded = { version = "0.7", optional = true }` under the `client` feature flag.

- [ ] **Refactor `build_url` in `src/apis/params.rs`**:
  - Replace manual parameter formatting with `serde_urlencoded::to_string`.

- [ ] **Add Unit Tests for Query Parameter Encoding**:
  - Test encoding for special characters (`&`, `=`, `?`, spaces), date arrays, and filters.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
