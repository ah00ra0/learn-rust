use bench_demo::*;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_alloc(c: &mut Criterion) {
    let mut g = c.benchmark_group("vec_build_100k");
    g.bench_function("no_capacity", |b| b.iter(|| build_no_capacity(black_box(100_000))));
    g.bench_function("with_capacity", |b| b.iter(|| build_with_capacity(black_box(100_000))));
    g.finish();
}

fn bench_sum(c: &mut Criterion) {
    let data = build_with_capacity(1_000_000);
    let mut g = c.benchmark_group("sum_1m");
    g.bench_function("indexed", |b| b.iter(|| sum_indexed(black_box(&data))));
    g.bench_function("iter", |b| b.iter(|| sum_iter(black_box(&data))));
    g.finish();

    let mut g = c.benchmark_group("even_squares_1m");
    g.bench_function("iter", |b| b.iter(|| sum_even_squares_iter(black_box(&data))));
    g.bench_function("loop", |b| b.iter(|| sum_even_squares_loop(black_box(&data))));
    g.bench_function("rayon", |b| b.iter(|| sum_even_squares_par(black_box(&data))));
    g.finish();
}

fn bench_layout(c: &mut Criterion) {
    let aos = make_aos(1_000_000);
    let soa = make_soa(1_000_000);
    let mut g = c.benchmark_group("total_mass_1m");
    g.bench_function("aos", |b| b.iter(|| total_mass_aos(black_box(&aos))));
    g.bench_function("soa", |b| b.iter(|| total_mass_soa(black_box(&soa))));
    g.finish();
}

fn bench_dispatch(c: &mut Criterion) {
    // مقایسه‌ی عادلانه ۱: فقط Square؛ تفاوت = هزینه‌ی heap + vtable
    let boxed_sq: Vec<Box<dyn Shape>> =
        (0..100_000).map(|i| Box::new(Square(i as f64)) as Box<dyn Shape>).collect();
    let plain_sq: Vec<Square> = (0..100_000).map(|i| Square(i as f64)).collect();
    let mut g = c.benchmark_group("area_100k_single_type");
    g.bench_function("dyn", |b| b.iter(|| total_area_dyn(black_box(&boxed_sq))));
    g.bench_function("generic", |b| b.iter(|| total_area_generic(black_box(&plain_sq))));
    g.finish();

    // مقایسه‌ی عادلانه ۲: ترکیب دو نوع؛ dyn در برابر enum
    let boxed_mix: Vec<Box<dyn Shape>> = (0..100_000)
        .map(|i| -> Box<dyn Shape> {
            if i % 2 == 0 { Box::new(Square(i as f64)) } else { Box::new(Circle(i as f64)) }
        })
        .collect();
    let enum_mix: Vec<AnyShape> = (0..100_000)
        .map(|i| if i % 2 == 0 { AnyShape::Square(i as f64) } else { AnyShape::Circle(i as f64) })
        .collect();
    let mut g = c.benchmark_group("area_100k_mixed");
    g.bench_function("dyn", |b| b.iter(|| total_area_dyn(black_box(&boxed_mix))));
    g.bench_function("enum", |b| b.iter(|| total_area_generic(black_box(&enum_mix))));
    g.finish();
}

criterion_group!(benches, bench_alloc, bench_sum, bench_layout, bench_dispatch);
criterion_main!(benches);
