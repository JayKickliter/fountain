use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use fountaincode::xor::*;

fn bench_xor(c: &mut Criterion) {
    let mut lhs: Vec<u8> = (0_u32..1024).into_iter().map(|b| b as u8).collect();
    let rhs: Vec<u8> = (0_u32..1024).into_iter().map(|b| b as u8).collect();
    let mut group = c.benchmark_group("XOR");
    group.throughput(Throughput::Bytes(rhs.len() as u64));
    group.bench_function("Serial", |b| b.iter(|| xor_bytes_fallback(&mut lhs, &rhs)));
    group.bench_function("SSE2", |b| b.iter(|| unsafe{ xor_bytes_sse2(&mut lhs, &rhs) }));
    group.bench_function("AVX2", |b| b.iter(|| unsafe{ xor_bytes_avx2(&mut lhs, &rhs) }));
    group.finish();
}

criterion_group!(benches, bench_xor);
criterion_main!(benches);
