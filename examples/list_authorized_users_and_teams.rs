#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let response = client
        .list_authorized_users_and_teams()
        .cursor("your cursor")
        .email("your email")
        .limit("your limit")
        .name("your name")
        .await
        .unwrap();
    println!("{:#?}", response);
}