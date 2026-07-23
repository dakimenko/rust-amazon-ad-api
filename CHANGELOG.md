# Changelog

## 0.1.0 (unreleased)

Initial Rust port of `python-amazon-ad-api` v0.8.4.

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
- Adaptive token-bucket rate limiter with safety factor and dynamic
  limit adjustment from `x-ad-api-rate-limit-*` response headers.
- OAuth2 LWA authentication with profile selection and `Arc`-shared token cache.
- `reqwest-middleware` stack: retry (exponential backoff on 429/5xx),
  tracing middleware, custom auth header injection.
- Side-effect-free in-memory report download with automatic gzip decompression.
- Async pagination via `async-stream` generators.
- Builder pattern (`derive_builder`) for all request types.
- Feature-gated compilation (`sp`, `sb`, `sd`, `dsp`, `cross`).
- Dynamic profile switching via `AmazonAdClient::with_profile()`.
- Marketplace enums with IDs, currencies, locales, and region mapping.
- AES-CBC encryption helpers for creative asset security.
- 15 unit tests (crypto, config, model deserialization).

### Infrastructure
- Rate limiter adapted from `amazon-spapi` v2.0.3.
- Error types with structured `ApiErrorDetail` parsing.
- Multi-source config loading (env vars, TOML file, inline code).
- Full CI pipeline: build, test, clippy, fmt, security audit.

### Not Ported (deprecated in Python source)
- SP v2, SB v3 endpoints (deprecated by Amazon).
- File-system side effects in download (replaced with in-memory buffers).
