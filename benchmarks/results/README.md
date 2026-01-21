# Benchmark Results

## Latest Benchmark Run

**Date**: 2026-01-20  
**Branch**: v0.2.5  
**Commit**: fc7d281

## Results Summary

This directory contains the latest manual benchmark results for v0.2.5.

Notable changes from the latest run:
- Mostly improvements across component and system benchmarks.
- Regressions detected in `Select Component/create_large_1000` (~+2.3%) and `Mixed Workload/query_all_interactive` (~+28%).

### Performance Metrics

Criterion benchmark data is stored in subdirectories organized by benchmark name. Each directory contains:
- `estimates.json`: Statistical estimates for the benchmark

### Viewing Full Results

For detailed HTML reports with charts and analysis, run:
```bash
cargo bench --benches
```

Then open `target/criterion/report/index.html` in your browser.

## How to Update

When making performance-sensitive changes:

1. Run benchmarks: `cargo bench --benches`
2. Copy new results from `target/criterion/` to `benchmarks/results/`
3. Update this README with date and commit info
4. Commit the changes with your PR
