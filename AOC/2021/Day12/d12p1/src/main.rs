use shared;
use std::collections::HashMap;

//finish parsing
//plan: find all paths using bfs kinda thing, the stop/remove the ones that contain two hits of a
//small cave, or something along those lines, check for many and get rid of them as small cave issues arise 
fn main() {
    // maybe hashmap with lists?? HashMap<(&str, bool), Vec<(&str, bool)>>
    //let mut cave_graph: Vec<Vec<(&str, bool)>> = parse_input("./input2.txt");
    let nodes = parse_input("./input2.txt");
    let converted_nodes = convert_nodes(&nodes); 
    let grouped_nodes = group_nodes(&converted_nodes);
    let cave_graph = create_graph(&grouped_nodes);
    println!("size of create_graph: {}", cave_graph.len());

    println!("get(&(\"start\", true)) of result from create_graph gives: [(\"{}\", {})]", cave_graph.get(&("start", true)).unwrap()[0].0, cave_graph.get(&("start", true)).unwrap()[0].1);

    let mut x = HashMap::new();
    let mut v = Vec::new();
    v.push(("a", true));
    x.insert(("start", true), v);
    println!("{}", x.contains_key(&("start", true)));
    let r = x.get(&("start", true)).unwrap()[0].0;
    println!("{}", r);
}

//return adjacency list of graph HashMap<(&str, bool), Vec<(&str, bool)>>
fn create_graph<'a>(node_pairs: &Vec<Vec<(&'a str, bool)>>) -> HashMap<(&'a str, bool), Vec<(&'a str, bool)>> {
    let mut result = HashMap::new();
    result.insert(("start", true), vec!(("A", false)));
    result
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

fn parse_input(filename: &str) -> Vec<(String, bool)> {
    // create nodes ()
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

//return is_small(cave)
fn find_cave_size(caveName: &str) -> bool {
    if is_all_upper(caveName) {
        return false;
    }
    return true;
}

fn is_all_upper(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_ascii_uppercase() {
            return false;
        }
    }
    true
}
