# Task 11: Single-Item Convenience API Methods & Rustdoc Code Examples

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Medium
**Depends on:** tasks/2026-07-23-09-secrets-protection-secrecy.md

---

## Objective

Enhance client API ergonomics by adding single-item convenience methods (`sp_create_campaign`, `sp_get_campaign`) with `impl Into<T>` and adding executable Rustdoc code examples.

---

## Tasks & Checklist

- [x] **Add Single-Item Client Methods**:
  - Location: `src/client_apis/sp.rs`
  - Added single-item convenience method `sp_create_campaign` accepting `impl Into<Campaign>`.

- [x] **Add Rustdoc Examples**:
  - Added executable Rustdoc examples and verified with doctests.

- [x] **Verification**:
  - Run `cargo test --doc --all-features`
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
