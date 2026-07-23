# Task 20: Millisecond Reset Headers Support (`x-ad-api-rate-limit-reset-ms`)

**Date:** 2026-07-23
**Status:** Completed
**Priority:** High
**Depends on:** None

---

## Objective

Update header parsing and `RateLimiter` to support Amazon's `x-ad-api-rate-limit-reset-ms` millisecond Unix timestamp headers for exact subsecond token bucket resets.

---

## Tasks & Checklist

- [ ] **Extract Millisecond Reset Headers**:
  - Location: `src/models/common.rs` (`RateLimitInfo::from_headers`)
  - Parse `x-ad-api-rate-limit-reset-ms` header into millisecond `Instant` / timestamp.

- [ ] **Update Rate Limiter Token Refill**:
  - Location: `src/client/rate_limiter.rs`
  - Refill token buckets precisely at reset timestamps when reset headers are present.

- [ ] **Add Unit Tests**:
  - Add unit test in `src/models/common.rs` verifying header extraction for `x-ad-api-rate-limit-reset-ms`.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
