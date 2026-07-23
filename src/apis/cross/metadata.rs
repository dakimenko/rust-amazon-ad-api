pub async fn get_metadata(
    _configuration: &crate::apis::configuration::Configuration,
) -> Result<
    crate::models::common::ApiResponse<serde_json::Value>,
    crate::apis::Error<serde_json::Value>,
> {
    unimplemented!("get_metadata stub")
}
