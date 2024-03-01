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
pub struct ResourceDescription {
    #[serde(rename = "resourceUsage", skip_serializing_if = "Option::is_none")]
    pub resource_usage: Option<std::collections::HashMap<String, models::ResourceUsage>>,
    #[serde(rename = "usagePct", skip_serializing_if = "Option::is_none")]
    pub usage_pct: Option<i32>,
}

impl ResourceDescription {
    pub fn new() -> ResourceDescription {
        ResourceDescription {
            resource_usage: None,
            usage_pct: None,
        }
    }
}
