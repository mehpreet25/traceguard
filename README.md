# TraceGuard

**A Rust-based Uniswap V3 Swap Activity Monitoring and Anomaly Detection System**

## 1. Overview

TraceGuard monitors swap activity in a Uniswap V3 liquidity pool on Ethereum. It analyses historical blockchain transactions and identifies unusual trading behaviour using statistical baselines and rule-based anomaly detection.

The objective is to highlight potentially suspicious activity and provide measurable evidence for further investigation.

TraceGuard is an anomaly-monitoring prototype, not a system that guarantees detection of every DeFi attack.

## 2. Problem Statement

Decentralised finance protocols can experience unusual trading patterns, including:

- Unusually large swaps.
- Sudden increases in trading volume.
- Abnormal bursts in transaction frequency.

Monitoring these patterns can help identify activity that deserves further investigation.

## 3. Technology Stack

| Technology | Purpose |
|---|---|
| Rust | Core application and detection logic |
| Tokio | Asynchronous runtime |
| Axum | HTTP API |
| The Graph | Historical Uniswap blockchain data |
| Serde / Serde JSON | Data serialization |
| Reqwest | HTTP requests |
| Thiserror | Error handling |
| Criterion | Performance benchmarking |

## 4. Architecture

1. **Data collection:** Fetch historical swap events from The Graph.
2. **Data validation:** Validate transaction information and swap amounts.
3. **Anomaly detection:** Apply statistical and rule-based detection algorithms.
4. **Alert generation:** Generate alerts containing measurements, thresholds, timestamps and transaction references.
5. **API:** Expose monitoring results through HTTP endpoints.

## 5. Detection Mechanisms

### Large Swap Detection

Uses the median and Median Absolute Deviation (MAD) to establish a statistical baseline for swap amounts.

Swaps exceeding the calculated threshold are flagged.

### Activity Burst Detection

Groups swaps into five-minute windows and analyses transaction frequency and trading volume.

### Rolling Activity Detection

Compares the current five-minute window against previous windows to identify sudden changes in activity.

The implementation supports rolling baselines of 5, 10 and 20 windows.

## 6. Data Source

TraceGuard uses real historical Uniswap V3 swap data retrieved through The Graph.

- Network: Ethereum
- Pool: WETH/USDC
- Pool address: `0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640`
- Dataset used in the recorded run: 500 swap events

The API key is stored in an environment variable and should never be committed to Git.

## 7. API Endpoints

Start the application:

```bash
cargo run
```

The API runs locally at:

`http://127.0.0.1:3000`

| Endpoint | Description |
|---|---|
| GET /health | Checks whether TraceGuard is running |
| GET /alerts | Returns monitoring results and generated alerts |

The current implementation fetches data at startup and serves the resulting monitoring snapshot.

## 8. Testing

Automated tests are implemented using Rust's built-in testing framework.

Run:

```bash
cargo test
```

**Recorded result: 12 tests passed.**

The tests cover:

- Median calculation.
- Large swap detection.
- Normal swap handling.
- Activity burst detection.
- Rolling activity detection.
- Minimum baseline requirements.

## 9. Performance Benchmarking

Criterion was used to benchmark the detection algorithms.

Recorded average execution times:

| Detector | Dataset | Execution time |
|---|---:|---:|
| Large swap | 500 swaps | 11.573 µs |
| Large swap | 5,000 swaps | 90.176 µs |
| Activity burst | 500 swaps | 11.308 µs |
| Rolling activity | 500 swaps | 169.65 µs |
| Rolling activity | 5,000 swaps | 1.7302 ms |

Run benchmarks using:

```bash
cargo bench
```

These measurements represent detector execution time on the development machine, excluding network requests.

## 10. Project Structure

```text
traceguard/
├── Cargo.toml
├── Cargo.lock
├── .env
├── .gitignore
├── README.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── detector.rs
│   ├── errors.rs
│   ├── models.rs
│   ├── uniswap.rs
│   └── validation.rs
├── data/
│   └── swaps_snapshot.json
└── benches/
    └── detector_benchmark.rs
```

## 11. Running the Project

### Prerequisites

- Rust and Cargo
- A The Graph API key

### Configuration

Create a `.env` file in the project root:

```env
THEGRAPH_API_KEY=your_api_key_here
```

Do not commit this file.

### Run the application

```bash
cargo run
```

### Run tests

```bash
cargo test
```

### Run benchmarks

```bash
cargo bench
```

## 12. Current Limitations and Future Improvements

- Implement continuous or scheduled blockchain monitoring.
- Improve anomaly thresholds using additional historical data.
- Add alert severity classification.
- Add a dashboard for monitoring and visualising alerts.
- Investigate CPU profiling with a compatible macOS profiling setup.
- Explore an AI-assisted explanation layer for generated alerts.

## 13. Conclusion

TraceGuard demonstrates how Rust can be used to build a blockchain monitoring prototype combining real historical Uniswap data, statistical anomaly detection, automated testing, HTTP APIs and performance benchmarking.

The project provides a foundation for further research into DeFi monitoring and suspicious transaction activity.