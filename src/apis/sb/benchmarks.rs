use crate::apis::configuration::Configuration;
use crate::apis::Error;
use crate::models::common::ApiResponse;
use crate::models::sb::benchmarks::*;

pub async fn get_benchmarks(
    configuration: &Configuration,
    keyword_or_target: &str,
    body: SbBenchmarkRequest,
) -> Result<ApiResponse<Vec<SbBenchmarkResult>>, Error<serde_json::Value>> {
    let req = configuration
        .client
        .request(
            reqwest::Method::POST,
            format!(
                "{}/v2/hsa/{}/benchmarks",
                configuration.base_path, keyword_or_target
            ),
        )
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&body)
        .build()?;
    crate::apis::helpers::execute_request(configuration, req).await
}
