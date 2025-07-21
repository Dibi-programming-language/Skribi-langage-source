use criterion::{criterion_group, criterion_main, Criterion};
use skribi_language_source::execute;

macro_rules! create_benchmark {
    ($c: expr, $name: expr, $path: expr) => {
        $c.bench_function($name, |b| {
            b.iter(|| {
                execute(
                    vec![
                    "".to_owned(),
                    $path.to_owned(),
                    ],
                    false,
                )
            })
        });
    };
}

pub fn criterion_general_benchmark(c: &mut Criterion) {
    create_benchmark!(c, "fibo 19 skribi", "resources/test_programs/algo/fibo.skrb");
    create_benchmark!(c, "or_eq skribi", "resources/test_programs/cmp/or_eq.skrb");
}

criterion_group!(benches, criterion_general_benchmark);
criterion_main!(benches);
