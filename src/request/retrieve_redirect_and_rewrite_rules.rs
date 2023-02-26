use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveRedirectAndRewriteRulesRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub cursor: Option<String>,
    pub destination: Option<String>,
    pub limit: Option<String>,
    pub service_id: String,
    pub source: Option<String>,
    pub type_: Option<String>,
}
impl<'a> RetrieveRedirectAndRewriteRulesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<serde_json::Value>> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!("/services/{service_id}/routes", service_id = self.service_id),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.destination {
            r = r.query("destination", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.source {
            r = r.query("source", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.query("type", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn destination(mut self, destination: &str) -> Self {
        self.destination = Some(destination.to_owned());
        self
    }
    pub fn limit(mut self, limit: &str) -> Self {
        self.limit = Some(limit.to_owned());
        self
    }
    pub fn source(mut self, source: &str) -> Self {
        self.source = Some(source.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveRedirectAndRewriteRulesRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<serde_json::Value>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}