# Task 12: Lock-Free OAuth Token Storage & Zero-Cost Concurrency

**Date:** 2026-07-23
**Status:** Completed
**Priority:** High
**Depends on:** None

---

## Objective

Integrate `arc-swap` into `AuthClient` to replace `tokio::sync::RwLock` for token and profile storage, enabling zero-cost, lock-free concurrent reads across high-frequency API requests.

---

## Tasks & Checklist

- [x] **Add `arc-swap` Dependency**:
  - Location: `Cargo.toml`
  - Added `arc-swap = { version = "1.7" }` under the `client` feature flag.

- [x] **Refactor `AuthClient` Token & Profile Storage**:
  - Location: `src/client/auth.rs`
  - Replaced `Arc<RwLock<Option<CachedToken>>>` with `Arc<ArcSwapOption<CachedToken>>`.
  - Replaced `Arc<RwLock<Option<Profile>>>` with `Arc<ArcSwapOption<Profile>>`.

- [x] **Update `get_access_token`, `get_profile_id`, `set_profile`**:
  - Implemented lock-free read via `.load_full()` and atomic update via `.store()`.
  - Made `get_profile_id` and `set_profile` synchronous (no `.await` needed).

- [x] **Verification**:
  - `cargo test --workspace --all-features`: PASS
  - `cargo clippy --workspace --all-features -- -D warnings`: PASS
  - Moved task file to `tasks/done_tasks/`.
