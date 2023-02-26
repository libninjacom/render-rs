#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .list_custom_domains(service_id)
        .created_after("your created after")
        .created_before("your created before")
        .cursor("your cursor")
        .domain_type("your domain type")
        .limit("your limit")
        .name("your name")
        .verification_status("your verification status")
        .await
        .unwrap();
    println!("{:#?}", response);
}