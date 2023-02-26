#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let response = client
        .list_services()
        // .created_after("your created after")
        // .created_before("your created before")
        // .cursor("your cursor")
        // .env("your env")
        // .limit("your limit")
        // .name("your name")
        // .owner_id("your owner id")
        // .region("your region")
        // .suspended("your suspended")
        // .type_("your type")
        // .updated_after("your updated after")
        // .updated_before("your updated before")
        .await
        .unwrap();
    println!("{:#?}", response);
}