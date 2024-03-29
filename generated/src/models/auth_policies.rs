/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthPolicies {
    #[serde(rename = "namespaceAuthentication", skip_serializing_if = "Option::is_none")]
    pub namespace_authentication: Option<NamespaceAuthentication>,
    #[serde(rename = "subscriptionAuthentication", skip_serializing_if = "Option::is_none")]
    pub subscription_authentication: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "topicAuthentication", skip_serializing_if = "Option::is_none")]
    pub topic_authentication: Option<TopicAuthentication>,
}

impl AuthPolicies {
    pub fn new() -> AuthPolicies {
        AuthPolicies {
            namespace_authentication: None,
            subscription_authentication: None,
            topic_authentication: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NamespaceAuthentication {
    #[serde(rename = "produce")]
    Produce,
    #[serde(rename = "consume")]
    Consume,
    #[serde(rename = "functions")]
    Functions,
    #[serde(rename = "sources")]
    Sources,
    #[serde(rename = "sinks")]
    Sinks,
    #[serde(rename = "packages")]
    Packages,
}

impl Default for NamespaceAuthentication {
    fn default() -> NamespaceAuthentication {
        Self::Produce
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TopicAuthentication {
    #[serde(rename = "produce")]
    Produce,
    #[serde(rename = "consume")]
    Consume,
    #[serde(rename = "functions")]
    Functions,
    #[serde(rename = "sources")]
    Sources,
    #[serde(rename = "sinks")]
    Sinks,
    #[serde(rename = "packages")]
    Packages,
}

impl Default for TopicAuthentication {
    fn default() -> TopicAuthentication {
        Self::Produce
    }
}

