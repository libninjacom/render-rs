use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct RetrieveEnvGroupRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub env_group_id: String,
}
impl<'a> RetrieveEnvGroupRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<EnvGroup> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!("/env-groups/{env_group_id}", env_group_id = self.env_group_id),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for RetrieveEnvGroupRequest<'a> {
    type Output = httpclient::InMemoryResult<EnvGroup>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}