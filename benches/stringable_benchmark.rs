use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stringable::stringable::Stringable;

fn to_uppercase_benchmark(c: &mut Criterion) {
    c.bench_function("stringable to_uppercase", |b| {
        b.iter(|| {
            let input = black_box("Hello, World!");
            let stringable = Stringable::new(input).to_uppercase();
            stringable.get_value().to_string() // Convert &str to String
        })
    });
}

criterion_group!(benches, to_uppercase_benchmark);
criterion_main!(benches);
