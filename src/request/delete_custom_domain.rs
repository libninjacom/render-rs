use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteCustomDomainRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub custom_domain_id_or_name: String,
    pub service_id: String,
}
impl<'a> DeleteCustomDomainRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/services/{service_id}/custom-domains/{custom_domain_id_or_name}",
                    custom_domain_id_or_name = self.custom_domain_id_or_name, service_id
                    = self.service_id
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for DeleteCustomDomainRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}