#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client.delete_service(service_id).await.unwrap();
    println!("{:#?}", response);
}