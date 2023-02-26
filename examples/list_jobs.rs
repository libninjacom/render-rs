#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let service_id = "your service id";
    let response = client
        .list_jobs(service_id)
        .created_after("your created after")
        .created_before("your created before")
        .cursor("your cursor")
        .finished_after("your finished after")
        .finished_before("your finished before")
        .limit("your limit")
        .started_after("your started after")
        .started_before("your started before")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}