# Task 22: Amazon Marketing Stream (AMS) v2 Dataset Schemas & Event Deserialization

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Medium
**Depends on:** None

---

## Objective

Update Amazon Marketing Stream models in `src/models/cross/stream.rs` with AMS v2 dataset types (`sp-performance`, `sb-performance`, `budget-usage`) and event payload deserialization schemas.

---

## Tasks & Checklist

- [ ] **Extend `StreamDataSet` Enums**:
  - Location: `src/models/cross/stream.rs`
  - Add `SpPerformance`, `SbPerformance`, `SdPerformance`, `SpTraffic`, `BudgetUsage` dataset variants matching Amazon API spec.

- [ ] **Add Serde Types for Real-Time AMS Events**:
  - Implement strongly-typed serde structs for real-time performance and traffic message payloads.

- [ ] **Add Unit Tests**:
  - Add deserialization unit tests for AMS v2 event JSON payloads.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
