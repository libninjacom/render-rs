use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TriggerDeployRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub clear_cache: Option<String>,
    pub service_id: String,
}
impl<'a> TriggerDeployRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Deploy> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!("/services/{service_id}/deploys", service_id = self.service_id),
            );
        if let Some(ref unwrapped) = self.clear_cache {
            r = r.json(json!({ "clearCache" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn clear_cache(mut self, clear_cache: &str) -> Self {
        self.clear_cache = Some(clear_cache.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for TriggerDeployRequest<'a> {
    type Output = httpclient::InMemoryResult<Deploy>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}