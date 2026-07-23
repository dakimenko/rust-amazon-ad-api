# Task 16: Performance Metrics & Telemetry Instrumentation

**Date:** 2026-07-23
**Status:** Pending
**Priority:** Low
**Depends on:** None

---

## Objective

Add optional `metrics` feature flag to instrument API calls, token refresh operations, and rate limiter wait durations for monitoring in Prometheus and Grafana.

---

## Tasks & Checklist

- [ ] **Add `metrics` Feature Flag**:
  - Location: `Cargo.toml`
  - Add `metrics = { version = "0.22", optional = true }` under `[dependencies]` and expose `metrics = ["dep:metrics"]` in `[features]`.

- [ ] **Instrument API Execution & Rate Limiter**:
  - Location: `src/apis/helpers.rs`, `src/client/rate_limiter.rs`
  - Record request counters (`amazon_ad_api_requests_total`) and wait histograms (`amazon_ad_api_rate_limit_wait_seconds`).

- [ ] **Verification**:
  - Run `cargo check --features metrics`
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
