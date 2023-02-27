#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let response = client.list_env_groups().await.unwrap();
    println!("{:#?}", response);
}