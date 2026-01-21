use criterion::{black_box, Criterion};

/// Benchmark scroll container operations
pub fn bench_scroll_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Scroll Operations");

    // Benchmark scroll offset calculation
    group.bench_function("calculate_scroll_offset", |b| {
        let content_size: f32 = 2000.0;
        let visible_size: f32 = 400.0;
        let mut offset: f32 = 0.0;
        let delta: f32 = 20.0;

        b.iter(|| {
            // Simulate scroll update
            offset = (offset + delta).clamp(0.0, content_size - visible_size);
            black_box(offset)
        })
    });

    // Benchmark scrollbar thumb position calculation
    group.bench_function("calculate_thumb_position", |b| {
        let content_size: f32 = 2000.0;
        let visible_size: f32 = 400.0;
        let track_length: f32 = 380.0;
        let thumb_size = track_length * (visible_size / content_size);
        let offset: f32 = 100.0;

        b.iter(|| {
            let usable_track = track_length - thumb_size;
            let scroll_ratio = offset / (content_size - visible_size);
            let thumb_pos = scroll_ratio * usable_track;
            black_box(thumb_pos)
        })
    });

    // Benchmark normalized scroll (like Bevy's approach)
    group.bench_function("calculate_normalized_scroll", |b| {
        let content_size: f32 = 2000.0;
        let visible_size: f32 = 400.0;
        let track_length: f32 = 380.0;
        let offset: f32 = 100.0;

        b.iter(|| {
            let thumb_size = track_length * (visible_size / content_size);
            let usable_track = track_length - thumb_size;
            let max_scroll = content_size - visible_size;
            let thumb_pos = offset * usable_track / max_scroll;
            black_box((thumb_pos, thumb_size))
        })
    });

    group.finish();
}
