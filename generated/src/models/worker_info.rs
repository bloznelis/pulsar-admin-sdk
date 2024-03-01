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
pub struct WorkerInfo {
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "workerHostname", skip_serializing_if = "Option::is_none")]
    pub worker_hostname: Option<String>,
    #[serde(rename = "workerId", skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
}

impl WorkerInfo {
    pub fn new() -> WorkerInfo {
        WorkerInfo {
            port: None,
            worker_hostname: None,
            worker_id: None,
        }
    }
}
