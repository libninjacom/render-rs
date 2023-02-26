#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let owner_id = "your owner id";
    let response = client.retrieve_user_or_team(owner_id).await.unwrap();
    println!("{:#?}", response);
}