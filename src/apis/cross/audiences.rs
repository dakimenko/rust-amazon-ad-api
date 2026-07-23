pub async fn get_taxonomy(
    _configuration: &crate::apis::configuration::Configuration,
) -> Result<
    crate::models::common::ApiResponse<serde_json::Value>,
    crate::apis::Error<serde_json::Value>,
> {
    unimplemented!("get_taxonomy stub")
}

pub async fn query_audiences(
    _configuration: &crate::apis::configuration::Configuration,
) -> Result<
    crate::models::common::ApiResponse<serde_json::Value>,
    crate::apis::Error<serde_json::Value>,
> {
    unimplemented!("query_audiences stub")
}
