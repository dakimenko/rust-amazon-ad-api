/// Build a URL by substituting path parameters.
///
/// Accepts an optional set of query parameters as a slice of key-value pairs.
///
/// # Example
/// ```ignore
/// let url = build_url(
///     "https://advertising-api.amazon.com",
///     "/v2/sp/campaigns/{campaignId}",
///     &[("campaignId", "123456")],
///     &[("stateFilter", "enabled")],
/// );
/// ```
pub fn build_url(
    base_url: &str,
    path_template: &str,
    path_params: &[(&str, &str)],
    query_params: &[(&str, &str)],
) -> String {
    let path = crate::apis::helpers::fill_query_params(path_template, path_params);
    let base = base_url.trim_end_matches('/');

    let mut url = String::with_capacity(base.len() + path.len() + query_params.len() * 24 + 1);
    url.push_str(base);
    url.push_str(&path);

    if !query_params.is_empty() {
        let mut serializer = ::url::form_urlencoded::Serializer::new(url);
        for (k, v) in query_params {
            serializer.append_pair(k, v);
        }
        serializer.finish()
    } else {
        url
    }
}
