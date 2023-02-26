#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .retrieve_headers(service_id)
        .cursor("your cursor")
        .limit("your limit")
        .name("your name")
        .path("your path")
        .value("your value")
        .await
        .unwrap();
    println!("{:#?}", response);
}