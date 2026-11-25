# TraceGuard

**A Rust project for monitoring swap activity in a Uniswap v3 liquidity pool.**

TraceGuard looks at real historical swap data from Ethereum and flags activity that stands out from the surrounding pattern. I built it to explore how Rust can be used for blockchain data processing, anomaly detection, API development, testing, and performance profiling.

It is a monitoring prototype: an alert means that activity is unusual compared with the chosen baseline, not that an attack has been confirmed.

## What does TraceGuard monitor?

The current version focuses on one Uniswap v3 WETH/USDC liquidity pool.

- **Large swaps:** Compares individual swap amounts against a statistical baseline using the median and Median Absolute Deviation (MAD).
- **Activity bursts:** Groups swaps into five-minute windows and checks for unusually high trading volume or swap counts.
- **Rolling activity:** Compares each five-minute window with earlier windows to identify sudden changes in activity. The implementation supports baselines of 5, 10, and 20 previous windows.

Alerts include measurements, thresholds, timestamps and transaction references where applicable.

## How it works

The application follows a simple workflow:

1. **Fetch data:** Retrieves up to 500 swap events from The Graph.
2. **Validate data:** Parses and validates the swap information before analysis.
3. **Detect unusual activity:** Runs the large-swap, activity-burst and rolling-window detectors.
4. **Generate alerts:** Records activity that crosses the calculated thresholds.
5. **Serve results:** Makes the monitoring results available through a local Axum API.

The current implementation fetches a dataset when the application starts and serves that monitoring snapshot. Continuous background monitoring is not implemented yet.

## Data source

TraceGuard uses real historical Uniswap v3 swap data retrieved through The Graph.

| Detail | Value |
|---|---|
| Network | Ethereum |
| Protocol | Uniswap v3 |
| Pool | WETH/USDC |
| Fee tier | 0.05% |
| Pool address | `0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640` |
| Recorded dataset | Up to 500 swap events |

The Graph API key is read from the `THEGRAPH_API_KEY` environment variable. It should be stored locally in a `.env` file and never committed to GitHub.

## Technology stack

| Technology | Purpose |
|---|---|
| Rust | Core application and detection logic |
| Tokio | Asynchronous runtime |
| Reqwest | Requests to The Graph |
| Serde / Serde JSON | Parsing and serialization |
| Axum | HTTP API |
| Thiserror | Error handling |
| Criterion | Performance benchmarking |
| Samply | Performance profiling |
| Inferno | Flamegraph generation |

## Running TraceGuard locally

### Requirements

- Rust and Cargo
- A The Graph API key with access to the Uniswap subgraph

### 1. Clone the repository

```bash
git clone https://github.com/mehpreet25/traceguard.git
cd traceguard
```

### 2. Configure the API key

Create a `.env` file in the project root:

```env
THEGRAPH_API_KEY=your_api_key_here
```

Keep this file private. It should not be pushed to GitHub.

### 3. Run the application

```bash
cargo run
```

TraceGuard fetches and analyses the swap data at startup. If the fetch succeeds, the API listens at:

`http://127.0.0.1:3000`

### 4. Try the API endpoints

| Endpoint | Description |
|---|---|
| `GET /health` | Checks whether the API is running |
| `GET /alerts` | Returns the monitoring snapshot, including swaps checked and generated alerts |

You can test them using:

```bash
curl http://127.0.0.1:3000/health
curl http://127.0.0.1:3000/alerts
```

## Testing

Run the automated tests with:

```bash
cargo test
```

The recorded test run completed with **12 tests passing**.

The tests cover areas including:

- Median calculation.
- Large-swap detection.
- Activity-burst detection.
- Rolling activity detection.
- Baseline edge cases.

Additional code-quality checks performed during development:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
```

## Performance benchmarking

Criterion was used to measure the execution time of the detection algorithms.

Recorded benchmark results from the development machine:

| Detector | Input size | Recorded time |
|---|---:|---:|
| Large swap | 500 swaps | 11.573 µs |
| Large swap | 5,000 swaps | 90.176 µs |
| Activity burst | 500 swaps | 11.308 µs |
| Rolling activity | 500 swaps | 169.65 µs |
| Rolling activity | 5,000 swaps | 1.7302 ms |

A later run of `rolling_detector_5000` reported an estimate of **1.7692 ms**, with a measurement interval of 1.7307–1.8229 ms.

Run the benchmarks again using:

```bash
cargo bench
```

These measurements can vary depending on machine load and environment. They measure detector execution on benchmark data, not the time taken to fetch data from The Graph.

## Performance profiling

I also used Samply to profile the `rolling_detector_5000` benchmark and Inferno to generate an SVG flamegraph.

The focused profiling process identified:

- **579 samples** whose call stacks contain `detect_activity_bursts_rolling`.
- **58 unique stack paths** in the filtered output.

These are filtered sample counts, not a measurement of the detector's exclusive CPU time. The flamegraph also contains surrounding runtime and system frames.

The generated profiling files are available in the `profiling/` directory:

| File | Description |
|---|---|
| `traceguard-focused-flamegraph.svg` | Focused flamegraph visualization |
| `traceguard-focused-profile.json.gz` | Saved Samply profile |
| `traceguard-focused.folded` | Folded stack data used to generate the flamegraph |

## Project structure

```text
traceguard/
├── benches/
│   └── detector_benchmark.rs
├── data/
│   └── swaps_snapshot.json
├── profiling/
│   ├── traceguard-focused-flamegraph.svg
│   ├── traceguard-focused-profile.json.gz
│   └── traceguard-focused.folded
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── detector.rs
│   ├── errors.rs
│   ├── models.rs
│   ├── uniswap.rs
│   └── validation.rs
├── Cargo.toml
└── README.md
```

## Current limitations and possible improvements

TraceGuard is still a focused prototype, so there are several areas that could be developed further:

- Add continuous or scheduled blockchain data fetching.
- Maintain longer historical datasets for more representative baselines.
- Add alert severity levels and filtering.
- Build a dashboard to visualise pool activity and alerts.
- Further isolate detector execution during performance profiling.
- Explore an AI-assisted explanation layer for summarising alert evidence, while keeping the detection logic measurable and rule-based.

## Why I built it

TraceGuard started as a way to learn more about DeFi monitoring while getting hands-on experience with Rust.

Working on it brought together several parts of software development: collecting real blockchain data, building detection algorithms, handling errors, exposing results through an API, writing tests and measuring performance.

There is still more that can be added, but this is the working foundation of TraceGuard.