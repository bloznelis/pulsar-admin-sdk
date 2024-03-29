/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`brokers_base_backlog_quota_check`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseBacklogQuotaCheckError {
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_delete_dynamic_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseDeleteDynamicConfigurationError {
    Status403(),
    Status412(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_get_active_brokers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseGetActiveBrokersError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_get_active_brokers_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseGetActiveBrokers0Error {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_get_all_dynamic_configurations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseGetAllDynamicConfigurationsError {
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_get_dynamic_configuration_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseGetDynamicConfigurationNameError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_get_internal_configuration_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseGetInternalConfigurationDataError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_get_leader_broker`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseGetLeaderBrokerError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_get_owned_namespaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseGetOwnedNamespacesError {
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_get_runtime_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseGetRuntimeConfigurationError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_health_check`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseHealthCheckError {
    Status403(),
    Status404(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_is_ready`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseIsReadyError {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_shut_down_broker_gracefully`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseShutDownBrokerGracefullyError {
    Status403(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_update_dynamic_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseUpdateDynamicConfigurationError {
    Status403(),
    Status404(),
    Status412(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`brokers_base_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrokersBaseVersionError {
    Status500(),
    UnknownValue(serde_json::Value),
}


pub async fn brokers_base_backlog_quota_check(configuration: &configuration::Configuration, ) -> Result<(), Error<BrokersBaseBacklogQuotaCheckError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/backlog-quota-check", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseBacklogQuotaCheckError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_delete_dynamic_configuration(configuration: &configuration::Configuration, config_name: &str) -> Result<(), Error<BrokersBaseDeleteDynamicConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/configuration/{configName}", local_var_configuration.base_path, configName=crate::apis::urlencode(config_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseDeleteDynamicConfigurationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_get_active_brokers(configuration: &configuration::Configuration, ) -> Result<Vec<String>, Error<BrokersBaseGetActiveBrokersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BrokersBaseGetActiveBrokersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_get_active_brokers_0(configuration: &configuration::Configuration, cluster: &str) -> Result<Vec<String>, Error<BrokersBaseGetActiveBrokers0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/{cluster}", local_var_configuration.base_path, cluster=crate::apis::urlencode(cluster));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BrokersBaseGetActiveBrokers0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_get_all_dynamic_configurations(configuration: &configuration::Configuration, ) -> Result<(), Error<BrokersBaseGetAllDynamicConfigurationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/configuration/values", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseGetAllDynamicConfigurationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_get_dynamic_configuration_name(configuration: &configuration::Configuration, ) -> Result<(), Error<BrokersBaseGetDynamicConfigurationNameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/configuration", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseGetDynamicConfigurationNameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_get_internal_configuration_data(configuration: &configuration::Configuration, ) -> Result<models::InternalConfigurationData, Error<BrokersBaseGetInternalConfigurationDataError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/internal-configuration", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BrokersBaseGetInternalConfigurationDataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_get_leader_broker(configuration: &configuration::Configuration, ) -> Result<models::BrokerInfo, Error<BrokersBaseGetLeaderBrokerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/leaderBroker", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BrokersBaseGetLeaderBrokerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_get_owned_namespaces(configuration: &configuration::Configuration, cluster_name: &str, broker_id: &str) -> Result<std::collections::HashMap<String, models::NamespaceOwnershipStatus>, Error<BrokersBaseGetOwnedNamespacesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/{clusterName}/{brokerId}/ownedNamespaces", local_var_configuration.base_path, clusterName=crate::apis::urlencode(cluster_name), brokerId=crate::apis::urlencode(broker_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BrokersBaseGetOwnedNamespacesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_get_runtime_configuration(configuration: &configuration::Configuration, ) -> Result<(), Error<BrokersBaseGetRuntimeConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/configuration/runtime", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseGetRuntimeConfigurationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_health_check(configuration: &configuration::Configuration, topic_version: Option<&str>) -> Result<(), Error<BrokersBaseHealthCheckError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/health", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = topic_version {
        local_var_req_builder = local_var_req_builder.query(&[("topicVersion", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseHealthCheckError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_is_ready(configuration: &configuration::Configuration, ) -> Result<(), Error<BrokersBaseIsReadyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/ready", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseIsReadyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_shut_down_broker_gracefully(configuration: &configuration::Configuration, max_concurrent_unload_per_sec: Option<i32>, forced_terminate_topic: Option<bool>) -> Result<(), Error<BrokersBaseShutDownBrokerGracefullyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/shutdown", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = max_concurrent_unload_per_sec {
        local_var_req_builder = local_var_req_builder.query(&[("maxConcurrentUnloadPerSec", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = forced_terminate_topic {
        local_var_req_builder = local_var_req_builder.query(&[("forcedTerminateTopic", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseShutDownBrokerGracefullyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_update_dynamic_configuration(configuration: &configuration::Configuration, config_name: &str, config_value: &str) -> Result<(), Error<BrokersBaseUpdateDynamicConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/configuration/{configName}/{configValue}", local_var_configuration.base_path, configName=crate::apis::urlencode(config_name), configValue=crate::apis::urlencode(config_value));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<BrokersBaseUpdateDynamicConfigurationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn brokers_base_version(configuration: &configuration::Configuration, ) -> Result<String, Error<BrokersBaseVersionError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/brokers/version", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BrokersBaseVersionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

