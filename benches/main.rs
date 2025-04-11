// https://bheisler.github.io/criterion.rs/book/getting_started.html

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use remarkable::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dummy", |b| b.iter(|| black_box(dummy())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
