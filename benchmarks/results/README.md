# Benchmark Results

## Latest Benchmark Run

**Date**: 2026-01-03  
**Branch**: v0.2.4  
**Commit**: d4d856f

## Results Summary

This directory contains the baseline benchmark results for the v0.2.4 release.

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
