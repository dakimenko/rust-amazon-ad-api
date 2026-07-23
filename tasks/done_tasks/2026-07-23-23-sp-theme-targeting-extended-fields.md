# Task 23: Sponsored Products v3 Theme Targeting & Extended Fields Schema

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Medium
**Depends on:** None

---

## Objective

Update Sponsored Products v3 campaign models to support Theme Targeting (`spThemeTargeting`) and the `includeExtendedDataFields` parameter.

---

## Tasks & Checklist

- [ ] **Add `includeExtendedDataFields` Parameter**:
  - Location: `src/models/sp/campaigns.rs` (`ListCampaignsRequest`)
  - Ensure `include_extended_data_fields: Option<bool>` serializes to `includeExtendedDataFields`.

- [ ] **Add Theme Targeting Schema**:
  - Location: `src/models/sp/targeting.rs`
  - Implement `SpThemeTargeting` struct and enum variants matching SP v3 API spec.

- [ ] **Add Unit Tests**:
  - Add unit tests verifying JSON payload serialization/deserialization for extended fields and theme targeting.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
