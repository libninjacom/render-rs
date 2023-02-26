#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let job_id = "your job id";
    let service_id = "your service id";
    let response = client.retrieve_job(job_id, service_id).await.unwrap();
    println!("{:#?}", response);
}