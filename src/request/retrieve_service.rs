use serde_json::json;
use crate::model::*;
use crate::RenderClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveServiceRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub service_id: String,
}
impl<'a> RetrieveServiceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Service> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/services/{service_id}", service_id = self.service_id));
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveServiceRequest<'a> {
    type Output = httpclient::InMemoryResult<Service>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}