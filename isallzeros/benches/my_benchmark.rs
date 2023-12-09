use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use isallzeros::is_all_zeros_fast;
use isallzeros::is_all_zeros_iter;
use isallzeros::is_all_zeros_simd;
use isallzeros::is_all_zeros_stupid;

fn bench_fibs(c: &mut Criterion) {
    let data: Vec<u8> = vec![0; 1024 * 1024];
    let mut group = c.benchmark_group("is_all_zeros");

    group.bench_with_input(BenchmarkId::new("stupid", 0), &data, |b, data| {
        b.iter(|| is_all_zeros_stupid(data))
    });
    group.bench_with_input(BenchmarkId::new("iter", 0), &data, |b, data| {
        b.iter(|| is_all_zeros_iter(data))
    });
    group.bench_with_input(BenchmarkId::new("fast", 0), &data, |b, data| {
        b.iter(|| is_all_zeros_fast(data))
    });
    group.bench_with_input(BenchmarkId::new("simd", 0), &data, |b, data| {
        b.iter(|| is_all_zeros_simd(data))
    });

    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
