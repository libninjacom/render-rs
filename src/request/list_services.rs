use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListServicesRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub cursor: Option<String>,
    pub env: Option<String>,
    pub limit: Option<String>,
    pub name: Option<String>,
    pub owner_id: Option<String>,
    pub region: Option<String>,
    pub suspended: Option<String>,
    pub type_: Option<String>,
    pub updated_after: Option<String>,
    pub updated_before: Option<String>,
}
impl<'a> ListServicesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<ServiceCursor>> {
        let mut r = self.http_client.client.get("/services");
        if let Some(ref unwrapped) = self.created_after {
            r = r.query("createdAfter", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created_before {
            r = r.query("createdBefore", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.env {
            r = r.query("env", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.owner_id {
            r = r.query("ownerId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.region {
            r = r.query("region", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.suspended {
            r = r.query("suspended", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.query("type", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.updated_after {
            r = r.query("updatedAfter", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.updated_before {
            r = r.query("updatedBefore", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn created_after(mut self, created_after: &str) -> Self {
        self.created_after = Some(created_after.to_owned());
        self
    }
    pub fn created_before(mut self, created_before: &str) -> Self {
        self.created_before = Some(created_before.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn env(mut self, env: &str) -> Self {
        self.env = Some(env.to_owned());
        self
    }
    pub fn limit(mut self, limit: &str) -> Self {
        self.limit = Some(limit.to_owned());
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
    pub fn region(mut self, region: &str) -> Self {
        self.region = Some(region.to_owned());
        self
    }
    pub fn suspended(mut self, suspended: &str) -> Self {
        self.suspended = Some(suspended.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
    pub fn updated_after(mut self, updated_after: &str) -> Self {
        self.updated_after = Some(updated_after.to_owned());
        self
    }
    pub fn updated_before(mut self, updated_before: &str) -> Self {
        self.updated_before = Some(updated_before.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListServicesRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<ServiceCursor>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}