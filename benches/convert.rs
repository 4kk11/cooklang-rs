use cooklang::{
    convert::{ConvertTo, System, UnitsFile},
    Converter, Quantity, Value,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("conversions");
    let converter = Converter::default();

    let input = vec![
        (1.5, "tsp"),
        (2.0, "tsp"),
        (3.0, "tsp"),
        (3.5, "tbsp"),
        (300.0, "ml"),
        (1.5, "l"),
        (20.0, "g"),
    ]
    .into_iter()
    .map(|(v, u)| Quantity::new(Value::Number(v.into()), Some(u.to_string())))
    .collect::<Vec<_>>();

    let input = black_box(input);

    group.bench_with_input("fractions", &input, |b, input| {
        b.iter(|| {
            let mut input = input.clone();
            for q in &mut input {
                let _ = q.convert(ConvertTo::Best(System::Imperial), &converter);
                let _ = q.fit(&converter);
            }
        })
    });
    group.bench_with_input("regular", &input, |b, input| {
        b.iter(|| {
            let mut input = input.clone();
            for q in &mut input {
                let _ = q.convert(ConvertTo::Best(System::Metric), &converter);
                let _ = q.fit(&converter);
            }
        })
    });
}

fn bundled_units(c: &mut Criterion) {
    let mut group = c.benchmark_group("bundled_units");
    group.bench_function("get_bundled", |b| {
        b.iter_batched(
            || {},
            |_| UnitsFile::bundled(),
            criterion::BatchSize::NumIterations((100_000_000 / size_of::<UnitsFile>()) as u64),
        );
    });
    group.bench_function("parse_toml", |b| {
        b.iter_batched(
            || {},
            |_| toml::from_str::<UnitsFile>(include_str!("../units.toml")),
            criterion::BatchSize::NumIterations((100_000_000 / size_of::<UnitsFile>()) as u64),
        );
    });
}

criterion_group!(benches, conversions, bundled_units);
criterion_main!(benches);
