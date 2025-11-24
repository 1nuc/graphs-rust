use std::collections::HashMap;
use polars::prelude::*;
use petgraph::prelude::*;
use petgraph::visit::Bfs;

fn create_graph(data: Vec<(i64, i64)>) -> Graph<i64, (), Undirected>{
    let mut graph = UnGraph::<i64, ()>::new_undirected();
    let mut node_map = HashMap::new();

    //a hashmap having the keys as the values from the datasets with value of the node index of the
    //petgraph
    //if the key or original value exists then it will return a reference to it so it can add a
    //connection with second node
    //same thing happens for the second tuple index y
    for (x, y) in data.into_iter() {
        let nx = *node_map.entry(x).or_insert_with(|| graph.add_node(x));
        let ny = *node_map.entry(y).or_insert_with(|| graph.add_node(y));
        graph.add_edge(nx, ny, ());
    }
    graph
}

fn bfs_algo(src_node: NodeIndex, goal_node: NodeIndex, graph: Graph<i64, (), Undirected>){
    let mut bfs= Bfs::new(&graph, src_node);
    while let Some(node)= bfs.next(&graph){
        println!("Visiting Node {:?}", graph[node]);
        if graph[node]==graph[goal_node]{
            println!("Intened Node Found {:?}", graph[node]);
            break;
        }
    }
}

fn main() {
    let path= PlPath::new("input/dataset_algo.csv");
    
    let df= LazyCsvReader::new(path).finish().unwrap().collect().unwrap();
    let data: Vec<_>= df.get_columns()[0].i64().unwrap().iter().zip(
        df.get_columns()[1].i64().unwrap().iter()
        ).map(|(x,y)| (x.unwrap(), y.unwrap()
            )).collect();

    let graph= create_graph(data);
    let src_node: NodeIndex=NodeIndex::new(0);
    let goal_node: NodeIndex=NodeIndex::new(900);
    bfs_algo(src_node, goal_node, graph);

}
