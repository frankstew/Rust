use shared;
use petgraph::Graph;

fn main() {
    let mut g = Graph::<&str, &str>::new();
    let one = g.add_node("one");
    let two = g.add_node("two");
    g.extend_with_edged(&[(one, two)]);
    println!("{}", serde_json::to_string_pretty(&graph).unwrap());
}
