use criterion::{criterion_group, criterion_main, Criterion};
use skribi_language_source::execute;

pub fn criterion_general_benchmark(c: &mut Criterion) {
    c.bench_function("fibo 19 skribi", |b| {
        b.iter(|| {
            execute(
                vec![
                    "".to_owned(),
                    "resources/test_programs/algo/fibo.skrb".to_owned(),
                ],
                false,
            )
        })
    });
}

criterion_group!(benches, criterion_general_benchmark);
criterion_main!(benches);
