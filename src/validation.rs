use crate::errors::TraceGuardError;
use crate::models::SwapEvent;
use crate::models::{EventType, ProtocolEvent};

pub fn validate_event(event: &ProtocolEvent) -> Result<(), TraceGuardError> {
    if event.event_id.trim().is_empty() {
        return Err(TraceGuardError::InvalidEvent(
            "event_id cannot be empty".to_string(),
        ));
    }

    if event.protocol.trim().is_empty() {
        return Err(TraceGuardError::InvalidEvent(
            "protocol cannot be empty".to_string(),
        ));
    }

    if !event.amount_usd.is_finite() || event.amount_usd < 0.0 {
        return Err(TraceGuardError::InvalidEvent(
            "amount_usd must be finite and non-negative".to_string(),
        ));
    }

    if !event.liquidity_before_usd.is_finite()
        || !event.liquidity_after_usd.is_finite()
        || event.liquidity_before_usd <= 0.0
        || event.liquidity_after_usd < 0.0
    {
        return Err(TraceGuardError::InvalidLiquidity(
            "starting liquidity must be positive and ending liquidity must be non-negative"
                .to_string(),
        ));
    }

    match event.event_type {
        EventType::Withdrawal if event.amount_usd > event.liquidity_before_usd => {
            return Err(TraceGuardError::InvalidEvent(
                "withdrawal amount exceeds starting liquidity".to_string(),
            ));
        }
        _ => {}
    }

    Ok(())
}

// Add this function below validate_event()
pub fn validate_swap(swap: &SwapEvent) -> Result<(), TraceGuardError> {
    if swap.transaction_hash.trim().is_empty() {
        return Err(TraceGuardError::InvalidEvent(
            "transaction hash cannot be empty".to_string(),
        ));
    }

    if swap.pool_address.trim().is_empty() {
        return Err(TraceGuardError::InvalidEvent(
            "pool address cannot be empty".to_string(),
        ));
    }

    if !swap.amount_usd.is_finite() || swap.amount_usd < 0.0 {
        return Err(TraceGuardError::InvalidEvent(
            "swap amount must be finite and non-negative".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_event() -> ProtocolEvent {
        ProtocolEvent {
            event_id: "evt_001".to_string(),
            protocol: "DemoLend".to_string(),
            timestamp: 1_792_670_400,
            event_type: EventType::Withdrawal,
            amount_usd: 400_000.0,
            liquidity_before_usd: 1_000_000.0,
            liquidity_after_usd: 600_000.0,
        }
    }

    #[test]
    fn accepts_valid_event() {
        assert!(validate_event(&valid_event()).is_ok());
    }

    #[test]
    fn rejects_negative_amount() {
        let mut event = valid_event();
        event.amount_usd = -1.0;

        assert!(validate_event(&event).is_err());
    }

    #[test]
    fn rejects_zero_starting_liquidity() {
        let mut event = valid_event();
        event.liquidity_before_usd = 0.0;

        assert!(validate_event(&event).is_err());
    }

    #[test]
    fn rejects_empty_event_id() {
        let mut event = valid_event();
        event.event_id = "   ".to_string();

        assert!(validate_event(&event).is_err());
    }
}
