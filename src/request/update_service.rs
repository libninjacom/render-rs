use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateServiceRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub auto_deploy: Option<String>,
    pub branch: Option<String>,
    pub name: Option<String>,
    pub service_details: Option<serde_json::Value>,
    pub service_id: String,
}
impl<'a> UpdateServiceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Service> {
        let mut r = self
            .http_client
            .client
            .patch(&format!("/services/{service_id}", service_id = self.service_id));
        if let Some(ref unwrapped) = self.auto_deploy {
            r = r.json(json!({ "autoDeploy" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.branch {
            r = r.json(json!({ "branch" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.service_details {
            r = r.json(json!({ "serviceDetails" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn auto_deploy(mut self, auto_deploy: &str) -> Self {
        self.auto_deploy = Some(auto_deploy.to_owned());
        self
    }
    pub fn branch(mut self, branch: &str) -> Self {
        self.branch = Some(branch.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn service_details(mut self, service_details: serde_json::Value) -> Self {
        self.service_details = Some(service_details);
        self
    }
}
impl<'a> ::std::future::IntoFuture for UpdateServiceRequest<'a> {
    type Output = httpclient::InMemoryResult<Service>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}