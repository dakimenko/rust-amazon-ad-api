# Task 21: Automatic Async Report Polling & Streaming Download Helper

**Date:** 2026-07-23
**Status:** Completed
**Priority:** High
**Depends on:** tasks/2026-07-23-08-streaming-gzip-decompression.md

---

## Objective

Add `sp_download_report_polled` convenience method on `AmazonAdClient` to automate the full async report lifecycle (creation, status polling, and stream downloading).

---

## Tasks & Checklist

- [ ] **Add Async Polling Helper**:
  - Location: `src/client_apis/cross.rs`, `src/client_apis/sp.rs`
  - Implement `sp_download_report_polled` with exponential backoff polling on `GET /v3/reports/{reportId}` until status is `COMPLETED`.

- [ ] **Stream Presigned S3 URL Payload**:
  - Download presigned S3 URL directly using `download()` with automatic gzip decompression.

- [ ] **Add Wiremock Integration Test**:
  - Location: `tests/integration/client_tests.rs`
  - Add integration test simulating report creation, status polling (`IN_PROGRESS` -> `COMPLETED`), and download.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
