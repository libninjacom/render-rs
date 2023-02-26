use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct AddCustomDomainRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub name: Option<String>,
    pub service_id: String,
}
impl<'a> AddCustomDomainRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<serde_json::Value>> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/services/{service_id}/custom-domains", service_id = self.service_id
                ),
            );
        if let Some(ref unwrapped) = self.name {
            r = r.json(json!({ "name" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for AddCustomDomainRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<serde_json::Value>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}