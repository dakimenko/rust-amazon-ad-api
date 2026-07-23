# Task 27: High-Concurrency Batch Request Chunking (`futures-util`)

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Low
**Depends on:** tasks/2026-07-23-06-auto-pagination-streams.md

---

## Objective

Implement batch request chunking helpers (`sp_batch_create_keywords`) on `AmazonAdClient` using `futures_util::StreamExt::chunks` for rate-limiter-aware bulk operations.

---

## Tasks & Checklist

- [ ] **Add Batch Operations Helpers**:
  - Location: `src/client_apis/sp.rs`
  - Implement `sp_batch_create_keywords` accepting large vectors/streams of keywords and executing requests in chunks of 1000 items.

- [ ] **Add Unit Tests**:
  - Add unit test verifying stream chunking splits large payloads into rate-limited request batches.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
