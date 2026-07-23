# Task 18: Automatic MIME Type Inference for Media Assets

**Date:** 2026-07-23
**Status:** Pending
**Priority:** Low
**Depends on:** None

---

## Objective

Integrate `mime_guess` into creative asset uploading utilities to infer MIME types (`image/png`, `video/mp4`) automatically from file extensions.

---

## Tasks & Checklist

- [ ] **Add `mime_guess` Dependency**:
  - Location: `Cargo.toml`
  - Add `mime_guess = { version = "2.0", optional = true }` under the `client` feature flag.

- [ ] **Add Media Upload Helper with Auto MIME Type**:
  - Location: `src/client_apis/sb.rs`, `src/client_apis/sd.rs`
  - Implement convenience helpers setting `Content-Type` headers based on guessed MIME types.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
