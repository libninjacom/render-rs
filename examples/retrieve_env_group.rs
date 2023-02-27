#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let env_group_id = "your env group id";
    let response = client.retrieve_env_group(env_group_id).await.unwrap();
    println!("{:#?}", response);
}