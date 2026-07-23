# Task 7: Model Trait Extensions & Header Lookup Normalization

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Low
**Depends on:** tasks/2026-07-23-01-panic-safety-error-handling.md

---

## Objective

Clean up redundant header checks in `ApiResponse::from_parts` and add `PartialEq` derivations on core model structs to improve testing and state comparisons.

---

## Tasks & Checklist

- [x] **Normalize Header Key Lookups**:
  - Location: `src/models/common.rs` (`ApiResponse::from_parts`)
  - Simplified header extraction by relying on lowercase key lookups.

- [x] **Derive `PartialEq` on Core Model Structs**:
  - Location: `src/models/sp/campaigns.rs`
  - Added `PartialEq` derive to `Campaign`, `DailyBudget`, `DynamicBidding`, `PlacementBidding`.

- [x] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
