#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let response = client
        .create_service()
        .auto_deploy("your auto deploy")
        .branch("your branch")
        .env_vars(vec![serde_json::json!({})])
        .name("your name")
        .owner_id("your owner id")
        .repo("your repo")
        .secret_files(vec![serde_json::json!({})])
        .service_details(serde_json::json!({}))
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}