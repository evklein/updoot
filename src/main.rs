use updoot::LobstersClient;

//https://github.com/jsinger67/id-tree-layout
// https://github.com/iwburns/id-tree
#[tokio::main]
async fn main() {
    let lobsters_client = LobstersClient{};

    let submissions = lobsters_client.build_user_tree().await;
    println!("{:?}", submissions);
    println!("{:?}", submissions.len());
}
