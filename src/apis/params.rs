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

    let url = format!("{}{}", base_url.trim_end_matches('/'), path);

    if query_params.is_empty() {
        url
    } else {
        let qs: Vec<String> = query_params
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}={}",
                    crate::apis::urlencode(k),
                    crate::apis::urlencode(v)
                )
            })
            .collect();
        format!("{}?{}", url, qs.join("&"))
    }
}
