use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListDeploysRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub cursor: Option<String>,
    pub end_time: Option<String>,
    pub limit: Option<i64>,
    pub service_id: String,
    pub start_time: Option<String>,
}
impl<'a> ListDeploysRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<DeployCursor>> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!("/services/{service_id}/deploys", service_id = self.service_id),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.query("endTime", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_time {
            r = r.query("startTime", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn end_time(mut self, end_time: &str) -> Self {
        self.end_time = Some(end_time.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn start_time(mut self, start_time: &str) -> Self {
        self.start_time = Some(start_time.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListDeploysRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<DeployCursor>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}