use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateJobRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub plan_id: Option<String>,
    pub service_id: String,
    pub start_command: Option<String>,
}
impl<'a> CreateJobRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/services/{service_id}/jobs", service_id = self.service_id));
        if let Some(ref unwrapped) = self.plan_id {
            r = r.json(json!({ "planId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_command {
            r = r.json(json!({ "startCommand" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn plan_id(mut self, plan_id: &str) -> Self {
        self.plan_id = Some(plan_id.to_owned());
        self
    }
    pub fn start_command(mut self, start_command: &str) -> Self {
        self.start_command = Some(start_command.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for CreateJobRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}