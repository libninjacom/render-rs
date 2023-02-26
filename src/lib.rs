//! [`RenderClient`](struct.RenderClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]

pub mod model;
pub mod request;

use crate::model::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GqlRequest {
    pub operation_name: &'static str,
    pub variables: serde_json::Value,
    pub query: &'static str,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnvGroup {
    pub id: String,
    pub name: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "envVars")]
    pub env_vars: Vec<GqlEnvVar>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GqlEnvVar {
    pub id: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "isFile")]
    pub is_file: bool,
}

impl From<(&str, &str)> for GqlEnvVar {
    fn from((key, value): (&str, &str)) -> Self {
        Self {
            id: "".to_string(),
            key: key.to_string(),
            value: value.to_string(),
            is_file: false,
        }
    }
}

const ENV_GROUPS_FOR_OWNER: &str = r#"
query envGroupsForOwner($ownerId: String!) {
  envGroupsForOwner(ownerId: $ownerId) {
    ...envGroupFields
    __typename
  }
}

fragment envGroupFields on EnvGroup {
  id
  name
  ownerId
  created_at
  updatedAt
  envVars {
    ...envVarFields
    __typename
  }
  __typename
}

fragment envVarFields on EnvVar {
  id
  isFile
  key
  value
  __typename
}"#;


const CREATE_ENV_GROUP: &str = r#"
mutation createEnvGroup($name: String!, $envVarInputs: [EnvVarInput!]!, $ownerId: String!) {
  createEnvGroup(name: $name, envVarInputs: $envVarInputs, ownerId: $ownerId) {
    ...envGroupFields
    __typename
  }
}

fragment envGroupFields on EnvGroup {
  id
  name
  ownerId
  createdAt
  updatedAt
  envVars {
    ...envVarFields
    __typename
  }
  __typename
}

fragment envVarFields on EnvVar {
  id
  isFile
  key
  value
  __typename
}"#;

const UPDATE_ENV_GROUP: &str = r#"
mutation updateEnvGroupEnvVars($id: String!, $envVarInputs: [EnvVarInput!]!) {
  updateEnvGroupEnvVars(id: $id, envVarInputs: $envVarInputs) {
    ...envVarFields
    __typename
  }
}

fragment envVarFields on EnvVar {
  id
  isFile
  key
  value
  __typename
}"#;


impl RenderClient {
    pub async fn get_env_groups(&self, owner_id: &str) -> httpclient::Result<Vec<EnvGroup>> {
        let url = "https://api.render.com/graphql";
        let gql = GqlRequest {
            operation_name: "envGroupsForOwner",
            variables: serde_json::json!({
                "ownerId": owner_id,
            }),
            query: ENV_GROUPS_FOR_OWNER,
        };
        let req = self.client
            .post(url)
            .json(&gql);
        let res = self.authenticate(req)
            .send_awaiting_body()
            .await?;
        let mut res: serde_json::Value = res.json()?;
        let groups: Vec<EnvGroup> = serde_json::from_value(res["data"]["envGroupsForOwner"].take())?;
        Ok(groups)
    }

    pub async fn create_env_var_group(&self, owner_id: &str, name: &str, vars: &[GqlEnvVar]) -> httpclient::Result<EnvGroup> {
        let url = "https://api.render.com/graphql";
        let gql = GqlRequest {
            operation_name: "createEnvGroup",
            variables: serde_json::json!({
                "ownerId": owner_id,
                "name": name,
                "envVarInputs": vars,
            }),
            query: CREATE_ENV_GROUP,
        };
        let req = self.client
            .post(url)
            .json(&gql);
        let res = self.authenticate(req)
            .send_awaiting_body()
            .await?;
        let mut res: serde_json::Value = res.json()?;
        let group: EnvGroup = serde_json::from_value(res["data"]["createEnvGroup"].take())?;
        Ok(group)
    }

