# Task 25: CSPRNG IV Generation Security (`getrandom`)

**Date:** 2026-07-23
**Status:** Completed
**Priority:** Medium
**Depends on:** None

---

## Objective

Integrate `getrandom` into `src/client/crypto.rs` to generate AES-CBC initialization vectors (IV) using system OS-level cryptographically secure random number generators (CSPRNG).

---

## Tasks & Checklist

- [ ] **Add `getrandom` Dependency**:
  - Location: `Cargo.toml`
  - Add `getrandom = { version = "0.2", optional = true }` under the `client` feature flag.

- [ ] **Update `encrypt_aes_cbc` Helper**:
  - Location: `src/client/crypto.rs`
  - Generate IV bytes using `getrandom::getrandom(&mut iv)`.

- [ ] **Add Unit Test for Secure IV Generation**:
  - Add unit test verifying random IV generation produces distinct ciphertexts across repeated calls.

- [ ] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
