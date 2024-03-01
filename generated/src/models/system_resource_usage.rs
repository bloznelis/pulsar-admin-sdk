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
pub struct SystemResourceUsage {
    #[serde(rename = "bandwidthIn", skip_serializing_if = "Option::is_none")]
    pub bandwidth_in: Option<Box<models::ResourceUsage>>,
    #[serde(rename = "bandwidthOut", skip_serializing_if = "Option::is_none")]
    pub bandwidth_out: Option<Box<models::ResourceUsage>>,
    #[serde(rename = "cpu", skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Box<models::ResourceUsage>>,
    #[serde(rename = "directMemory", skip_serializing_if = "Option::is_none")]
    pub direct_memory: Option<Box<models::ResourceUsage>>,
    #[serde(rename = "memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Box<models::ResourceUsage>>,
}

impl SystemResourceUsage {
    pub fn new() -> SystemResourceUsage {
        SystemResourceUsage {
            bandwidth_in: None,
            bandwidth_out: None,
            cpu: None,
            direct_memory: None,
            memory: None,
        }
    }
}
