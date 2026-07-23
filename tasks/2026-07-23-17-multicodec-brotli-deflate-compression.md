# Task 17: Multi-Codec Decompression (Brotli & Deflate)

**Date:** 2026-07-23
**Status:** Pending
**Priority:** Low
**Depends on:** tasks/2026-07-23-08-streaming-gzip-decompression.md

---

## Objective

Extend report downloading in `src/client/download.rs` to support `Brotli` (`br`) and `Deflate` HTTP response encodings via `async-compression`.

---

## Tasks & Checklist

- [ ] **Enable `brotli` and `deflate` Features**:
  - Location: `Cargo.toml`
  - Update `async-compression` dependency to include `features = ["tokio", "gzip", "brotli", "deflate"]`.

- [ ] **Extend `download` for Multi-Codec Decoding**:
  - Location: `src/client/download.rs`
  - Add auto-decoding for `Content-Encoding: br` and `Content-Encoding: deflate`.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
