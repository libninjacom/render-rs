use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListCustomDomainsRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub cursor: Option<String>,
    pub domain_type: Option<String>,
    pub limit: Option<String>,
    pub name: Option<String>,
    pub service_id: String,
    pub verification_status: Option<String>,
}
impl<'a> ListCustomDomainsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<serde_json::Value>> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/services/{service_id}/custom-domains", service_id = self.service_id
                ),
            );
        if let Some(ref unwrapped) = self.created_after {
            r = r.query("createdAfter", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created_before {
            r = r.query("createdBefore", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.domain_type {
            r = r.query("domainType", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.verification_status {
            r = r.query("verificationStatus", &unwrapped.to_string());
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
    pub fn domain_type(mut self, domain_type: &str) -> Self {
        self.domain_type = Some(domain_type.to_owned());
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
    pub fn verification_status(mut self, verification_status: &str) -> Self {
        self.verification_status = Some(verification_status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListCustomDomainsRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<serde_json::Value>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}