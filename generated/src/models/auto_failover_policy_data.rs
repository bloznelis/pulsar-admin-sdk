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
pub struct AutoFailoverPolicyData {
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "policyType", skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<PolicyType>,
}

impl AutoFailoverPolicyData {
    pub fn new() -> AutoFailoverPolicyData {
        AutoFailoverPolicyData {
            parameters: None,
            policy_type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PolicyType {
    #[serde(rename = "min_available")]
    MinAvailable,
}

impl Default for PolicyType {
    fn default() -> PolicyType {
        Self::MinAvailable
    }
}

