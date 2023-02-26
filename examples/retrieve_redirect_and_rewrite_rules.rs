#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .retrieve_redirect_and_rewrite_rules(service_id)
        .cursor("your cursor")
        .destination("your destination")
        .limit("your limit")
        .source("your source")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}