use serde_json::json;
use crate::model::*;
use crate::RenderClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateEnvironmentVariablesRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub body: Vec<EnvVar>,
    pub service_id: String,
}
impl<'a> UpdateEnvironmentVariablesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<EnvVarCursor>> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!("/services/{service_id}/env-vars", service_id = self.service_id),
            );
        r = r.json(json!({ "body" : self.body }));
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for UpdateEnvironmentVariablesRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<EnvVarCursor>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}