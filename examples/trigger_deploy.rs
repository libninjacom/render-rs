#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .trigger_deploy(service_id)
        .clear_cache("your clear cache")
        .await
        .unwrap();
    println!("{:#?}", response);
}