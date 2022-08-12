use std::collections::HashMap;

use updoot::LobstersClient;
use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree, TreeBuilder};
use id_tree_layout::{Layouter, Visualize};

struct MyNodeData(String);

// You need to implement id_tree_layout::Visualize for your nodes data type.
// This way you provide basic formatting information.
impl Visualize for MyNodeData {
    fn visualize(&self) -> std::string::String {
        // We simply convert the i32 value to string here.
        self.0.to_string()
    }
    fn emphasize(&self) -> bool {
        // This simply emphasizes only the leaf nodes.
        // It only works for this example.
        false
    }
}

//https://github.com/jsinger67/id-tree-layout
// https://github.com/iwburns/id-tree
#[tokio::main]
async fn main() {
    let lobsters_client = LobstersClient{};

    let users_with_children = lobsters_client.build_user_tree().await;
    let jcs_children = users_with_children.get("jcs").unwrap();

    let mut tree: Tree<MyNodeData> = TreeBuilder::new().with_node_capacity(100).build();
    let jcs_node_id: NodeId = tree.insert(Node::new(MyNodeData(String::from("jcs"))), AsRoot).unwrap();

    build_tree(&mut tree, &users_with_children, &jcs_node_id, &jcs_children);

    // Here comes the visualization part.
    Layouter::new(&tree)
        .with_file_path(std::path::Path::new("test.svg"))
        .write()
        .expect("Failed writing layout")
}

fn build_tree(tree: &mut Tree<MyNodeData>, tree_map: &HashMap<String, Vec<String>>, parent: &NodeId, children: &Vec<String>) {
    for child in children {
        let child_node_id = &tree.insert(
                                            Node::new(MyNodeData(String::from(child))),
                                            UnderNode(parent))
                                        .unwrap();
        if tree_map.contains_key(child) {
            build_tree(tree, tree_map, child_node_id, tree_map.get(child).unwrap());
        }
    }
}