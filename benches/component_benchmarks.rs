//! Component Benchmarks
//!
//! Measures the performance of UI component creation and configuration.
//! These benchmarks focus on the data structure operations without the ECS.

mod components;

use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    components::button::bench_button_creation,
    components::checkbox::bench_checkbox,
    components::switch::bench_switch,
    components::radio::bench_radio,
    components::slider::bench_slider,
    components::select::bench_select,
    components::progress::bench_progress,
    components::chip::bench_chip,
    components::fab::bench_fab,
    components::icon_button::bench_icon_button,
    components::card::bench_card,
    components::list::bench_list,
    components::loading_indicator::bench_loading_indicator,
    components::search_bar::bench_search_bar,
    components::divider::bench_divider,
    components::tokens::bench_tokens,
    components::state_changes::bench_state_changes,
);

criterion_main!(benches);
