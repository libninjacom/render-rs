use serde_json::json;
use crate::model::*;
use crate::RenderClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListJobsRequest<'a> {
    pub(crate) http_client: &'a RenderClient,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub cursor: Option<String>,
    pub finished_after: Option<String>,
    pub finished_before: Option<String>,
    pub limit: Option<String>,
    pub service_id: String,
    pub started_after: Option<String>,
    pub started_before: Option<String>,
    pub status: Option<String>,
}
impl<'a> ListJobsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Vec<serde_json::Value>> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/services/{service_id}/jobs", service_id = self.service_id));
        if let Some(ref unwrapped) = self.created_after {
            r = r.query("createdAfter", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created_before {
            r = r.query("createdBefore", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.finished_after {
            r = r.query("finishedAfter", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.finished_before {
            r = r.query("finishedBefore", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.started_after {
            r = r.query("startedAfter", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.started_before {
            r = r.query("startedBefore", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn created_after(mut self, created_after: &str) -> Self {
        self.created_after = Some(created_after.to_owned());
        self
    }
    pub fn created_before(mut self, created_before: &str) -> Self {
        self.created_before = Some(created_before.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn finished_after(mut self, finished_after: &str) -> Self {
        self.finished_after = Some(finished_after.to_owned());
        self
    }
    pub fn finished_before(mut self, finished_before: &str) -> Self {
        self.finished_before = Some(finished_before.to_owned());
        self
    }
    pub fn limit(mut self, limit: &str) -> Self {
        self.limit = Some(limit.to_owned());
        self
    }
    pub fn started_after(mut self, started_after: &str) -> Self {
        self.started_after = Some(started_after.to_owned());
        self
    }
    pub fn started_before(mut self, started_before: &str) -> Self {
        self.started_before = Some(started_before.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListJobsRequest<'a> {
    type Output = httpclient::InMemoryResult<Vec<serde_json::Value>>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}