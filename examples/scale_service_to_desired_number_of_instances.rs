#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .scale_service_to_desired_number_of_instances(service_id)
        .num_instances(1.0)
        .await
        .unwrap();
    println!("{:#?}", response);
}