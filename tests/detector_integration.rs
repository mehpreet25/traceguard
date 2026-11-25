use traceguard::detector::detect_large_swaps;
use traceguard::models::SwapEvent;

#[test]
fn integration_detects_unusually_large_swap() {
    let mut swaps: Vec<SwapEvent> = (0..20)
        .map(|i| SwapEvent {
            transaction_hash: format!("tx-{i}"),
            pool_address: "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640".to_string(),
            timestamp: 1_700_000_000 + i as u64,
            amount_usd: 100.0 + i as f64,
            sender: "sender".to_string(),
            recipient: "recipient".to_string(),
        })
        .collect();

    swaps.push(SwapEvent {
        transaction_hash: "unusual-tx".to_string(),
        pool_address: "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640".to_string(),
        timestamp: 1_700_000_100,
        amount_usd: 10_000.0,
        sender: "sender".to_string(),
        recipient: "recipient".to_string(),
    });

    let alerts = detect_large_swaps(&swaps);

    assert!(
        !alerts.is_empty(),
        "Expected the detector to identify the unusually large swap"
    );
}
