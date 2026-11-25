use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapEvent {
    pub transaction_hash: String,
    pub pool_address: String,
    pub timestamp: u64,
    pub amount_usd: f64,
    pub sender: String,
    pub recipient: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_type: String,
    pub timestamp: u64,
    pub measured_value: f64,
    pub threshold: f64,
    pub transaction_hash: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringResult {
    pub swaps_checked: usize,
    pub alerts: Vec<Alert>,
    pub last_updated: u64,
}
