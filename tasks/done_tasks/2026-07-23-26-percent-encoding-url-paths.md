# Task 26: Safe Dynamic URL Path Segment Encoding (`percent-encoding`)

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Medium
**Depends on:** None

---

## Objective

Integrate `percent-encoding` into low-level API path formatting to ensure dynamic URL path segments (IDs, profile IDs, portfolio IDs) are RFC 3986 percent-encoded.

---

## Tasks & Checklist

- [ ] **Add `percent-encoding` Dependency**:
  - Location: `Cargo.toml`
  - Add `percent-encoding = { version = "2.3", optional = true }` under the `client` feature flag.

- [ ] **Add Path Segment Encoding Helper**:
  - Location: `src/apis/params.rs`
  - Implement `encode_path_segment` using `utf8_percent_encode`.

- [ ] **Apply Encoding Helper Across API Paths**:
  - Apply `encode_path_segment` to dynamic path parameters in `src/apis/`.

- [ ] **Add Unit Tests**:
  - Add unit test verifying percent-encoding of path segments containing spaces, slashes, and special characters.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
