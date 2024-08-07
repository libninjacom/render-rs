use serde_json::json;
use crate::model::*;
use crate::RenderClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListEnvGroupsRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
}
impl<'a> ListEnvGroupsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<EnvGroupCursor>> {
        let mut r = self.http_client.client.get("/env-groups");
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for ListEnvGroupsRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<EnvGroupCursor>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}