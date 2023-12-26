use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn my_benches(c: &mut Criterion) {
    c.bench_function("NodeId Comparison Bench", |b| {
        let foo_id = day_08::entity::Graph::label_to_node_id("ZZZ");
        let bar_id = day_08::entity::Graph::label_to_node_id("AAA");
        b.iter(|| {
            black_box({
                foo_id == bar_id
            });
        });
    });


    c.bench_function("Label Comparison Bench", |b| {
        let foo_id = "ZZZ";
        let bar_id = "AAA";
        b.iter(|| {
            black_box({
                foo_id == bar_id
            });
        });
    });


    c.bench_function("NodeId from Label Conversion Bench", |b| {
        let foo_id = "ZZZ";
        b.iter(|| {
            black_box({
                day_08::entity::Graph::label_to_node_id(foo_id)
            });
        });
    });


    c.bench_function("NodeId Conversion and Comparison Bench", |b| {
        let bar_id = day_08::entity::Graph::label_to_node_id("AAA");
        let baz_id = day_08::entity::Graph::label_to_node_id("BBB");
        let bay_id = day_08::entity::Graph::label_to_node_id("CCC");
        b.iter(|| {
            black_box({
                let foo_id = day_08::entity::Graph::label_to_node_id("ZZZ");
                foo_id == bar_id || foo_id == baz_id || foo_id == bay_id
            });
        });
    });


    c.bench_function("day_08_parsing", |b| {
        let contents: String = std::fs::read_to_string("./day_08/inputs/input.txt")
            .expect("Should have been able to read the file");
        b.iter(|| {
            black_box({
                day_08::parser::parse(&contents)
                    .expect("Should have been able to parse the file")
            });
        });
    });


    c.bench_function("day_08_part1", |b| {
        b.iter(|| {
            black_box({
                day_08::part1::execute("./day_08/inputs/input.txt", "", false);
            });
        });
    });
}

criterion_group!(benches, my_benches);
criterion_main!(benches);
