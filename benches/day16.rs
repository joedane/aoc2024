use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../src/bin/d16.rs"]
mod d16;

pub fn criterion_benchmark(c: &mut Criterion) {
    println!("XXXXXX");
    c.bench_function("d16", |b| b.iter(|| d16::main()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
