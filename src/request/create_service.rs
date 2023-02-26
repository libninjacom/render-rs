use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateServiceRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub auto_deploy: Option<String>,
    pub branch: Option<String>,
    pub env_vars: Option<Vec<serde_json::Value>>,
    pub name: Option<String>,
    pub owner_id: Option<String>,
    pub repo: Option<String>,
    pub secret_files: Option<Vec<serde_json::Value>>,
    pub service_details: Option<serde_json::Value>,
    pub type_: Option<String>,
}
impl<'a> CreateServiceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self.http_client.client.post("/services");
        if let Some(ref unwrapped) = self.auto_deploy {
            r = r.json(json!({ "autoDeploy" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.branch {
            r = r.json(json!({ "branch" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.env_vars {
            r = r.json(json!({ "envVars" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.owner_id {
            r = r.json(json!({ "ownerId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.repo {
            r = r.json(json!({ "repo" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.secret_files {
            r = r.json(json!({ "secretFiles" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.service_details {
            r = r.json(json!({ "serviceDetails" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.json(json!({ "type" : unwrapped }));
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
    pub fn env_vars(mut self, env_vars: Vec<serde_json::Value>) -> Self {
        self.env_vars = Some(env_vars);
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn owner_id(mut self, owner_id: &str) -> Self {
        self.owner_id = Some(owner_id.to_owned());
        self
    }
    pub fn repo(mut self, repo: &str) -> Self {
        self.repo = Some(repo.to_owned());
        self
    }
    pub fn secret_files(mut self, secret_files: Vec<serde_json::Value>) -> Self {
        self.secret_files = Some(secret_files);
        self
    }
    pub fn service_details(mut self, service_details: serde_json::Value) -> Self {
        self.service_details = Some(service_details);
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateServiceRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}