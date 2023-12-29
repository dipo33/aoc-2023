use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn my_benches(c: &mut Criterion) {
    c.bench_function("d10-parsing", |b| {
        let contents: String = std::fs::read_to_string("./day_10/inputs/input.txt")
            .expect("Should have been able to read the file");
        b.iter(|| {
            black_box({
                day_10::parser::parse(contents.as_str())
                    .expect("Should have been able to parse the file")
            });
        });
    });

    c.bench_function("d10p1", |b| {
        b.iter(|| {
            black_box({
                day_10::part1::execute("./day_10/inputs/input.txt", "", false);
            });
        });
    });

    c.bench_function("d10p2", |b| {
        b.iter(|| {
            black_box({
                day_10::part2::execute("./day_10/inputs/input.txt", "", false);
            });
        });
    });
}

criterion_group!(benches, my_benches);
criterion_main!(benches);
