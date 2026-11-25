use crate::models::{Alert, SwapEvent};
use std::collections::BTreeMap;

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);

    let middle = values.len() / 2;

    if values.len().is_multiple_of(2) {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    }
}

// 1. Detect unusually large individual swaps.

pub fn detect_large_swaps(swaps: &[SwapEvent]) -> Vec<Alert> {
    let mut alerts = Vec::new();

    if swaps.len() < 20 {
        println!("Not enough swaps to establish a baseline.");
        return alerts;
    }

    let mut amounts: Vec<f64> = swaps.iter().map(|swap| swap.amount_usd).collect();

    let baseline = median(&mut amounts);

    let mut deviations: Vec<f64> = amounts
        .iter()
        .map(|amount| (amount - baseline).abs())
        .collect();

    let mad = median(&mut deviations);

    if mad == 0.0 {
        println!("Baseline variability is zero; detection skipped.");
        return alerts;
    }

    let threshold = baseline + 3.0 * 1.4826 * mad;

    println!("\n--- Large swap analysis ---");
    println!("Median swap volume: ${baseline:.2}");
    println!("Anomaly threshold: ${threshold:.2}");

    for swap in swaps {
        if swap.amount_usd > threshold {
            println!(
                "UNUSUAL SWAP | Volume: ${:.2} | Timestamp: {} | Tx: {}",
                swap.amount_usd, swap.timestamp, swap.transaction_hash
            );

            alerts.push(Alert {
                alert_type: "Large Swap".to_string(),
                timestamp: swap.timestamp,
                measured_value: swap.amount_usd,
                threshold,
                transaction_hash: Some(swap.transaction_hash.clone()),
                description: format!(
                    "Swap volume ${:.2} exceeded the threshold of ${:.2}",
                    swap.amount_usd, threshold
                ),
            });
        }
    }

    println!("Unusual swaps detected: {}", alerts.len());

    alerts
}

// 2. Detect bursts using the overall activity baseline.
pub fn detect_activity_bursts(swaps: &[SwapEvent]) -> Vec<Alert> {
    const WINDOW_SECONDS: u64 = 5 * 60;

    let mut alerts = Vec::new();

    if swaps.is_empty() {
        println!("No swaps available for activity analysis.");
        return alerts;
    }

    let mut windows: BTreeMap<u64, (f64, usize)> = BTreeMap::new();

    for swap in swaps {
        let window_start = (swap.timestamp / WINDOW_SECONDS) * WINDOW_SECONDS;
        let stats = windows.entry(window_start).or_insert((0.0, 0));
        stats.0 += swap.amount_usd;
        stats.1 += 1;
    }

    if windows.len() < 5 {
        println!(
            "Not enough five-minute windows for burst analysis: {}",
            windows.len()
        );
        return alerts;
    }

    let mut volumes: Vec<f64> = windows.values().map(|stats| stats.0).collect();
    let mut counts: Vec<f64> = windows.values().map(|stats| stats.1 as f64).collect();

    let volume_median = median(&mut volumes);
    let count_median = median(&mut counts);

    let mut volume_deviations: Vec<f64> = volumes
        .iter()
        .map(|value| (value - volume_median).abs())
        .collect();

    let mut count_deviations: Vec<f64> = counts
        .iter()
        .map(|value| (value - count_median).abs())
        .collect();

    let volume_mad = median(&mut volume_deviations);
    let count_mad = median(&mut count_deviations);

    let volume_threshold = if volume_mad > 0.0 {
        volume_median + 2.0 * 1.4826 * volume_mad
    } else {
        volume_median * 2.0
    };

    let count_threshold = if count_mad > 0.0 {
        count_median + 3.0 * 1.4826 * count_mad
    } else {
        count_median * 2.0
    };

    println!("\n--- Five-minute activity analysis ---");
    println!("Windows analyzed: {}", windows.len());
    println!("Median window volume: ${volume_median:.2}");
    println!("Median swaps per window: {count_median:.1}");

    for (timestamp, (volume, count)) in &windows {
        let volume_burst = *volume > volume_threshold;
        let count_burst = (*count as f64) > count_threshold;

        if volume_burst {
            alerts.push(Alert {
                alert_type: "Activity Burst - Volume".to_string(),
                timestamp: *timestamp,
                measured_value: *volume,
                threshold: volume_threshold,
                transaction_hash: None,
                description: format!(
                    "Five-minute trading volume ${:.2} exceeded threshold ${:.2}",
                    volume, volume_threshold
                ),
            });
        }

        if count_burst {
            alerts.push(Alert {
                alert_type: "Activity Burst - Swap Count".to_string(),
                timestamp: *timestamp,
                measured_value: *count as f64,
                threshold: count_threshold,
                transaction_hash: None,
                description: format!(
                    "Five-minute swap count {} exceeded threshold {:.1}",
                    count, count_threshold
                ),
            });
        }

        if volume_burst || count_burst {
            println!(
                "ACTIVITY BURST | Window: {} | Volume: ${:.2} | Swaps: {} | Volume alert: {} | Count alert: {}",
                timestamp, volume, count, volume_burst, count_burst
            );
        }
    }

    println!("Activity-burst alerts generated: {}", alerts.len());

    alerts
}

