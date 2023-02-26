#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .update_service(service_id)
        .auto_deploy("your auto deploy")
        .branch("your branch")
        .name("your name")
        .service_details(serde_json::json!({}))
        .await
        .unwrap();
    println!("{:#?}", response);
}