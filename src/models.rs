use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolEvent {
    pub event_id: String,
    pub protocol: String,
    pub timestamp: u64,
    pub event_type: EventType,
    pub amount_usd: f64,
    pub liquidity_before_usd: f64,
    pub liquidity_after_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Withdrawal,
    Deposit,
    Swap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapEvent {
    pub transaction_hash: String,
    pub pool_address: String,
    pub timestamp: u64,
    pub amount_usd: f64,
    pub sender: String,
    pub recipient: String,
}
