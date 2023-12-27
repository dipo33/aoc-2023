use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn my_benches(c: &mut Criterion) {
    c.bench_function("d%%FORMATTED_DAY%%-parsing", |b| {
        let contents: String = std::fs::read_to_string("./%%PACKAGE_NAME%%/inputs/input.txt")
            .expect("Should have been able to read the file");
        b.iter(|| {
            black_box({
                %%PACKAGE_NAME%%::parser::parse(contents.as_str())
                    .expect("Should have been able to parse the file")
            });
        });
    });

    c.bench_function("d%%FORMATTED_DAY%%p1", |b| {
        b.iter(|| {
            black_box({
                %%PACKAGE_NAME%%::part1::execute("./%%PACKAGE_NAME%%/inputs/input.txt", "", false);
            });
        });
    });

    c.bench_function("d%%FORMATTED_DAY%%p2", |b| {
        b.iter(|| {
            black_box({
                %%PACKAGE_NAME%%::part2::execute("./%%PACKAGE_NAME%%/inputs/input.txt", "", false);
            });
        });
    });
}

criterion_group!(benches, my_benches);
criterion_main!(benches);
