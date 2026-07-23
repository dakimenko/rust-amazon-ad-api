# Task 15: Strongly-Typed Date Models & ISO-8601 Verification

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Low
**Depends on:** None

---

## Objective

Introduce strongly-typed date helpers (`time::Date`) into report requests and budget rules models to ensure ISO-8601 formatting correctness at compile-time.

---

## Tasks & Checklist

- [ ] **Add Date Conversion Helpers**:
  - Location: `src/models/common.rs`
  - Add helper functions to format `time::Date` into standard Amazon Ads date formats (`YYYYMMDD` and `YYYY-MM-DD`).

- [ ] **Integrate Date Helpers into Report Request Builders**:
  - Location: `src/models/sp/reports.rs`, `src/models/sb/reports.rs`, `src/models/sd/reports.rs`
  - Add builder setters accepting `time::Date`.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
