use std::collections::HashMap;

use updoot::models::hn_request_models::Comment;
use yew::prelude::*;
use yew_router::prelude::*;

use id_tree::InsertBehavior::{AsRoot, UnderNode};
use id_tree::{Node, NodeId, Tree, TreeBuilder};
use id_tree_layout::{Layouter, Visualize};
use updoot::{HackerNewsClient, LobstersClient};

pub mod components;
pub mod routes;

use components::navbar::NavBar;
use components::game_component::GameComponent;
use routes::Route;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <NavBar />
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
        </>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Game => html! {
            <GameComponent />
        },
        Route::Tree => html! {
            <></>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
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
