use criterion::{criterion_group, criterion_main, Criterion};
use graphs_lib::{AlgoMethods, Data};

fn run_benches(c: &mut Criterion){
    let algo=Data::initialize_graphs();
    let mut group= c.benchmark_group("algorthmics");
    group.bench_function("BFS", |b| b.iter(|| algo.run_bfs()));
    group.bench_function("DFS", |b| b.iter(|| algo.run_dfs()));
    group.bench_function("Bidirectional", |b| b.iter(|| algo.run_bidirectional()));
    group.finish();
}
criterion_group!(benches, run_benches);
criterion_main!(benches);
