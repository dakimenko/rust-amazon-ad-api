# Changelog

## 0.1.0 (unreleased)

Initial Rust port of `python-amazon-ad-api` v0.8.5 with enterprise-grade security, high-throughput concurrency, and stream-based pagination.

### Added
- **Sponsored Products v3**: Campaigns, Ad Groups, Keywords, Negative Keywords,
  Product Ads, Product Targeting, Negative Product Targeting,
  Campaign Negative Keywords, Campaign Negative Targets — full CRUD.
- **Sponsored Brands v4**: Campaigns, Ad Groups, Creatives, Ads, Keywords,
  Product/Negative Targeting, Media, Benchmarks, Themes, Reports, Snapshots.
- **Sponsored Display**: Campaigns, Ad Groups, Product Ads, Product/Negative
  Targeting, Creatives (moderation + preview), Brand Safety Deny Lists,
  Budget Rules, Reports, Snapshots.
- **DSP**: Reports with versioned Accept headers.
- **Cross-cutting**: Profiles, Portfolios v3, Unified Reports v3, Localization,
  Creative Assets, Marketing Stream, Attribution, Audiences, Billing,
  Brand Metrics, Stores.
- **High-Throughput Lock-Free Rate Limiter**: Driven by `moka::sync::Cache` and sub-second `std::time::Instant` precision token refills, eliminating Tokio async mutex scheduling overhead on in-memory operations.
- **Secret Credentials Protection**: `SecretString` wrapper for `client_secret` and `refresh_token` ensuring automatic zeroization and `[REDACTED]` Debug log masking.
- **Streaming Gzip Decompression**: Non-blocking streaming report decompression powered by `async-compression` and `tokio-util` directly from network streams.
- **Ergonomic Client API Extensions**: Single-item convenience methods (`sp_create_campaign`) accepting `impl Into<T>` and stream-based auto-pagination (`sp_stream_campaigns`).
- **HTTP Integration Test Suite**: Complete integration test suite using `wiremock` under `tests/integration/` for OAuth authentication, API payload verification, and 400 error payload parsing.
- **Panic Safety & Structured Error Handling**: Safe parsing of headers, zero `unwrap()` panics in deep object parsing, and auto-populated `ResponseContent.entity` on HTTP error responses.
- OAuth2 LWA authentication with profile selection and `Arc`-shared token cache.
- `reqwest-middleware` stack: retry (exponential backoff on 429/5xx),
  tracing middleware, custom auth header injection.
- Builder pattern (`derive_builder`) for all request types.
- Feature-gated compilation (`sp`, `sb`, `sd`, `dsp`, `cross`).
- Dynamic profile switching via `AmazonAdClient::with_profile()`.
- Marketplace enums with IDs, currencies, locales, and region mapping.
- AES-CBC encryption helpers for creative asset security.

### Architecture & Quality
- **CI Pipeline**: Configured GitHub Actions workflow (`.github/workflows/ci.yml`) to test and lint with `--all-features`.
- **Panic-Safety**: Zero panics across `execute_request`, `parse_deep_object`, and macro parameter parsing.
- **Clippy Clean**: 100% compliant with `cargo clippy --workspace --all-features -- -D warnings`.
- **Rust Best Practices**: Dedicated project skill under `.gemini/skills/rust-best-practices/SKILL.md`.

### Not Ported (deprecated in Python source)
- SP v2, SB v3 endpoints (deprecated by Amazon).
- Synchronous file-system side effects in download (replaced with streaming in-memory buffers).
