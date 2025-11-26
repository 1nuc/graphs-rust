use criterion::{criterion_group, criterion_main, Criterion};
use graphs_lib::{AlgoMethods, Data};

fn benchmark_bfs(c: &mut Criterion){
    let mut group= c.benchmark_group("BFS");
    for size in [1000,10000, 30000, 50000]{
        let algo=Data::initialize_graphs(Some(size));
        group.bench_function(format!("size: {}", size), |b| b.iter(|| algo.run_bfs()));
    }
    let algo=Data::initialize_graphs(None);
    group.bench_function("size: full", |b| b.iter(|| algo.run_bfs()));
    group.finish();
}
fn benchmark_dfs(c: &mut Criterion){
    let mut group= c.benchmark_group("DFS");
    for size in [1000,10000, 30000, 50000]{
        let algo=Data::initialize_graphs(Some(size));
        group.bench_function(format!("size: {}", size), |b| b.iter(|| algo.run_dfs()));
    }
    let algo=Data::initialize_graphs(None);
    group.bench_function("size: full", |b| b.iter(|| algo.run_dfs()));
    group.finish();
}
fn benchmark_bidirectional(c: &mut Criterion){
    let mut group= c.benchmark_group("Bidirectional Search");
    for size in [1000,10000, 30000, 50000]{
        let algo=Data::initialize_graphs(Some(size));
        group.bench_function(format!("size: {}", size), |b| b.iter(|| algo.run_bidirectional()));
    }
    let algo=Data::initialize_graphs(None);
    group.bench_function("size: full", |b| b.iter(|| algo.run_bidirectional()));
    group.finish();
}
criterion_group!(benches,benchmark_bfs, benchmark_dfs, benchmark_bidirectional);
criterion_main!(benches);
