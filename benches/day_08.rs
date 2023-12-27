use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn my_benches(c: &mut Criterion) {
    c.bench_function("d08aux - Node Id Comparison", |b| {
        let foo_id = day_08::entity::Graph::label_to_node_id("ZZZ");
        let bar_id = day_08::entity::Graph::label_to_node_id("AAA");
        b.iter(|| {
            black_box({
                foo_id == bar_id
            });
        });
    });


    c.bench_function("d08aux - Node Label Comparison", |b| {
        let foo_id = "ZZZ";
        let bar_id = "AAA";
        b.iter(|| {
            black_box({
                foo_id == bar_id
            });
        });
    });


    c.bench_function("d08aux - Node Label to Id Conversion", |b| {
        let foo_id = "ZZZ";
        b.iter(|| {
            black_box({
                day_08::entity::Graph::label_to_node_id(foo_id)
            });
        });
    });


    c.bench_function("d08-parsing", |b| {
        let contents: String = std::fs::read_to_string("./day_08/inputs/input.txt")
            .expect("Should have been able to read the file");
        b.iter(|| {
            black_box({
                day_08::parser::parse(&contents)
                    .expect("Should have been able to parse the file")
            });
        });
    });


    c.bench_function("d08p1", |b| {
        b.iter(|| {
            black_box({
                day_08::part1::execute("./day_08/inputs/input.txt", "", false);
            });
        });
    });


    c.bench_function("d08p2", |b| {
        b.iter(|| {
            black_box({
                day_08::part2::execute("./day_08/inputs/input.txt", "", false);
            });
        });
    });
}

criterion_group!(benches, my_benches);
criterion_main!(benches);
