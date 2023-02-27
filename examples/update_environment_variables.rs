#![allow(unused_imports)]
use render_api::RenderClient;
use render_api::model::*;
#[tokio::main]
async fn main() {
    let client = RenderClient::from_env();
    let body = vec![
        EnvVar { key : "your key".to_owned(), value : "your value".to_owned() }
    ];
    let service_id = "your service id";
    let response = client.update_environment_variables(body, service_id).await.unwrap();
    println!("{:#?}", response);
}