    pub async fn update_env_var_group(&self, group_id: &str, vars: &[GqlEnvVar]) -> httpclient::Result<EnvGroup> {
        let url = "https://api.render.com/graphql";
        let gql = GqlRequest {
            operation_name: "updateEnvGroup",
            variables: serde_json::json!({
                "envGroupId": group_id,
                "envVarInputs": vars,
            }),
            query: UPDATE_ENV_GROUP,
        };
        let req = self.client
            .post(url)
            .json(&gql);
        let res = self.authenticate(req)
            .send_awaiting_body()
            .await?;
        let mut res: serde_json::Value = res.json()?;
        let group: EnvGroup = serde_json::from_value(res["data"]["updateEnvGroup"].take())?;
        Ok(group)
    }
}
pub struct RenderClient {
    pub client: httpclient::Client,
    authentication: RenderAuthentication,
}
impl RenderClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("https://api.render.com/v1"),
            authentication: RenderAuthentication::from_env(),
        }
    }
}
impl RenderClient {
    pub fn new(url: &str, authentication: RenderAuthentication) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: RenderAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            RenderAuthentication::ApiKeyAuth { api_key_auth } => {
                r = r.bearer_auth(api_key_auth);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**List authorized users and teams

[https://api-docs.render.com/reference/get-owners](https://api-docs.render.com/reference/get-owners)*/
    pub fn list_authorized_users_and_teams(
        &self,
    ) -> request::ListAuthorizedUsersAndTeamsRequest {
        request::ListAuthorizedUsersAndTeamsRequest {
            http_client: &self,
            cursor: None,
            email: None,
            limit: None,
            name: None,
        }
    }
    /**Retrieve user or team

[https://api-docs.render.com/reference/get-owner](https://api-docs.render.com/reference/get-owner)*/
    pub fn retrieve_user_or_team(
        &self,
        owner_id: &str,
    ) -> request::RetrieveUserOrTeamRequest {
        request::RetrieveUserOrTeamRequest {
            http_client: &self,
            owner_id: owner_id.to_owned(),
        }
    }
    /**List services

[https://api-docs.render.com/reference/get-services](https://api-docs.render.com/reference/get-services)*/
    pub fn list_services(&self) -> request::ListServicesRequest {
        request::ListServicesRequest {
            http_client: &self,
            created_after: None,
            created_before: None,
            cursor: None,
            env: None,
            limit: None,
            name: None,
            owner_id: None,
            region: None,
            suspended: None,
            type_: None,
            updated_after: None,
            updated_before: None,
        }
    }
    /**Create service

[https://api-docs.render.com/reference/create-service](https://api-docs.render.com/reference/create-service)*/
    pub fn create_service(&self) -> request::CreateServiceRequest {
        request::CreateServiceRequest {
            http_client: &self,
            auto_deploy: None,
            branch: None,
            env_vars: None,
            name: None,
            owner_id: None,
            repo: None,
            secret_files: None,
            service_details: None,
            type_: None,
        }
    }
    /**Retrieve service

[https://api-docs.render.com/reference/get-service](https://api-docs.render.com/reference/get-service)*/
    pub fn retrieve_service(&self, service_id: &str) -> request::RetrieveServiceRequest {
        request::RetrieveServiceRequest {
            http_client: &self,
            service_id: service_id.to_owned(),
        }
    }
    /**Delete service

[https://api-docs.render.com/reference/delete-service](https://api-docs.render.com/reference/delete-service)*/
    pub fn delete_service(&self, service_id: &str) -> request::DeleteServiceRequest {
        request::DeleteServiceRequest {
            http_client: &self,
            service_id: service_id.to_owned(),
        }
    }
    /**Update service

[https://api-docs.render.com/reference/update-service](https://api-docs.render.com/reference/update-service)*/
    pub fn update_service(&self, service_id: &str) -> request::UpdateServiceRequest {
        request::UpdateServiceRequest {
            http_client: &self,
            auto_deploy: None,
            branch: None,
            name: None,
            service_details: None,
            service_id: service_id.to_owned(),
        }
    }
    /**Retrieve environment variables

[https://api-docs.render.com/reference/get-env-vars-for-service](https://api-docs.render.com/reference/get-env-vars-for-service)*/
    pub fn retrieve_environment_variables(
        &self,
        service_id: &str,
    ) -> request::RetrieveEnvironmentVariablesRequest {
        request::RetrieveEnvironmentVariablesRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            service_id: service_id.to_owned(),
        }
    }
    /**Update environment variables

[https://api-docs.render.com/reference/update-env-vars-for-service](https://api-docs.render.com/reference/update-env-vars-for-service)*/
    pub fn update_environment_variables(
        &self,
        body: serde_json::Value,
        service_id: &str,
    ) -> request::UpdateEnvironmentVariablesRequest {
        request::UpdateEnvironmentVariablesRequest {
            http_client: &self,
            body,
            service_id: service_id.to_owned(),
        }
    }
    /**Retrieve headers

[https://api-docs.render.com/reference/get-headers](https://api-docs.render.com/reference/get-headers)*/
    pub fn retrieve_headers(&self, service_id: &str) -> request::RetrieveHeadersRequest {
        request::RetrieveHeadersRequest {
            http_client: &self,
            cursor: None,
            limit: None,
            name: None,
            path: None,
            service_id: service_id.to_owned(),
            value: None,
        }
    }
    /**Retrieve redirect and rewrite rules

[https://api-docs.render.com/reference/get-routes](https://api-docs.render.com/reference/get-routes)*/
    pub fn retrieve_redirect_and_rewrite_rules(
        &self,
        service_id: &str,
    ) -> request::RetrieveRedirectAndRewriteRulesRequest {
        request::RetrieveRedirectAndRewriteRulesRequest {
            http_client: &self,
            cursor: None,
            destination: None,
            limit: None,
            service_id: service_id.to_owned(),
            source: None,
            type_: None,
        }
    }
    /**Suspend service

[https://api-docs.render.com/reference/suspend-service-1](https://api-docs.render.com/reference/suspend-service-1)*/
    pub fn suspend_service(&self, service_id: &str) -> request::SuspendServiceRequest {
        request::SuspendServiceRequest {
            http_client: &self,
            service_id: service_id.to_owned(),
        }
    }
    /**Resume service

[https://api-docs.render.com/reference/resume-service-1](https://api-docs.render.com/reference/resume-service-1)*/
    pub fn resume_service(&self, service_id: &str) -> request::ResumeServiceRequest {
        request::ResumeServiceRequest {
            http_client: &self,
            service_id: service_id.to_owned(),
        }
    }
    /**Scale service to desired number of instances

[https://api-docs.render.com/reference/scale-service](https://api-docs.render.com/reference/scale-service)*/
    pub fn scale_service_to_desired_number_of_instances(
        &self,
        service_id: &str,
    ) -> request::ScaleServiceToDesiredNumberOfInstancesRequest {
        request::ScaleServiceToDesiredNumberOfInstancesRequest {
            http_client: &self,
            num_instances: None,
            service_id: service_id.to_owned(),
        }
    }
    /**List deploys

[https://api-docs.render.com/reference/get-deploys](https://api-docs.render.com/reference/get-deploys)*/
    pub fn list_deploys(&self, service_id: &str) -> request::ListDeploysRequest {
        request::ListDeploysRequest {
            http_client: &self,
            cursor: None,
            end_time: None,
            limit: None,
            service_id: service_id.to_owned(),
            start_time: None,
        }
    }
    /**Trigger a deploy

[https://api-docs.render.com/reference/create-deploy](https://api-docs.render.com/reference/create-deploy)*/
    pub fn trigger_a_deploy(&self, service_id: &str) -> request::TriggerADeployRequest {
        request::TriggerADeployRequest {
            http_client: &self,
            clear_cache: None,
            service_id: service_id.to_owned(),
        }
    }
    /**Retrieve deploy

[https://api-docs.render.com/reference/get-deploy](https://api-docs.render.com/reference/get-deploy)*/
    pub fn retrieve_deploy(
        &self,
        deploy_id: &str,
        service_id: &str,
    ) -> request::RetrieveDeployRequest {
        request::RetrieveDeployRequest {
            http_client: &self,
            deploy_id: deploy_id.to_owned(),
            service_id: service_id.to_owned(),
        }
    }
    /**List custom domains

[https://api-docs.render.com/reference/get-custom-domains](https://api-docs.render.com/reference/get-custom-domains)*/
    pub fn list_custom_domains(
        &self,
        service_id: &str,
    ) -> request::ListCustomDomainsRequest {
        request::ListCustomDomainsRequest {
            http_client: &self,
            created_after: None,
            created_before: None,
            cursor: None,
            domain_type: None,
            limit: None,
            name: None,
            service_id: service_id.to_owned(),
            verification_status: None,
        }
    }
    /**Add custom domain

[https://api-docs.render.com/reference/create-custom-domain](https://api-docs.render.com/reference/create-custom-domain)*/
    pub fn add_custom_domain(
        &self,
        service_id: &str,
    ) -> request::AddCustomDomainRequest {
        request::AddCustomDomainRequest {
            http_client: &self,
            name: None,
            service_id: service_id.to_owned(),
        }
    }
    /**Retrieve custom domain

[https://api-docs.render.com/reference/get-custom-domain](https://api-docs.render.com/reference/get-custom-domain)*/
    pub fn retrieve_custom_domain(
        &self,
        custom_domain_id_or_name: &str,
        service_id: &str,
    ) -> request::RetrieveCustomDomainRequest {
        request::RetrieveCustomDomainRequest {
            http_client: &self,
            custom_domain_id_or_name: custom_domain_id_or_name.to_owned(),
            service_id: service_id.to_owned(),
        }
    }
    /**Delete custom domain

[https://api-docs.render.com/reference/delete-custom-domain](https://api-docs.render.com/reference/delete-custom-domain)*/
    pub fn delete_custom_domain(
        &self,
        custom_domain_id_or_name: &str,
        service_id: &str,
    ) -> request::DeleteCustomDomainRequest {
        request::DeleteCustomDomainRequest {
            http_client: &self,
            custom_domain_id_or_name: custom_domain_id_or_name.to_owned(),
            service_id: service_id.to_owned(),
        }
    }
    /**Verify DNS configuration

[https://api-docs.render.com/reference/refresh-custom-domain](https://api-docs.render.com/reference/refresh-custom-domain)*/
    pub fn verify_dns_configuration(
        &self,
        custom_domain_id_or_name: &str,
        service_id: &str,
    ) -> request::VerifyDnsConfigurationRequest {
        request::VerifyDnsConfigurationRequest {
            http_client: &self,
            custom_domain_id_or_name: custom_domain_id_or_name.to_owned(),
            service_id: service_id.to_owned(),
        }
    }
    /**List jobs

[https://api-docs.render.com/reference/list-job](https://api-docs.render.com/reference/list-job)*/
    pub fn list_jobs(&self, service_id: &str) -> request::ListJobsRequest {
        request::ListJobsRequest {
            http_client: &self,
            created_after: None,
            created_before: None,
            cursor: None,
            finished_after: None,
            finished_before: None,
            limit: None,
            service_id: service_id.to_owned(),
            started_after: None,
            started_before: None,
            status: None,
        }
    }
    /**Create job

[https://api-docs.render.com/reference/post-job](https://api-docs.render.com/reference/post-job)*/
    pub fn create_job(&self, service_id: &str) -> request::CreateJobRequest {
        request::CreateJobRequest {
            http_client: &self,
            plan_id: None,
            service_id: service_id.to_owned(),
            start_command: None,
        }
    }
    /**Retrieve job

[https://api-docs.render.com/reference/get-job](https://api-docs.render.com/reference/get-job)*/
    pub fn retrieve_job(
        &self,
        job_id: &str,
        service_id: &str,
    ) -> request::RetrieveJobRequest {
        request::RetrieveJobRequest {
            http_client: &self,
            job_id: job_id.to_owned(),
            service_id: service_id.to_owned(),
        }
    }
}
pub enum RenderAuthentication {
    ApiKeyAuth { api_key_auth: String },
}
impl RenderAuthentication {
    pub fn from_env() -> Self {
        Self::ApiKeyAuth {
            api_key_auth: std::env::var("RENDER_API_KEY_AUTH")
                .expect("Environment variable RENDER_API_KEY_AUTH is not set."),
        }
    }
}