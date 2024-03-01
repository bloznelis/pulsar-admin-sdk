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
pub struct NonPersistentPublisherStats {
    #[serde(rename = "accessMode", skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<AccessMode>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "averageMsgSize", skip_serializing_if = "Option::is_none")]
    pub average_msg_size: Option<f64>,
    #[serde(rename = "chunkedMessageRate", skip_serializing_if = "Option::is_none")]
    pub chunked_message_rate: Option<f64>,
    #[serde(rename = "clientVersion", skip_serializing_if = "Option::is_none")]
    pub client_version: Option<String>,
    #[serde(rename = "connectedSince", skip_serializing_if = "Option::is_none")]
    pub connected_since: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "msgDropRate", skip_serializing_if = "Option::is_none")]
    pub msg_drop_rate: Option<f64>,
    #[serde(rename = "msgRateIn", skip_serializing_if = "Option::is_none")]
    pub msg_rate_in: Option<f64>,
    #[serde(rename = "msgThroughputIn", skip_serializing_if = "Option::is_none")]
    pub msg_throughput_in: Option<f64>,
    #[serde(rename = "producerId", skip_serializing_if = "Option::is_none")]
    pub producer_id: Option<i64>,
    #[serde(rename = "producerName", skip_serializing_if = "Option::is_none")]
    pub producer_name: Option<String>,
    #[serde(rename = "supportsPartialProducer", skip_serializing_if = "Option::is_none")]
    pub supports_partial_producer: Option<bool>,
}

impl NonPersistentPublisherStats {
    pub fn new() -> NonPersistentPublisherStats {
        NonPersistentPublisherStats {
            access_mode: None,
            address: None,
            average_msg_size: None,
            chunked_message_rate: None,
            client_version: None,
            connected_since: None,
            metadata: None,
            msg_drop_rate: None,
            msg_rate_in: None,
            msg_throughput_in: None,
            producer_id: None,
            producer_name: None,
            supports_partial_producer: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccessMode {
    #[serde(rename = "Shared")]
    Shared,
    #[serde(rename = "Exclusive")]
    Exclusive,
    #[serde(rename = "ExclusiveWithFencing")]
    ExclusiveWithFencing,
    #[serde(rename = "WaitForExclusive")]
    WaitForExclusive,
}

impl Default for AccessMode {
    fn default() -> AccessMode {
        Self::Shared
    }
}

