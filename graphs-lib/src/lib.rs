use std::hint::black_box;
use std::collections::HashMap;
use petgraph::csr::DefaultIx;
use polars::prelude::*;
use petgraph::{prelude::*, Graph};
use petgraph::visit::{Bfs, Dfs, VisitMap};
use bma_benchmark::benchmark;

pub trait AlgoMethods{
    fn initialize_graphs(size: Option<usize>) -> Self;
    fn bfs_algo(&self, src_node: NodeIndex, goal_node: NodeIndex);
    fn dfs_algo(&self, src_node: NodeIndex, goal_node: NodeIndex);
    fn bidirectional_search(&self, src_node: NodeIndex, goal_node: NodeIndex);
    fn benchmark_algos_version2(); 
    fn run_bfs(&self);
    fn run_dfs(&self);
    fn run_bidirectional(&self);
}
#[derive(Clone)]
pub struct Data{
   pub data: Vec<(i64, i64)>,
   pub graph: Graph<i64, (), Undirected>,
}
impl AlgoMethods for Data{


    fn run_bfs(&self){
        let node_count=self.graph.node_count();
        let src_node: NodeIndex=NodeIndex::new(0);
        let goal_node: NodeIndex=NodeIndex::new(node_count-1);
        self.bfs_algo(src_node, goal_node);
    }
    fn run_dfs(&self){
        let node_count=self.graph.node_count();
        let src_node: NodeIndex=NodeIndex::new(0);
        let goal_node: NodeIndex=NodeIndex::new(node_count-1);
        self.dfs_algo(src_node, goal_node);
    }

    fn run_bidirectional(&self){
        let node_count=self.graph.node_count();
        let src_node: NodeIndex=NodeIndex::new(0);
        let goal_node: NodeIndex=NodeIndex::new(node_count-1);
        self.bidirectional_search(src_node, goal_node);

    }
    
    fn benchmark_algos_version2() { 
        let algo_struct= Self::initialize_graphs(None);
        let src_node: NodeIndex=NodeIndex::new(0);
        let goal_node: NodeIndex=NodeIndex::new(900);
        //bfs
        let algo=&algo_struct;
        benchmark!(5,  {
            algo.bfs_algo(src_node, goal_node);
        });
        //dfs
        let algo=&algo_struct;
        benchmark!(5,  {
            algo.dfs_algo(src_node, goal_node);
        });
        //bidirectional search
        let algo=&algo_struct;
        benchmark!(5,  {
            algo.bidirectional_search(src_node, goal_node);
        });
    }

    fn initialize_graphs(size: Option<usize>) -> Self{
        let path= PlPath::new("input/dataset_algo.csv");
        let df: DataFrame=match size{
            Some(val) =>{
                LazyCsvReader::new(path).finish().unwrap().collect().unwrap().head(Some(val))
            },
            None => {
                LazyCsvReader::new(path).finish().unwrap().collect().unwrap()
            }
        };

        let data: Vec<_>= df.sort(vec!["Node to","Node from"],Default::default()).unwrap().get_columns()[0].i64().unwrap().iter().zip(
            df.get_columns()[1].i64().unwrap().iter()
            ).map(|(x,y)| (x.unwrap(), y.unwrap()
                )).collect();
        let mut graph_ = UnGraph::<i64, ()>::new_undirected();
        let mut node_map = HashMap::new();

        //a hashmap having the keys as the values from the datasets with value of the node index of the
        //petgraph
        //if the key or original value exists then it will return a reference to it so it can add a
        //connection with second node
        //same thing happens for the second tuple index y
        for (x, y) in data.clone().into_iter() {
            let nx = *node_map.entry(x).or_insert_with(|| graph_.add_node(x));
            let ny = *node_map.entry(y).or_insert_with(|| graph_.add_node(y));
            graph_.add_edge(nx, ny, ());
        }
        Data {
            data: data,
            graph: graph_,
        }
    }

    fn bfs_algo(&self,src_node: NodeIndex, goal_node: NodeIndex){
        let graph=&self.graph;
        let mut bfs= Bfs::new(&graph, src_node);
        println!("=== Breadth First Search Traversal ===");
        while let Some(node)= bfs.next(&graph){
            // println!("Visiting Node {:?}", graph[node]);
            if graph[node]==graph[goal_node]{
                println!("Intened Node Found {:?}", graph[node]);
                break;
            }
        }
    }

    fn dfs_algo(&self, src_node: NodeIndex, goal_node: NodeIndex){
        let graph=&self.graph;
        let mut dfs= Dfs::new(&graph, src_node);
        println!("=== Depth First Search Traversal ===");
        while let Some(node)= dfs.next(&graph){
            // println!("Visiting Node {:?}", graph[node]);
            if graph[node]==graph[goal_node]{
                println!("Intened Node Found {:?}", graph[node]);
                break;
            }
        }
    }


    fn bidirectional_search(&self,src_node: NodeIndex, goal_node: NodeIndex){
        let graph=&self.graph;
        let mut bfs_start= Bfs::new(&graph, src_node);
        let mut bfs_end= Bfs::new(&graph, goal_node);
        println!("=== Bidirectional Search ===");
        let mut found=false;
        while !found{
            //forward Search
            if let Some(node)=bfs_start.next(&graph){
                if bfs_end.discovered.is_visited(&node){
                    println!("intersection point is found node: {:?}", graph[node]);
                    found=true;
                }
            }
            //backword search
            if let Some(node)=bfs_end.next(&graph){
                if bfs_start.discovered.is_visited(&node){
                    println!("intersection point is found node: {:?}", graph[node]);
                    found=true;
                }
            }
            if bfs_start.next(&graph).is_none() && bfs_end.next(&graph).is_none(){
                println!("There is no path between nodes");
                break;
            }
        }
    }
}

