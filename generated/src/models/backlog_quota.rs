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
pub struct BacklogQuota {
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "limitSize", skip_serializing_if = "Option::is_none")]
    pub limit_size: Option<i64>,
    #[serde(rename = "limitTime", skip_serializing_if = "Option::is_none")]
    pub limit_time: Option<i32>,
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

impl BacklogQuota {
    pub fn new() -> BacklogQuota {
        BacklogQuota {
            limit: None,
            limit_size: None,
            limit_time: None,
            policy: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Policy {
    #[serde(rename = "producer_request_hold")]
    ProducerRequestHold,
    #[serde(rename = "producer_exception")]
    ProducerException,
    #[serde(rename = "consumer_backlog_eviction")]
    ConsumerBacklogEviction,
}

impl Default for Policy {
    fn default() -> Policy {
        Self::ProducerRequestHold
    }
}

