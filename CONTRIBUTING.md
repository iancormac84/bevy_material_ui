# Contributing to bevy_material_ui

Thank you for your interest in contributing to bevy_material_ui!

## Development Setup

### Prerequisites

- Rust 1.80+ (latest stable recommended)
- Cargo
- Git

### Building from Source

```bash
git clone https://github.com/edgarhsanchez/bevy_material_ui.git
cd bevy_material_ui
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
cargo run --example showcase
cargo run --example button_demo
# See examples/ directory for more
```

## Benchmarking Requirements

**Important**: Before submitting a PR that affects performance-critical code, you must run benchmarks locally and include the results.

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench --benches

# Run specific benchmark suite
cargo bench --bench color_benchmarks
cargo bench --bench component_benchmarks
cargo bench --bench system_benchmarks
```

### Including Benchmark Results

When submitting PRs that may impact performance:

1. Run the relevant benchmarks locally
2. Copy benchmark data to `benchmarks/results/`:
   ```bash
   # Copy estimates for tracking
   find target/criterion -name "estimates.json" -exec sh -c 'mkdir -p "benchmarks/results/$(dirname {})" && cp {} "benchmarks/results/$(dirname {})"' \;
   ```
3. Update `benchmarks/results/README.md` with the date and commit hash
4. Include a summary of benchmark results in your PR description
5. If there are significant performance changes (>5%), explain why

Example PR description section:
```markdown
## Benchmark Results

Ran `cargo bench --benches` before and after changes:

- `hct_to_argb_single`: 2,103 ns/iter → 1,950 ns/iter (-7.3%)
- `palette_generation`: No significant change
- `color_scheme_creation`: 15,420 ns/iter → 15,380 ns/iter (~0%)

See `benchmarks/results/` for full data.
```

### Benchmark Output Location

Criterion stores results in:
- HTML reports: `target/criterion/*/report/index.html`
- Raw data: `target/criterion/*/base/`

You can view detailed results by opening the HTML reports in your browser.

## Code Quality

### Formatting

```bash
cargo fmt --all
```

### Linting

```bash
cargo clippy --all-targets --all-features
```

### Documentation

```bash
cargo doc --no-deps --open
```

## Pull Request Process

1. **Fork** the repository
2. **Create a branch** from `main` for your feature or fix
3. **Make your changes** with clear, descriptive commits
4. **Run tests** and ensure they pass
5. **Run benchmarks** if your changes affect performance
6. **Format code** with `cargo fmt`
7. **Check for warnings** with `cargo clippy`
8. **Update documentation** if adding new features
9. **Submit a PR** with a clear description of changes

### PR Description Template

```markdown
## Summary
Brief description of the changes

## Changes
- List of specific changes made

## Testing
How you tested your changes

## Benchmark Results (if applicable)
Performance impact summary

## Breaking Changes (if applicable)
What breaks and how to migrate
```

## Code Style

- Follow Rust standard naming conventions (snake_case, CamelCase, SCREAMING_SNAKE_CASE)
- Use meaningful variable and function names
- Add doc comments for public APIs
- Keep functions focused and reasonably sized
- Prefer explicitness over cleverness

## Questions?

Feel free to:
- Open an issue for discussion
- Ask in pull request comments
- Tag @edgarhsanchez for questions

Thank you for contributing!
