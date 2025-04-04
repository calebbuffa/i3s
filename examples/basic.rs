extern crate i3s;

use i3s::node::Node;
use std::sync::Arc;

fn callback(node: &Arc<Node>, level: &u8) -> bool {
    let msg = format!("Node ID: {}, Level: {}", node.index, level);
    println!("{}", msg);
    true
}

fn main() {
    let path = r"";
    let scene_layer = i3s::SceneLayer::from_uri(&path).unwrap();
    let mut nodes = scene_layer.nodes();
    nodes.traverse(callback);
    println!("Total nodes: {}", nodes.len());
}
