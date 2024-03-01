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
pub struct MemoryLimit {
    #[serde(rename = "absoluteValue", skip_serializing_if = "Option::is_none")]
    pub absolute_value: Option<i64>,
    #[serde(rename = "percentOfMaxDirectMemory", skip_serializing_if = "Option::is_none")]
    pub percent_of_max_direct_memory: Option<f64>,
}

impl MemoryLimit {
    pub fn new() -> MemoryLimit {
        MemoryLimit {
            absolute_value: None,
            percent_of_max_direct_memory: None,
        }
    }
}

