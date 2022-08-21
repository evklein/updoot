use std::collections::HashMap;

use updoot::models::hn_request_models::Comment;
use yew::prelude::*;

use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree, TreeBuilder};
use id_tree_layout::{Layouter, Visualize};
use updoot::{HackerNewsClient, LobstersClient};

pub mod components;

use components::hn_comment_component::{HNCommentComponent};

#[function_component(App)]
fn app() -> Html {
    let comment = Comment {
        by: "norvig".to_owned(),
        id: 2921983,
        kids: Vec::new(),
        parent: 2921506,
        text: "Aw shucks, guys ... you make me blush with your compliments.<p>Tell you what, Ill make a deal: I'll keep writing if you keep reading. K?".to_owned(),
        time: 1314211127,
        item_type: "comment".to_owned(),
    };

    html! {
        <>
        <div class="container">
            <h1 class="title">
                { "Hello, world!" }
            </h1>
            <p class="subtitle">
                { "My first website with " }
                <strong>{"Bulma"}</strong>{"!"}
            </p>
            <button class="button is-primary">
                {"Generate User Tree"}
            </button>
            <div class="columns">
                <HNCommentComponent comment={comment.clone()} />
                <HNCommentComponent comment={comment.clone()} />
            </div>
        </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}

async fn run_hn_100_test() {
    let hn_client = HackerNewsClient {};
    let top_100_hn_items = hn_client.get_latest_items(100).await;
    match top_100_hn_items {
        Ok(items) => println!("Success!\n\n {:?}", items),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// TODO: Move this code into its own mod
struct LobsterNode(String);

impl Visualize for LobsterNode {
    fn visualize(&self) -> std::string::String {
        self.0.to_string()
    }
    fn emphasize(&self) -> bool {
        false
    }
}

async fn do_old_lobsters_tree_build() {
    let lobsters_client = LobstersClient {};

    let users_with_children = lobsters_client.build_user_tree().await;
    let jcs_children = users_with_children.get("jcs").unwrap();

    let mut tree: Tree<LobsterNode> = TreeBuilder::new().with_node_capacity(100).build();
    let jcs_node_id: NodeId = tree
        .insert(Node::new(LobsterNode(String::from("jcs"))), AsRoot)
        .unwrap();

    build_tree(&mut tree, &users_with_children, &jcs_node_id, &jcs_children);

    write_tree_svg_to_file(&tree);
}

fn build_tree(
    tree: &mut Tree<LobsterNode>,
    tree_map: &HashMap<String, Vec<String>>,
    parent: &NodeId,
    children: &Vec<String>,
) {
    for child in children {
        let child_node_id = &tree
            .insert(
                Node::new(LobsterNode(String::from(child))),
                UnderNode(parent),
            )
            .unwrap();
        if tree_map.contains_key(child) {
            build_tree(tree, tree_map, child_node_id, tree_map.get(child).unwrap());
        }
    }
}

fn write_tree_svg_to_file(tree: &Tree<LobsterNode>) {
    Layouter::new(&tree)
        .with_file_path(std::path::Path::new("test.svg"))
        .write()
        .expect("Failed writing layout");
}
