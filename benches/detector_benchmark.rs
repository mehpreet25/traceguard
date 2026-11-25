use criterion::{Criterion, black_box, criterion_group, criterion_main};
use traceguard::detector::{
    detect_activity_bursts, detect_activity_bursts_rolling, detect_large_swaps,
};
use traceguard::models::SwapEvent;

fn make_swaps(count: usize) -> Vec<SwapEvent> {
    (0..count)
        .map(|i| SwapEvent {
            transaction_hash: format!("tx-{i}"),
            pool_address: "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640".to_string(),
            timestamp: 1_700_000_000 + (i as u64 * 30),
            amount_usd: 100.0 + (i % 50) as f64,
            sender: "sender".to_string(),
            recipient: "recipient".to_string(),
        })
        .collect()
}

fn benchmark_detectors(c: &mut Criterion) {
    let swaps_500 = make_swaps(500);
    let swaps_5000 = make_swaps(5000);

    c.bench_function("large_swap_detector_500", |b| {
        b.iter(|| detect_large_swaps(black_box(&swaps_500)))
    });

    c.bench_function("activity_burst_detector_500", |b| {
        b.iter(|| detect_activity_bursts(black_box(&swaps_500)))
    });

    c.bench_function("rolling_detector_500", |b| {
        b.iter(|| detect_activity_bursts_rolling(black_box(&swaps_500), 5))
    });

    c.bench_function("large_swap_detector_5000", |b| {
        b.iter(|| detect_large_swaps(black_box(&swaps_5000)))
    });

    c.bench_function("rolling_detector_5000", |b| {
        b.iter(|| detect_activity_bursts_rolling(black_box(&swaps_5000), 5))
    });
}

criterion_group!(benches, benchmark_detectors);
criterion_main!(benches);
