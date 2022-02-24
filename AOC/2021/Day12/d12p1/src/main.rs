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
    println!();
    print_graph(&cave_graph);
    let path = find_a_path(&cave_graph);
    print_path(&path); 
}

fn print_path(path: &Vec<(&str, bool)>) {
    print!("PATH: ");
    for i in 0..path.len() {
        print!("({}, {})", path[i].0, path[i].1);
        if (i != path.len() - 1) {
            print!(" -- ");
        }
    }
    println!();
}

fn find_a_path<'a>(g: &HashMap<(&'a str, bool), Vec<(&'a str, bool)>>) -> Vec<(&'a str, bool)> {
    let start = ("start", true);
    let mut curr_node: (&str, bool) = start;
    //use this as a stack fr dfs/path finding
    let mut path = vec!(start);
    loop {
        let connected_nodes = g.get(&curr_node).unwrap();
        if connected_nodes.contains(&("end", true)) {
            path.push(("end", true));
            break;
        }
        //always looks at first node, causes infinite loops most of the time, switch to work with
        //stack(vec) and choose curr_node based on that
        //should be able to all simple paths and only mark node as visited if its a small cave https://www.baeldung.com/cs/simple-paths-between-two-vertices
        curr_node = connected_nodes[0];
        path.push(curr_node);
    }
    path

}


fn print_graph(g: &HashMap<(&str, bool), Vec<(&str, bool)>>) {
    for (key, nodes) in &*g {
        print!("({}, {}) -- [", key.0, key.1);
        for i in 0..nodes.len() {
            print!("({}, {})", nodes[i].0, nodes[i].1);
            if (i != nodes.len() - 1) {
                print!(", ");
            }
        }
        println!("]");
    }
}

//return adjacency list of graph HashMap<(&str, bool), Vec<(&str, bool)>>
fn create_graph<'a>(node_pairs: &Vec<Vec<(&'a str, bool)>>) -> HashMap<(&'a str, bool), Vec<(&'a str, bool)>> {
    let mut result: HashMap<(&'a str, bool), Vec<(&'a str, bool)>> = HashMap::new();
    for node_pair in node_pairs {
        // add nodes, and edge for each pair, hopefully the value for each key can be kutable
        //check if the nodes exist already, then connect them once they do
        if (!result.contains_key(&node_pair[0])) {
            //insert
            let v = vec!(node_pair[1]);
            result.insert(node_pair[0], v);
        } else {
            //append adjacency list
            if let Some(x) = result.get_mut(&node_pair[0]) {
                x.push(node_pair[1]);
            }
        }

        if (!result.contains_key(&node_pair[1])) {
            //add it
            let v = vec!(node_pair[0]);
            result.insert(node_pair[1], v);
        } else {
            //append adjacency list
            if let Some(x) = result.get_mut(&node_pair[1]) {
                x.push(node_pair[0]);
            }
        }
    }
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
