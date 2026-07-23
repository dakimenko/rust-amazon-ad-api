# AGENTS.md — AI Agent Guidelines & Project Manual

This repository contains `amazon-ad-api`, an unofficial Rust client crate for the [Amazon Advertising API](https://advertising.amazon.com/API/docs/en-us), ported from [`python-amazon-ad-api`](https://github.com/denisneuf/python-amazon-ad-api) (v0.8.5).

This document defines the rules, architecture, workflow, and standards for AI agents (AGY/Antigravity, Cursor, Cline, Claude Code, Windsurf, Copilot, etc.) operating in this repository.

---

## 1. Core Architecture & Layering

The codebase follows a clean 3-layer architecture:

```
                          ┌──────────────────────────┐
                          │     AmazonAdClient       │
                          │   (src/client_apis/)     │
                          └────────────┬─────────────┘
                                       │
                                       ▼
                          ┌──────────────────────────┐
                          │     Low-level APIs       │
                          │      (src/apis/)         │
                          └────────────┬─────────────┘
                                       │
                                       ▼
                          ┌──────────────────────────┐
                          │        Models            │
                          │     (src/models/)        │
                          └──────────────────────────┘
```

1. **Models Layer (`src/models/`)**
   - Strongly-typed Rust data structures built with `serde` and `derive_builder`.
   - Subdivided by API domain: `sp/` (Sponsored Products v3), `sb/` (Sponsored Brands v4), `sd/` (Sponsored Display), `dsp/` (DSP reports), `cross/` (Profiles, Portfolios, Reports v3, Attribution, Stream), `common.rs`, `marketplaces.rs`.
   - Must use exact field serde annotations (`camelCase`, `snake_case`, or explicit `#[serde(rename = "...")]`) matching Amazon API specs.

2. **Low-level API Layer (`src/apis/`)**
   - Raw HTTP request functions that consume `&Configuration`.
   - Handles specific Amazon API Accept/Content-Type version headers (e.g., `application/vnd.spCampaign.v3+json`).
   - Grouped into module folders: `sp/`, `sb/`, `sd/`, `dsp/`, `cross/`.

3. **Client API Layer (`src/client_apis/`)**
   - High-level convenience methods exposed directly on `AmazonAdClient`.
   - Delegates to low-level functions in `src/apis/` using the client's internal `Configuration`.

4. **Client Infrastructure (`src/client/`)**
   - `auth.rs`: OAuth2 Login with Amazon (LWA) token management with an `Arc`-shared token cache.
   - `rate_limiter.rs`: High-throughput token-bucket rate limiter driven by `moka::sync::Cache` parsing `x-ad-api-rate-limit-*` response headers.
   - `config.rs`: Credential loading from Environment variables (`AD_API_*`) and TOML files (`credentials.toml`) with `SecretString` redaction.
   - `download.rs`: Non-blocking streaming report download & automatic gzip decompression via `async-compression`.
   - `crypto.rs`: Encryption/decryption helpers.

---

## 2. Feature Flags Matrix

The crate supports granular feature flags for minimal dependency trees:

| Feature  | Includes | Default Enabled |
|----------|----------|-----------------|
| `client` | HTTP client, OAuth2 auth, rate limiter, reqwest middleware | **Yes** |
| `sp`     | Sponsored Products v3 models & APIs | **Yes** |
| `sb`     | Sponsored Brands v4 models & APIs | **Yes** |
| `sd`     | Sponsored Display models & APIs | **Yes** |
| `dsp`    | DSP report models & APIs | No |
| `cross`  | Profiles, portfolios, reports v3, attribution, stream | No |

**Rule for Agents:** When introducing new modules or dependencies, ensure conditional compilation attributes (`#[cfg(feature = "...")]`) are strictly maintained across `src/lib.rs`, `src/models/mod.rs`, `src/apis/mod.rs`, and `src/client_apis/mod.rs`.

---

## 3. Mandatory Build, Test & Quality Commands

Before declaring any task completed, agents **MUST** execute the following verification suite:

```bash
# 1. Workspace Build
cargo build --workspace

# 2. Build with All Features
cargo build --all-features

# 3. Build Models-only (No Client / Default Features)
cargo build --no-default-features --features=sp,sb,sd,dsp,cross

# 4. Run All Unit & Integration Tests
cargo test --workspace --all-features

# 5. Linting (Must compile clean without warnings)
cargo clippy --workspace --all-features -- -D warnings

# 6. Formatting Check
cargo fmt --check

# 7. Check Examples
cargo check --examples
```

---

## 4. Task Management Workflow (`tasks/`)

All agent and human tasks follow a strict lifecycle managed under `tasks/`:

```
tasks/
├── README.md                      # Task management documentation
├── TEMPLATE.md                    # Task template for new tasks
├── YYYY-MM-DD-task-name.md        # Active / Pending tasks
└── done_tasks/
    └── YYYY-MM-DD-completed.md    # Completed task archive
```

### Task Execution Rules for Agents:
1. **Active Tasks**: Create new task files in `tasks/YYYY-MM-DD-<description>.md`.
2. **Metadata Header**: Every task file MUST begin with metadata:
   ```markdown
   # Task Title

   **Date:** YYYY-MM-DD
   **Status:** Pending | In Progress | Completed
   **Priority:** High | Medium | Low
   **Depends on:** None | <other-task-file>
   ```
3. **Completion Criteria**: Once all subtasks are finished and full `cargo test` + `cargo clippy` pass, update **Status** to `Completed` and move the file to `tasks/done_tasks/`.

---

## 5. Development & Code Style Guidelines

1. **Rust Edition & Version**: Target Rust 2021 edition (`rust-version = "1.75"`).
2. **Error Handling**: Use `thiserror` (v2) for library errors and `anyhow` (v1) for binaries/examples/client tools. Never suppress errors with silent `unwrap()` or empty fallbacks in production code.
3. **Builder Derivation**: All model structs that are used as request payloads should derive `derive_builder::Builder` with `#[builder(setter(into, strip_option), default)]` where appropriate.
4. **Documentation**:
   - Every public module, struct, enum, and function MUST have standard Rustdoc comments (`///`).
   - Include code examples (`/// ```rust`) for public client APIs where feasible.
5. **No Breaking Changes to API Contracts**:
   - If modifying `apis/`, update `client_apis/` accordingly.
   - Respect upstream `python-amazon-ad-api` endpoint definitions when porting missing APIs.

---

## 6. Safety & Debugging Protocols

- **Traceback-First Diagnosis**: Never guess the root cause of a failing test. Always read the exact `cargo test` or `cargo clippy` output log before modifying code.
- **No Superficial Symptom Patches**: Fix the actual root cause instead of commenting out assertions or masking deserialization errors.
- **Environment Isolation**: Local tests must use `wiremock` or mock fixtures (see `dev-dependencies`). Never make live HTTP requests to Amazon Advertising endpoints during automated unit tests.

---

## 7. Mandatory Git & Documentation Workflow Rule

- **Pre-Git Documentation Requirement**: **ALWAYS** update project documentation (`CHANGELOG.md`, `README.md`, `AGENTS.md`, and task status files in `tasks/`) BEFORE executing Git repository operations (`git commit`, `git push`).
- **Atomic Commits**: Git commits and pushes must NEVER omit documentation updates corresponding to the code or architectural changes introduced.
