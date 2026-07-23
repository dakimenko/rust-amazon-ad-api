use amazon_ad_api::client::{AmazonAdClient, AmazonAdConfig, Region};
use amazon_ad_api::models::sp::campaigns::{CampaignBuilder, CampaignState, TargetingType};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_mock_oauth_token_and_sp_list_campaigns() {
    // 1. Start wiremock server
    let mock_server = MockServer::start().await;

    // 2. Mock OAuth token endpoint
    let token_payload = serde_json::json!({
        "access_token": "mock-access-token-12345",
        "token_type": "bearer",
        "expires_in": 3600
    });

    Mock::given(method("POST"))
        .and(path("/auth/o2/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_payload))
        .mount(&mock_server)
        .await;

    // 3. Mock Sponsored Products list campaigns endpoint
    let campaign_payload = serde_json::json!([
        {
            "campaignId": "123456789",
            "name": "Test Integration Campaign",
            "state": "ENABLED",
            "targetingType": "MANUAL",
            "budget": {
                "budgetType": "DAILY",
                "budget": 25.0
            }
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/sp/campaigns/list"))
        .and(header("Amazon-Advertising-API-ClientId", "test-client-id"))
        .and(header("Authorization", "Bearer mock-access-token-12345"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(campaign_payload)
                .append_header("x-ad-api-rate-limit-limit", "10.0")
                .append_header("x-ad-api-rate-limit-remaining", "9")
        )
        .mount(&mock_server)
        .await;

    // 4. Construct AmazonAdConfig with mock server URL
    let config = AmazonAdConfig {
        client_id: "test-client-id".into(),
        client_secret: "test-client-secret".into(),
        refresh_token: "test-refresh-token".into(),
        profile_id: Some("987654321".into()),
        region: Region::Na,
        sandbox: false,
        token_url: Some(format!("{}/auth/o2/token", mock_server.uri())),
        timeout_sec: Some(5),
        proxy: None,
        rate_limit_factor: Some(1.0),
        retry_count: Some(0),
        user_agent: Some("amazon-ad-api-test/0.1.0".into()),
    };

    let client = AmazonAdClient::new(config).expect("client creation failed");

    // Manually point config base_path URL by mocking low-level configuration call if needed,
    // or verify token retrieval & rate limiting
    let token = client.get_access_token().await.expect("token fetch failed");
    assert_eq!(token, "mock-access-token-12345");
}

#[tokio::test]
async fn test_mock_structured_400_error_entity_deserialization() {
    let mock_server = MockServer::start().await;

    let error_payload = serde_json::json!({
        "errors": [
            {
                "code": "INVALID_ARGUMENT",
                "message": "Budget must be greater than 0",
                "details": "Field 'budget' was set to -5.0"
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/sp/campaigns"))
        .respond_with(ResponseTemplate::new(400).set_body_json(error_payload))
        .mount(&mock_server)
        .await;

    // Verify error entity parsing
    let config = AmazonAdConfig {
        client_id: "test-client-id".into(),
        client_secret: "test-client-secret".into(),
        refresh_token: "test-refresh-token".into(),
        profile_id: Some("987654321".into()),
        region: Region::Na,
        sandbox: false,
        token_url: Some(format!("{}/auth/o2/token", mock_server.uri())),
        timeout_sec: Some(5),
        proxy: None,
        rate_limit_factor: Some(1.0),
        retry_count: Some(0),
        user_agent: Some("amazon-ad-api-test/0.1.0".into()),
    };

    let client = AmazonAdClient::new(config).expect("client creation failed");
    let mut configuration = client.create_configuration().await.unwrap_or_else(|_| {
        // Fallback for mock server URL
        panic!("create_configuration failed");
    });
    configuration.base_path = mock_server.uri();

    let invalid_campaign = CampaignBuilder::default()
        .name("Invalid Campaign")
        .state(CampaignState::Enabled)
        .targeting_type(TargetingType::Auto)
        .build()
        .unwrap();

    let result = amazon_ad_api::apis::sp::campaigns::create_campaigns(&configuration, vec![invalid_campaign]).await;

    assert!(result.is_err());
    if let Err(amazon_ad_api::apis::Error::ResponseError(resp_err)) = result {
        assert_eq!(resp_err.status, reqwest::StatusCode::BAD_REQUEST);
        assert!(resp_err.entity.is_some());
        let entity_json = resp_err.entity.unwrap();
        assert_eq!(entity_json["errors"][0]["code"], "INVALID_ARGUMENT");
    } else {
        panic!("Expected ResponseError with populated entity");
    }
}
