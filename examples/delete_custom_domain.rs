#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let custom_domain_id_or_name = "your custom domain id or name";
    let service_id = "your service id";
    let response = client
        .delete_custom_domain(custom_domain_id_or_name, service_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}