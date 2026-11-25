use crate::detector::detect_activity_bursts;
use crate::detector::detect_activity_bursts_rolling;
use crate::detector::detect_large_swaps;
use crate::models::MonitoringResult;
use crate::validation::validate_swap;
use serde::Deserialize;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

const ENDPOINT: &str =
    "https://gateway.thegraph.com/api/subgraphs/id/5zvR82QoaXYFyDEKLZ9t6v9adgnptxYpKpSbxtgVENFV";

const POOL_ADDRESS: &str = "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640";

#[derive(Debug, Deserialize)]
struct GraphResponse {
    data: Option<GraphData>,
    errors: Option<Vec<GraphError>>,
}

#[derive(Debug, Deserialize)]
struct GraphError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct GraphData {
    swaps: Vec<GraphSwap>,
}

#[derive(Debug, Deserialize)]
struct GraphSwap {
    timestamp: String,

    #[serde(rename = "amountUSD", default)]
    amount_usd: String,

    sender: String,
    recipient: String,
    transaction: GraphTransaction,
}

#[derive(Debug, Deserialize)]
struct GraphTransaction {
    id: String,
}

pub async fn fetch_recent_swaps() -> Result<MonitoringResult, Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("THEGRAPH_API_KEY")?;
    let query = format!(
        r#"{{
        swaps(
            first: 500
            orderBy: timestamp
            orderDirection: desc
            where: {{ pool: "{POOL_ADDRESS}" }}
        ) {{
            timestamp
            amountUSD
            sender
            recipient
            transaction {{ id }}
        }}
    }}"#
    );

    let client = reqwest::Client::new();

    let response = client
        .post(ENDPOINT)
        .bearer_auth(&api_key)
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?;

    let body = response.error_for_status()?.text().await?;

    let result: GraphResponse = serde_json::from_str(&body)?;

    if let Some(errors) = result.errors {
        for error in errors {
            eprintln!("GraphQL error: {}", error.message);
        }
        return Err("The Graph returned a query error".into());
    }

    let data = result.data.ok_or("The response contained no data")?;

    println!("Recent Uniswap v3 WETH/USDC swaps:");

    let mut swap_events = Vec::new();

    for swap in data.swaps {
        let event = crate::models::SwapEvent {
            transaction_hash: swap.transaction.id,
            pool_address: POOL_ADDRESS.to_string(),
            timestamp: swap.timestamp.parse()?,
            amount_usd: swap.amount_usd.parse()?,
            sender: swap.sender,
            recipient: swap.recipient,
        };

        match validate_swap(&event) {
            Ok(()) => {
                swap_events.push(event);
            }
            Err(error) => {
                eprintln!("Skipping invalid swap: {error}");
            }
        }
    }

    let large_swap_alerts = detect_large_swaps(&swap_events);

    let activity_alerts = detect_activity_bursts(&swap_events);

    println!("\n========== BASELINE: 5 WINDOWS ==========");
    let rolling_5_alerts = detect_activity_bursts_rolling(&swap_events, 5);

    println!("\n========== BASELINE: 10 WINDOWS ==========");
    let rolling_10_alerts = detect_activity_bursts_rolling(&swap_events, 10);

    println!("\n========== BASELINE: 20 WINDOWS ==========");
    let rolling_20_alerts = detect_activity_bursts_rolling(&swap_events, 20);

    let mut all_alerts = Vec::new();

    all_alerts.extend(large_swap_alerts);
    all_alerts.extend(activity_alerts);
    all_alerts.extend(rolling_5_alerts);
    all_alerts.extend(rolling_10_alerts);
    all_alerts.extend(rolling_20_alerts);

    println!("\nTotal alerts generated: {}", all_alerts.len());

    println!("Successfully validated {} swap events.", swap_events.len());

    // Save validated swaps for reproducible experiments.
    std::fs::create_dir_all("data")?;

    let snapshot = serde_json::to_string_pretty(&swap_events)?;

    std::fs::write("data/swaps_snapshot.json", snapshot)?;

    println!("Dataset snapshot saved to data/swaps_snapshot.json");

    let last_updated = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    Ok(MonitoringResult {
        swaps_checked: swap_events.len(),
        alerts: all_alerts,
        last_updated,
    })
}
