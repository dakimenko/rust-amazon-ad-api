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
- **Lock-Free OAuth Token Cache (`arc-swap`)**: `ArcSwapOption` replaces `tokio::sync::RwLock` in `AuthClient` for zero-cost concurrent token and profile reads. `get_profile_id` and `set_profile` are now fully synchronous.
- **Millisecond-Precision Rate Limit Headers**: `RateLimitInfo` now parses `x-ad-api-rate-limit-reset-ms` for sub-second reset accuracy.
- **Async Report Polling Helper (`cross_download_report_polled`)**: Automates the full Unified Reports v3 lifecycle with exponential backoff polling, COMPLETED detection, and streaming gzip-decoded S3 download in a single call.
- **Safe URL Path Encoding (`percent-encoding`)**: `encode_path_segment` helper in `src/apis/mod.rs` ensures RFC 3986-compliant percent-encoding of dynamic API path IDs.
- **Multi-Codec Decompression (Brotli & Deflate)**: `download.rs` now auto-detects and decompresses `Content-Encoding: br` and `Content-Encoding: deflate` alongside existing gzip support.
- **CSPRNG IV Generation (`getrandom`)**: `aes_encrypt_with_random_iv` in `src/client/crypto.rs` generates AES-CBC initialization vectors from OS-level entropy, preventing deterministic ciphertext reuse.
- **Cancellation-Safe Rate Limiter**: `RateLimiter::wait` is documented and tested as cancellation-safe — no tokens consumed if the future is cancelled (e.g., via `tokio::time::timeout`).
- **Type-Safe Query String Builder**: `build_query_string` in `src/apis/helpers.rs` serializes any `serde::Serialize` struct to a properly percent-encoded query string, skipping null fields automatically.
- **Batch Keyword Creation (`sp_batch_create_keywords`)**: Automatically splits large keyword lists into API-compliant 1000-item chunks, executing sequentially and collecting all responses in order.
- **AMS v2 Stream Datasets & Event Deserialization**: Extended `src/models/cross/stream.rs` with `StreamDataSet` enums (`SpPerformance`, `SbPerformance`, `SdPerformance`, `SpTraffic`, `BudgetUsage`) and strongly-typed serde structs for real-time stream messages.
- **SP v3 Theme Targeting & Extended Fields**: Added `SpThemeTargetingClause`, `SpThemeType`, and `SpThemeTargetingExpression` in `src/models/sp/targeting.rs`.
- **Automatic MIME Type Inference (`mime_guess`)**: `guess_mime_type` in `src/apis/helpers.rs` infers MIME types for creative media uploads from file extensions.
- **SIMD-Accelerated JSON Deserialization (`simd-json`)**: Optional `simd-json` feature flag in `Cargo.toml` enabling SIMD-accelerated report deserialization.
- **Compact String Memory Optimization (`compact-str`)**: Optional `compact-str` feature flag for inline stack storage of short string identifiers.
- **Strongly-Typed Date Formatting**: `format_date_yyyymmdd`, `format_date_iso`, and `parse_date` helpers in `src/models/common.rs` using `time::Date`.
- **OpenTelemetry Tracing Instrumentation**: `execute_request` instrumented with `#[tracing::instrument]` recording `http.method`, `http.url`, and `http.status_code` span attributes.
- **Algorithmic Performance Optimizations**:
  - `derive_endpoint_key`: Replaced `.split('/').collect::<Vec>()` with zero-allocation stream iterator combinators per API request.
  - `build_url`: Streamed query parameters directly into capacity-preallocated target buffer using `url::form_urlencoded::Serializer`, eliminating intermediate `Vec<String>` allocations.
  - `parse_deep_object`: Refactored to single-vector recursive accumulator (`parse_deep_object_into`), eliminating recursive `Vec` creations and `.append()` vector copies.
  - `nest_dict`: Switched to `.split('.').peekable()` iterator traversal, avoiding heap vector allocation on every map entry insertion.
  - `RateLimiter::_wait_for_token`: Hoisted token bucket lookup out of the sleep loop and pre-allocated status `HashMap` capacity.
  - `aes_encrypt_with_random_iv`: Replaced double-vector allocation with single `vec![0u8; 16 + pad_len]` buffer and in-place `split_at_mut` encryption.
  - `download`: Consolidated `content-encoding` header lookups into a single pass.
  - `paginate_cursor` / `paginate_all_cursor`: Eliminated `Option<String>` cursor cloning on every page iteration via `cursor.take()`.
  - `AmazonAdConfig::token_url`: Added custom `token_url` override check and static `Region::token_url` zero-alloc return.
  - `create_configuration`: Pre-allocated `HeaderMap::with_capacity(5)` and converted string parsing to zero-parsing `HeaderValue::from_static` / `from_str`.
- OAuth2 LWA authentication with profile selection and `Arc`-shared token cache.
- `reqwest-middleware` stack: retry (exponential backoff on 429/5xx),
  tracing middleware, custom auth header injection.
- Builder pattern (`derive_builder`) for all request types.
- Feature-gated compilation (`sp`, `sb`, `sd`, `dsp`, `cross`).
- Dynamic profile switching via `AmazonAdClient::with_profile()`.
- Marketplace enums with IDs, currencies, locales, and region mapping.
- AES-CBC encryption helpers for creative asset security.

### Architecture & Quality
- **Local Verification Suite**: Quality assurance and test verification (`cargo test`, `cargo clippy`, `cargo fmt`) performed directly in local development environment.
- **Panic-Safety**: Zero panics across `execute_request`, `parse_deep_object`, and macro parameter parsing.
- **Clippy Clean**: 100% compliant with `cargo clippy --workspace --all-features -- -D warnings`.
- **Rust Best Practices**: Dedicated project skill under `.gemini/skills/rust-best-practices/SKILL.md`.

### Not Ported (deprecated in Python source)
- SP v2, SB v3 endpoints (deprecated by Amazon).
- Synchronous file-system side effects in download (replaced with streaming in-memory buffers).
