extern crate i3s;

use i3s::node::Node;
use std::time::Instant;

fn callback(node: &Node, level: &u8) -> bool {
    // println!("Node Index {}, Level: {}", node.index, level);
    true
}

fn main() {
    let path = r"";
    let scene_layer = i3s::SceneLayer::from_uri(&path).unwrap();
    let now = Instant::now();
    let nodes = scene_layer.nodes();
    nodes.traverse(callback);
    let elapsed = now.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    let now = Instant::now();
    nodes.traverse(callback);
    let elapsed = now.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Total nodes: {}", nodes.len());
}
