# Task 9: Secure Credentials in Memory & Zeroize Secrets

**Date:** 2026-07-23
**Status:** Completed
**Priority:** High
**Depends on:** None

---

## Objective

Protect OAuth credentials (`client_secret`, `refresh_token`, `access_token`) in memory using `secrecy` to prevent accidental logging or memory dump exposure.

---

## Tasks & Checklist

- [x] **Add `secrecy` Dependency**:
  - Location: `Cargo.toml`
  - Added `secrecy` under the `client` feature flag.

- [x] **Wrap Credentials in `SecretString`**:
  - Location: `src/client/config.rs` (`AmazonAdConfig`), `src/client/auth.rs` (`AuthClient`)
  - Wrapped `client_secret` and `refresh_token` in `SecretString`.

- [x] **Redact `Debug` Output**:
  - Implemented custom `Debug` on `AmazonAdConfig` to automatically mask secrets (`[REDACTED]`).

- [x] **Verification**:
  - Run `cargo test --workspace --all-features`
  - Run `cargo clippy --workspace --all-features -- -D warnings`
  - Move task file to `tasks/done_tasks/` upon completion.
