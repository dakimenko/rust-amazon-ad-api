# Task 5: Non-blocking Async I/O, Decompression & Log Security Hardening

**Date:** 2026-07-23
**Status:** Completed
**Priority:** High
**Depends on:** tasks/2026-07-23-01-panic-safety-error-handling.md

---

## Objective

Fix potential clock-drift panics in OAuth token caching, protect sensitive credentials in debug logs, and prevent CPU/IO thread-starvation in async report downloads.

---

## Tasks & Checklist

- [x] **Fix Clock Drift Panic Vector**:
  - Location: `src/client/auth.rs` (`get_current_timestamp`)
  - Replaced `.unwrap()` on `SystemTime::now().duration_since(UNIX_EPOCH)` with `.unwrap_or_default()`.

- [x] **Redact Access Token in Logs**:
  - Location: `src/client/auth.rs` (`refresh_access_token`)
  - Redacted `access_token` values in `tracing::debug!` logs.

- [x] **Offload CPU-Bound Gzip Decompression**:
  - Location: `src/client/download.rs` (`download`)
  - Offloaded synchronous `flate2::read::GzDecoder::read_to_end` decompression to `tokio::task::spawn_blocking`.

- [x] **Replace Blocking Disk I/O**:
  - Location: `src/client/download.rs` (`SaveToFile::save_to_file`)
  - Offloaded file writing to `tokio::task::spawn_blocking`.

- [x] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
