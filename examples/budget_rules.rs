use amazon_ad_api::client::{AmazonAdClient, AmazonAdConfig};
use amazon_ad_api::models::sp::budget_rules::{
    BudgetRule, BudgetRuleDetails, BudgetRuleType, BudgetRuleStatus,
    RuleRecurrence, BudgetIncreaseBy
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 1. Initialize configuration and client
    let config = AmazonAdConfig::from_env()?;
    let client = AmazonAdClient::new(config)?;

    // 2. Create the lower-level API configuration
    let api_config = client.create_configuration().await?;

    // 3. Define a budget rule
    let rule = BudgetRule {
        rule_id: None,
        name: Some("Holiday Weekend Boost".to_string()),
        status: Some(BudgetRuleStatus::Active),
        rule_details: Some(BudgetRuleDetails {
            rule_type: Some(BudgetRuleType::Schedule),
            recurrence: Some(RuleRecurrence {
                recurrence_type: Some("WEEKLY".to_string()),
                days_of_week: Some(vec!["SATURDAY".to_string(), "SUNDAY".to_string()]),
                intra_day_schedule: None,
            }),
            budget_increase_by: Some(BudgetIncreaseBy::Percent(50.0)),
            duration: None,
            performance_measure_condition: None,
        }),
    };

    println!("Creating budget rule...");

    // 4. Call the budget rules API
    let response = amazon_ad_api::apis::sp::budget_rules::create_budget_rule(
        &api_config,
        vec![rule]
    ).await;

    match response {
        Ok(res) => {
            println!("Successfully created budget rules: {:?}", res.payload);
        }
        Err(e) => {
            println!("Failed to create budget rules. (This is expected if the payload or authentication is invalid without a sandbox)");
            println!("Error: {}", e);
        }
    }

    Ok(())
}