// 3. Detect bursts using only preceding windows as the baseline.
pub fn detect_activity_bursts_rolling(swaps: &[SwapEvent], baseline_windows: usize) -> Vec<Alert> {
    const WINDOW_SECONDS: u64 = 5 * 60;

    let mut alerts = Vec::new();

    if baseline_windows == 0 {
        println!("Baseline must contain at least one window.");
        return alerts;
    }

    if swaps.is_empty() {
        println!("No swaps available for rolling analysis.");
        return alerts;
    }

    let mut windows: BTreeMap<u64, (f64, usize)> = BTreeMap::new();

    for swap in swaps {
        let start = (swap.timestamp / WINDOW_SECONDS) * WINDOW_SECONDS;
        let stats = windows.entry(start).or_insert((0.0, 0));
        stats.0 += swap.amount_usd;
        stats.1 += 1;
    }

    let Some((&first, _)) = windows.first_key_value() else {
        return alerts;
    };

    let Some((&last, _)) = windows.last_key_value() else {
        return alerts;
    };

    let mut timeline = Vec::new();
    let mut start = first;

    while start <= last {
        let (volume, count) = windows.get(&start).copied().unwrap_or((0.0, 0));
        timeline.push((start, volume, count));
        start += WINDOW_SECONDS;
    }

    if timeline.len() <= baseline_windows {
        println!("Not enough windows for rolling baseline.");
        return alerts;
    }

    println!("\n--- Rolling five-minute activity analysis ---");
    println!("Total windows, including quiet periods: {}", timeline.len());
    println!("Baseline length: {baseline_windows} earlier windows");

    for index in baseline_windows..timeline.len() {
        let history = &timeline[index - baseline_windows..index];

        let mut past_volumes: Vec<f64> = history.iter().map(|entry| entry.1).collect();

        let mut past_counts: Vec<f64> = history.iter().map(|entry| entry.2 as f64).collect();

        let volume_median = median(&mut past_volumes);
        let count_median = median(&mut past_counts);

        let mut volume_deviations: Vec<f64> = past_volumes
            .iter()
            .map(|value| (value - volume_median).abs())
            .collect();

        let mut count_deviations: Vec<f64> = past_counts
            .iter()
            .map(|value| (value - count_median).abs())
            .collect();

        let volume_mad = median(&mut volume_deviations);
        let count_mad = median(&mut count_deviations);

        let volume_threshold = if volume_mad > 0.0 {
            volume_median + 2.0 * 1.4826 * volume_mad
        } else {
            volume_median * 2.0
        };

        let count_threshold = if count_mad > 0.0 {
            count_median + 3.0 * 1.4826 * count_mad
        } else {
            count_median * 2.0
        };

        let (timestamp, volume, count) = timeline[index];

        let volume_alert = volume > volume_threshold;
        let count_alert = (count as f64) > count_threshold;

        println!(
            "WINDOW | {} | Volume: ${:.2} | Volume threshold: ${:.2} | Swaps: {} | Count threshold: {:.1}",
            timestamp, volume, volume_threshold, count, count_threshold
        );

        if volume_alert {
            alerts.push(Alert {
                alert_type: "Rolling Activity Burst - Volume".to_string(),
                timestamp,
                measured_value: volume,
                threshold: volume_threshold,
                transaction_hash: None,
                description: format!(
                    "Five-minute trading volume ${:.2} exceeded rolling threshold ${:.2} based on {} previous windows",
                    volume, volume_threshold, baseline_windows
                ),
            });
        }

        if count_alert {
            alerts.push(Alert {
                alert_type: "Rolling Activity Burst - Swap Count".to_string(),
                timestamp,
                measured_value: count as f64,
                threshold: count_threshold,
                transaction_hash: None,
                description: format!(
                    "Five-minute swap count {} exceeded rolling threshold {:.1} based on {} previous windows",
                    count, count_threshold, baseline_windows
                ),
            });
        }

        if volume_alert || count_alert {
            println!(
                "ROLLING ALERT | Window: {} | Volume: ${:.2} (threshold ${:.2}) | Swaps: {} (threshold {:.1})",
                timestamp, volume, volume_threshold, count, count_threshold
            );
        }
    }

    println!("Rolling activity-burst alerts generated: {}", alerts.len());

    alerts
}

