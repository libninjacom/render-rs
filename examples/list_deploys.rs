#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .list_deploys(service_id)
        .cursor("your cursor")
        .end_time("your end time")
        .limit("your limit")
        .start_time("your start time")
        .await
        .unwrap();
    println!("{:#?}", response);
}