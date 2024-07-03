use serde_json::json;
use crate::model::*;
use crate::RenderClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ScaleServiceToDesiredNumberOfInstancesRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub num_instances: Option<f64>,
    pub service_id: String,
}
impl<'a> ScaleServiceToDesiredNumberOfInstancesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!("/services/{service_id}/scale", service_id = self.service_id),
            );
        if let Some(ref unwrapped) = self.num_instances {
            r = r.json(json!({ "numInstances" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn num_instances(mut self, num_instances: f64) -> Self {
        self.num_instances = Some(num_instances);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for ScaleServiceToDesiredNumberOfInstancesRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}