#[cfg(test)]
mod tests {
    use super::{
        detect_activity_bursts, detect_activity_bursts_rolling, detect_large_swaps, median,
    };
    use crate::models::SwapEvent;

    fn make_swap(timestamp: u64, amount_usd: f64, index: usize) -> SwapEvent {
        SwapEvent {
            transaction_hash: format!("tx-{timestamp}-{index}"),
            pool_address: "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640".to_string(),
            timestamp,
            amount_usd,
            sender: "sender".to_string(),
            recipient: "recipient".to_string(),
        }
    }

    #[test]
    fn test_median_with_odd_number_of_values() {
        let mut values = vec![9.0, 1.0, 5.0];
        assert_eq!(median(&mut values), 5.0);
    }

    #[test]
    fn test_median_with_even_number_of_values() {
        let mut values = vec![8.0, 2.0, 4.0, 6.0];
        assert_eq!(median(&mut values), 5.0);
    }

    #[test]
    fn detects_unusually_large_swap() {
        let mut swaps = Vec::new();

        for i in 0..20 {
            let amount = if i % 2 == 0 { 100.0 } else { 110.0 };
            swaps.push(make_swap(1000 + i as u64, amount, i));
        }

        swaps.push(make_swap(2000, 1000.0, 20));

        let alerts = detect_large_swaps(&swaps);

        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].alert_type, "Large Swap");
        assert_eq!(alerts[0].measured_value, 1000.0);
    }

    #[test]
    fn does_not_flag_normal_swaps() {
        let mut swaps = Vec::new();

        for i in 0..20 {
            let amount = if i % 2 == 0 { 100.0 } else { 110.0 };
            swaps.push(make_swap(1000 + i as u64, amount, i));
        }

        let alerts = detect_large_swaps(&swaps);

        assert!(alerts.is_empty());
    }

    #[test]
    fn detects_activity_burst() {
        let mut swaps = Vec::new();

        // Five normal five-minute windows.
        for window in 0..5 {
            for i in 0..2 {
                swaps.push(make_swap(window * 300 + 10, 100.0, i as usize));
            }
        }

        // One unusually active window.
        for i in 0..10 {
            swaps.push(make_swap(1500 + 10, 100.0, i));
        }

        let alerts = detect_activity_bursts(&swaps);

        assert!(
            alerts
                .iter()
                .any(|alert| { alert.alert_type == "Activity Burst - Swap Count" })
        );

        assert!(
            alerts
                .iter()
                .any(|alert| { alert.alert_type == "Activity Burst - Volume" })
        );
    }

    #[test]
    fn detects_rolling_activity_burst() {
        let mut swaps = Vec::new();

        // Five normal windows establish the baseline.
        for window in 0..5 {
            for i in 0..2 {
                swaps.push(make_swap(window * 300 + 10, 100.0, i as usize));
            }
        }

        // The sixth window contains an unusual burst.
        for i in 0..10 {
            swaps.push(make_swap(1510, 100.0, i));
        }

        let alerts = detect_activity_bursts_rolling(&swaps, 5);

        assert!(
            alerts
                .iter()
                .any(|alert| { alert.alert_type == "Rolling Activity Burst - Swap Count" })
        );

        assert!(
            alerts
                .iter()
                .any(|alert| { alert.alert_type == "Rolling Activity Burst - Volume" })
        );
    }

    #[test]
    fn rolling_detector_rejects_zero_baseline_length() {
        let swaps = vec![make_swap(100, 100.0, 0)];

        let alerts = detect_activity_bursts_rolling(&swaps, 0);

        assert!(alerts.is_empty());
    }

    #[test]
    fn large_swap_detector_requires_at_least_twenty_swaps() {
        let swaps = vec![make_swap(100, 100.0, 0); 19];

        let alerts = detect_large_swaps(&swaps);

        assert!(alerts.is_empty());
    }
}
