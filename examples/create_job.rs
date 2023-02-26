#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .create_job(service_id)
        .plan_id("your plan id")
        .start_command("your start command")
        .await
        .unwrap();
    println!("{:#?}", response);
}