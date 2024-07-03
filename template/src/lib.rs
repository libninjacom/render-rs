//! [`RenderClient`](struct.RenderClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]

pub mod model;
pub mod request;
use ::serde::{Serialize, Deserialize};

use crate::model::*;

/// All this commented out code doesn't work because Render uses different tokens
/// for the GraphQL authentication (used by Render.com) and for the Rest API.
/// Hopefully Render.com implements REST API endpoints for creating & updating env-groups soon...
// #[derive(Clone, Debug, Serialize)]
// pub struct GqlRequest {
//     pub operation_name: &'static str,
//     pub variables: serde_json::Value,
//     pub query: &'static str,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct EnvGroup {
//     pub id: String,
//     pub name: String,
//     #[serde(rename = "ownerId")]
//     pub owner_id: String,
//     #[serde(rename = "created_at")]
//     pub created_at: String,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: String,
//     #[serde(rename = "envVars")]
//     pub env_vars: Vec<GqlEnvVar>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct GqlEnvVar {
//     pub id: String,
//     pub key: String,
//     pub value: String,
//     #[serde(rename = "isFile")]
//     pub is_file: bool,
// }

// impl From<(&str, &str)> for GqlEnvVar {
//     fn from((key, value): (&str, &str)) -> Self {
//         Self {
//             id: "".to_string(),
//             key: key.to_string(),
//             value: value.to_string(),
//             is_file: false,
//         }
//     }
// }

// const ENV_GROUPS_FOR_OWNER: &str = r#"
// query envGroupsForOwner($ownerId: String!) {
//   envGroupsForOwner(ownerId: $ownerId) {
//     ...envGroupFields
//     __typename
//   }
// }
//
// fragment envGroupFields on EnvGroup {
//   id
//   name
//   ownerId
//   created_at
//   updatedAt
//   envVars {
//     ...envVarFields
//     __typename
//   }
//   __typename
// }
//
// fragment envVarFields on EnvVar {
//   id
//   isFile
//   key
//   value
//   __typename
// }"#;
//
//
// const CREATE_ENV_GROUP: &str = r#"
// mutation createEnvGroup($name: String!, $envVarInputs: [EnvVarInput!]!, $ownerId: String!) {
//   createEnvGroup(name: $name, envVarInputs: $envVarInputs, ownerId: $ownerId) {
//     ...envGroupFields
//     __typename
//   }
// }
//
// fragment envGroupFields on EnvGroup {
//   id
//   name
//   ownerId
//   createdAt
//   updatedAt
//   envVars {
//     ...envVarFields
//     __typename
//   }
//   __typename
// }
//
// fragment envVarFields on EnvVar {
//   id
//   isFile
//   key
//   value
//   __typename
// }"#;
//
// const UPDATE_ENV_GROUP: &str = r#"
// mutation updateEnvGroupEnvVars($id: String!, $envVarInputs: [EnvVarInput!]!) {
//   updateEnvGroupEnvVars(id: $id, envVarInputs: $envVarInputs) {
//     ...envVarFields
//     __typename
//   }
// }
//
// fragment envVarFields on EnvVar {
//   id
//   isFile
//   key
//   value
//   __typename
// }"#;
//
// const GET_ENV_GROUP: &str = r#"
// query envGroup($id: String!) {
//   envGroup(id: $id) {
//     ...envGroupFields
//     __typename
//   }
// }
//
// fragment envGroupFields on EnvGroup {
//   id
//   name
//   ownerId
//   createdAt
//   updatedAt
//   envVars {
//     ...envVarFields
//     __typename
//   }
//   __typename
// }
//
// fragment envVarFields on EnvVar {
//   id
//   isFile
//   key
//   value
//   __typename
// }"#;

// impl RenderClient {
    // pub async fn get_env_groups(&self, owner_id: &str) -> httpclient::Result<Vec<EnvGroup>> {
    //     let url = "https://api.render.com/graphql";
    //     let gql = GqlRequest {
    //         operation_name: "envGroupsForOwner",
    //         variables: serde_json::json!({
    //             "ownerId": owner_id,
    //         }),
    //         query: ENV_GROUPS_FOR_OWNER,
    //     };
    //     let req = self.client
    //         .post(url)
    //         .json(&gql);
    //     let res = self.authenticate(req)
    //
    //         .await?;
    //     let mut res: serde_json::Value = res.json().map_err(Into::into)?;
    //     let groups: Vec<EnvGroup> = serde_json::from_value(res["data"]["envGroupsForOwner"].take())?;
    //     Ok(groups)
    // }
    //
    // pub async fn create_env_var_group(&self, owner_id: &str, name: &str, vars: &[GqlEnvVar]) -> httpclient::Result<EnvGroup> {
    //     let url = "https://api.render.com/graphql";
    //     let gql = GqlRequest {
    //         operation_name: "createEnvGroup",
    //         variables: serde_json::json!({
    //             "ownerId": owner_id,
    //             "name": name,
    //             "envVarInputs": vars,
    //         }),
    //         query: CREATE_ENV_GROUP,
    //     };
    //     let req = self.client
    //         .post(url)
    //         .json(&gql);
    //     let res = self.authenticate(req)
    //
    //         .await?;
    //     let mut res: serde_json::Value = res.json().map_err(Into::into)?;
    //     let group: EnvGroup = serde_json::from_value(res["data"]["createEnvGroup"].take())?;
    //     Ok(group)
    // }
    //
    // pub async fn update_env_var_group(&self, group_id: &str, vars: &[GqlEnvVar]) -> httpclient::Result<EnvGroup> {
    //     let url = "https://api.render.com/graphql";
    //     let gql = GqlRequest {
    //         operation_name: "updateEnvGroup",
    //         variables: serde_json::json!({
    //             "envGroupId": group_id,
    //             "envVarInputs": vars,
    //         }),
    //         query: UPDATE_ENV_GROUP,
    //     };
    //     let req = self.client
    //         .post(url)
    //         .json(&gql);
    //     let res = self.authenticate(req)
    //
    //         .await?;
    //     let mut res: serde_json::Value = res.json().map_err(Into::into)?;
    //     let group: EnvGroup = serde_json::from_value(res["data"]["updateEnvGroup"].take())?;
    //     Ok(group)
    // }

    // pub async fn get_env_group(&self, group_id: &str) -> httpclient::Result<EnvGroup> {
    //     let url = "https://api.render.com/graphql";
    //     let gql = GqlRequest {
    //         operation_name: "envGroup",
    //         variables: serde_json::json!({
    //             "id": group_id,
    //         }),
    //         query: GET_ENV_GROUP,
    //     };
    //     let req = self.client
    //         .post(url)
    //         .json(&gql);
    //     let res = self.authenticate(req)
    //
    //         .await?;
    //     let mut res: serde_json::Value = res.json().map_err(Into::into)?;
    //     let group: EnvGroup = serde_json::from_value(res["data"]["envGroup"].take())?;
    //     Ok(group)
    // }
// }