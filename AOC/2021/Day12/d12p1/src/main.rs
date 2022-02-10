use shared;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use petgraph::visit::Bfs;
use petgraph::visit::Dfs;
use petgraph::graphmap::GraphMap;

//finish parsing
//plan: find all paths using bfs kinda thing, the stop/remove the ones that contain two hits of a
//small cave, or something along those lines, check for many and get rid of them as small cave issues arise 
fn main() {
    //let g = initialize_test_graph();
    //println!("{:?}", Dot::new(&g));
    //get_graph_bearings(); 
    ////parse input into Vec<(String, CaveSize)>
    // fuck cavesize, just using bool, large = true, small = false
    let mut nodes = parse_input("./input2.txt");
    //convert nodes into Vec<(&str, bool)> because String can't implement Copy to be a
    //graphnode
    let mut convertedNodes = convert_nodes(&nodes);
    // pair the nodes up to create the graph from nodepairs, more readable to do this in a separate
    // step
    let nodePairs = group_nodes(&convertedNodes);
    print_nodepairs(&nodePairs);
    let caveGraph = create_graph(&nodePairs);
    println!("{:?}", Dot::new(&caveGraph));
}

fn group_nodes<'a>(nodes: &Vec<(&'a str , bool)>) -> Vec<Vec<(&'a str, bool)>> {
    let chunks = nodes.chunks(2);
    let mut res = Vec::new();
    for chunk in chunks {
        res.push(chunk.to_owned());
    }
    res
}

fn convert_nodes(nodes: &Vec<(String, bool)>) -> Vec<(&str, bool)> {
    let mut res = Vec::new();
    for node in nodes {
        res.push((&node.0[..], node.1)); 
    }
    res
}

//GraphMap::<(&str, bool), i8, petgraph::Undirected> 
//let mut caveNodeGraph = GraphMap::<(&str, bool), i8, petgraph::Undirected>::new();
fn create_graph<'a>(nodePairs: &Vec<Vec<(&'a str, bool)>>) -> GraphMap::<(&'a str, bool), i8, petgraph::Undirected> {
    let mut caveGraph = GraphMap::<(&str, bool), i8, petgraph::Undirected>::new();
    for nodePair in nodePairs {
        if (!caveGraph.contains_node(nodePair[0])) {
            caveGraph.add_node(nodePair[0]);
        }
        if (!caveGraph.contains_node(nodePair[1])) {
            caveGraph.add_node(nodePair[1]);
        }
        caveGraph.add_edge(nodePair[0], nodePair[1], 0);
    }
    println!("nodes: {}, edges: {}", caveGraph.node_count(), caveGraph.edge_count());
    caveGraph
}

fn print_nodepairs(nodePairs: &Vec<Vec<(&str, bool)>>) {
    println!("number of pairs: {}", nodePairs.len());
    for nodePair in nodePairs {
        print!("({}, ", nodePair[0].0);
        match nodePair[0].1 {
            false => print!("small)"),
            true => print!("large)")
        }
        print!(" -- ");
        print!("({}, ", nodePair[1].0);
        match nodePair[1].1 {
            false => println!("small)"),
            true => println!("large)")
        }
    }
}

//Todo: make the nodes able to be added to the graph, line is being borrowed???
fn parse_input(filename: &str) -> Vec<(String, bool)> {
    let mut result = Vec::new();
    let lines = shared::get_lines(filename);
    //let mut g = Graph::new_undirected();
    for line in lines {
        let nodePair = line.split("-").map(|node_name| (node_name.to_string(), find_cave_size(&node_name))).collect::<Vec<(String, bool)>>();
        for node in nodePair {
            result.push(node);
        }
        //let nodes_clone = nodes.clone();
        //shared::print_type_of(&nodes);
        //shared::print_type_of(&nodes);
        //println!("linestart");
        //for node in &nodes {
        //    println!("{:?}", node);
        //}
    }
    result
}

//fn add_nodes_and_edge(nodes: &mut Vec<(&str, bool)> , graph: &mut GraphMap::<(&str, bool), i8, petgraph::Undirected>) {
// 
//   if (!graph.contains_node(nodes[0])) {
//       graph.add_node(nodes[0]);
//   }
//   if (!graph.contains_node(nodes[1])) {
//       graph.add_node(nodes[1]);
//   }
//   //graph.add_edge()
//}

fn find_cave_size(caveName: &str) -> bool {
    if is_all_upper(caveName) {
        return true;
    }
    return false;
}

fn is_all_upper(s: &str) -> bool {
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
