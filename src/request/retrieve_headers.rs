use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveHeadersRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub cursor: Option<String>,
    pub limit: Option<String>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub service_id: String,
    pub value: Option<String>,
}
impl<'a> RetrieveHeadersRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<serde_json::Value>> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!("/services/{service_id}/headers", service_id = self.service_id),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.path {
            r = r.query("path", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.value {
            r = r.query("value", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
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
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_owned());
        self
    }
    pub fn value(mut self, value: &str) -> Self {
        self.value = Some(value.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveHeadersRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<serde_json::Value>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}