use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn my_benches(c: &mut Criterion) {
    c.bench_function("d01p1", |b| {
        b.iter(|| {
            black_box({
                day_01::part1::execute("./day_01/inputs/input.txt", "", false);
            });
        });
    });

    c.bench_function("d01p2", |b| {
        b.iter(|| {
            black_box({
                day_01::part2::execute("./day_01/inputs/input.txt", "", false);
            });
        });
    });

    c.bench_function("d01p2-trie", |b| {
        b.iter(|| {
            black_box({
                day_01::part2_trie::execute("./day_01/inputs/input.txt", "", false);
            });
        });
    });
}

criterion_group!(benches, my_benches);
criterion_main!(benches);
