use criterion::{black_box, Criterion, criterion_group, criterion_main};

use day_02::parser;

fn my_benches(c: &mut Criterion) {
    c.bench_function("d02-parsing", |b| {
        let contents: String = std::fs::read_to_string("./day_02/inputs/input.txt")
            .expect("Should have been able to read the file");
        b.iter(|| {
            black_box({
                parser::parse(contents.as_str())
                    .expect("Should have been able to parse the file")
            });
        });
    });

    c.bench_function("d02p1", |b| {
        b.iter(|| {
            black_box({
                day_02::part1::execute("./day_02/inputs/input.txt", "", false);
            });
        });
    });

    c.bench_function("d02p2", |b| {
        b.iter(|| {
            black_box({
                day_02::part2::execute("./day_02/inputs/input.txt", "", false);
            });
        });
    });
}

criterion_group!(benches, my_benches);
criterion_main!(benches);
