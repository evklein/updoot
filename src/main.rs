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

    let submissions = lobsters_client.build_user_tree().await;
    println!("{:?}", submissions);
    println!("{:?}", submissions.len());

    let mut tree: Tree<MyNodeData> = TreeBuilder::new().with_node_capacity(submissions.len()).build();
    let root_id: NodeId = tree.insert(Node::new(MyNodeData(String::from("jcs"))), AsRoot).unwrap();
    let child_id: NodeId = tree.insert(Node::new(MyNodeData(String::from("2"))), UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(MyNodeData(String::from("3"))), UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(MyNodeData(String::from("4"))), UnderNode(&child_id)).unwrap();
    tree.insert(Node::new(MyNodeData(String::from("5"))), UnderNode(&child_id)).unwrap();

    // Here comes the visualization part.
    Layouter::new(&tree)
        .with_file_path(std::path::Path::new("test.svg"))
        .write()
        .expect("Failed writing layout")
}
