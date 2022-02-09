use shared;
use petgraph::Graph;
use petgraph::dot::Dot; 
use petgraph::visit::Bfs;
use petgraph::visit::Dfs;
use petgraph::graphmap::GraphMap;
use petgraph::graphmap::NodeTrait;
use std::fmt;
use core::marker;
use core::cmp;
use core::hash;
use std::cmp::Ordering;

//finish parsing
//plan: find all paths using bfs kinda thing, the stop/remove the ones that contain two hits of a
//small cave, or something along those lines, check for many and get rid of them as small cave issues arise 
fn main() {
    //let g = initialize_test_graph();
    //println!("{:?}", Dot::new(&g));
    get_graph_bearings(); 
    //parse_input("./input2.txt");


}


fn parse_input(filename: &str) -> GraphMap::<(String, bool), i8, petgraph::Undirected> {
    let mut caveNodeGraph = GraphMap::<(String, bool), i8, petgraph::Undirected>::new();
    let lines = shared::get_lines(filename);
    //let mut g = Graph::new_undirected();
    for line in lines {
        let nodes = line.split("-").map(|node_name| convert_to_cave_node(node_name.to_string())).collect::<Vec<(String, bool)>>();
        println!("linestart");
        for node in nodes {
            println!("{:?}", node);
        }
        caveNodeGraph.add_node(nodes[0]);
        caveNodeGraph.add_node(nodes[1]);
        println!("lineEnd");
        //add_nodes_and_edge(&nodes, &mut caveNodeGraph);

    }
        caveNodeGraph
}

fn add_nodes_and_edge(nodes: Vec<(String, bool)> , graph: &mut GraphMap::<(String, bool), i8, <(dyn petgraph::EdgeType + 'static) as Trait>::Undirected>) {
 
   if (!graph.contains_node(nodes[0])) {
       graph.add_node(nodes[0]);
   }
   if (!graph.contains_node(nodes[1])) {
       graph.add_node(nodes[1]);
   }
   //graph.add_edge()
}

fn convert_to_cave_node(node_name: String) -> (String, bool) {
    (node_name.clone(), !is_all_upper(&node_name))
}

fn is_all_upper(s: &String) -> bool {
    for c in s.chars() {
        if !c.is_ascii_uppercase() {
            return false;
        }
    }
    true
}
// petgraph is Graph<nodetype, edgetype (i8 for us bc we are not weighting them), type of edge (default is directed, or make undirected), index type (default is u32, determines max size of graph)>
// creates graph created in input2.txt for testing
fn initialize_test_graph() -> GraphMap<(&'static str, bool), i8, petgraph::Undirected> {
    let mut g = GraphMap::<(&str, bool), i8, petgraph::Undirected>::new();
    let start = g.add_node(("start", true));
    let A = g.add_node(("A", false));
    let b = g.add_node(("b", true));
    let c = g.add_node(("c", true));
    let d = g.add_node(("d", true));
    let end = g.add_node(("end", true));
    g.add_edge(start, A, 0);
    g.add_edge(start, b, 0);
    g.add_edge(A, c, 0);
    g.add_edge(A, b, 0);
    g.add_edge(b, d, 0);
    g.add_edge(A, end, 0);
    g.add_edge(b, end, 0);
    //println!("{:?}", Dot::new(&g));
    g
}

// creates a simple binary tree and prints a bfs and dfs
//              0
//           1     2 
//         3   4  5  6
//        7 8  9
fn get_graph_bearings() {
    let mut g = GraphMap::<(&str, bool), i8, petgraph::Undirected>::new();
    let zero = g.add_node(("0", true));
    let one = g.add_node(("1", false));
    let two = g.add_node(("2", true));
    let three = g.add_node(("3", true));
    let four = g.add_node(("4", true));
    let five = g.add_node(("5", true));
    let six = g.add_node(("6", true));
    let seven = g.add_node(("7", true));
    let eight = g.add_node(("8", true));
    let nine = g.add_node(("9", true));
    g.add_edge(zero, one, 0);
    g.add_edge(zero, two, 0);
    g.add_edge(one, three, 0);
    g.add_edge(one, four, 0);
    g.add_edge(two, five, 0);
    g.add_edge(two, six, 0);
    g.add_edge(three, seven, 0);
    g.add_edge(three, eight, 0);
    g.add_edge(four, nine, 0);
    let mut bfs = Bfs::new(&g, zero);
    println!("BFS");
    while let Some(v) = bfs.next(&g) {
        println!("{:?}", v);
    }
    println!("DFS");
    let mut dfs = Dfs::new(&g, zero);
    while let Some(v) = dfs.next(&g) {
        println!("{:?}", v);
    }
    println!("{:?}", Dot::new(&g));
}
