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
pub struct RetentionPolicies {
    #[serde(rename = "retentionSizeInMB", skip_serializing_if = "Option::is_none")]
    pub retention_size_in_mb: Option<i64>,
    #[serde(rename = "retentionTimeInMinutes", skip_serializing_if = "Option::is_none")]
    pub retention_time_in_minutes: Option<i32>,
}

impl RetentionPolicies {
    pub fn new() -> RetentionPolicies {
        RetentionPolicies {
            retention_size_in_mb: None,
            retention_time_in_minutes: None,
        }
    }
}

