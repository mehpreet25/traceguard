use crate::errors::TraceGuardError;
use crate::models::SwapEvent;

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

    fn valid_swap() -> SwapEvent {
        SwapEvent {
            transaction_hash: "tx_001".to_string(),
            pool_address: "0x123".to_string(),
            timestamp: 1_700_000_000,
            amount_usd: 100.0,
            sender: "sender".to_string(),
            recipient: "recipient".to_string(),
        }
    }

    #[test]
    fn accepts_valid_swap() {
        assert!(validate_swap(&valid_swap()).is_ok());
    }

    #[test]
    fn rejects_empty_transaction_hash() {
        let mut swap = valid_swap();
        swap.transaction_hash = " ".to_string();
        assert!(validate_swap(&swap).is_err());
    }

    #[test]
    fn rejects_negative_amount() {
        let mut swap = valid_swap();
        swap.amount_usd = -1.0;
        assert!(validate_swap(&swap).is_err());
    }

    #[test]
    fn rejects_non_finite_amount() {
        let mut swap = valid_swap();
        swap.amount_usd = f64::NAN;
        assert!(validate_swap(&swap).is_err());
    }
}
