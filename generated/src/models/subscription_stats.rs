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
pub struct SubscriptionStats {
    #[serde(rename = "activeConsumerName", skip_serializing_if = "Option::is_none")]
    pub active_consumer_name: Option<String>,
    #[serde(rename = "allowOutOfOrderDelivery", skip_serializing_if = "Option::is_none")]
    pub allow_out_of_order_delivery: Option<bool>,
    #[serde(rename = "backlogSize", skip_serializing_if = "Option::is_none")]
    pub backlog_size: Option<i64>,
    #[serde(rename = "blockedSubscriptionOnUnackedMsgs", skip_serializing_if = "Option::is_none")]
    pub blocked_subscription_on_unacked_msgs: Option<bool>,
    #[serde(rename = "bytesOutCounter", skip_serializing_if = "Option::is_none")]
    pub bytes_out_counter: Option<i64>,
    #[serde(rename = "chunkedMessageRate", skip_serializing_if = "Option::is_none")]
    pub chunked_message_rate: Option<i32>,
    #[serde(rename = "consumers", skip_serializing_if = "Option::is_none")]
    pub consumers: Option<Vec<models::ConsumerStats>>,
    #[serde(rename = "consumersAfterMarkDeletePosition", skip_serializing_if = "Option::is_none")]
    pub consumers_after_mark_delete_position: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "delayedMessageIndexSizeInBytes", skip_serializing_if = "Option::is_none")]
    pub delayed_message_index_size_in_bytes: Option<i64>,
    #[serde(rename = "durable", skip_serializing_if = "Option::is_none")]
    pub durable: Option<bool>,
    #[serde(rename = "earliestMsgPublishTimeInBacklog", skip_serializing_if = "Option::is_none")]
    pub earliest_msg_publish_time_in_backlog: Option<i64>,
    #[serde(rename = "filterAcceptedMsgCount", skip_serializing_if = "Option::is_none")]
    pub filter_accepted_msg_count: Option<i64>,
    #[serde(rename = "filterProcessedMsgCount", skip_serializing_if = "Option::is_none")]
    pub filter_processed_msg_count: Option<i64>,
    #[serde(rename = "filterRejectedMsgCount", skip_serializing_if = "Option::is_none")]
    pub filter_rejected_msg_count: Option<i64>,
    #[serde(rename = "filterRescheduledMsgCount", skip_serializing_if = "Option::is_none")]
    pub filter_rescheduled_msg_count: Option<i64>,
    #[serde(rename = "keySharedMode", skip_serializing_if = "Option::is_none")]
    pub key_shared_mode: Option<String>,
    #[serde(rename = "lastAckedTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_acked_timestamp: Option<i64>,
    #[serde(rename = "lastConsumedFlowTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_consumed_flow_timestamp: Option<i64>,
    #[serde(rename = "lastConsumedTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_consumed_timestamp: Option<i64>,
    #[serde(rename = "lastExpireTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_expire_timestamp: Option<i64>,
    #[serde(rename = "lastMarkDeleteAdvancedTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_mark_delete_advanced_timestamp: Option<i64>,
    #[serde(rename = "messageAckRate", skip_serializing_if = "Option::is_none")]
    pub message_ack_rate: Option<f64>,
    #[serde(rename = "msgBacklog", skip_serializing_if = "Option::is_none")]
    pub msg_backlog: Option<i64>,
    #[serde(rename = "msgBacklogNoDelayed", skip_serializing_if = "Option::is_none")]
    pub msg_backlog_no_delayed: Option<i64>,
    #[serde(rename = "msgDelayed", skip_serializing_if = "Option::is_none")]
    pub msg_delayed: Option<i64>,
    #[serde(rename = "msgOutCounter", skip_serializing_if = "Option::is_none")]
    pub msg_out_counter: Option<i64>,
    #[serde(rename = "msgRateExpired", skip_serializing_if = "Option::is_none")]
    pub msg_rate_expired: Option<f64>,
    #[serde(rename = "msgRateOut", skip_serializing_if = "Option::is_none")]
    pub msg_rate_out: Option<f64>,
    #[serde(rename = "msgRateRedeliver", skip_serializing_if = "Option::is_none")]
    pub msg_rate_redeliver: Option<f64>,
    #[serde(rename = "msgThroughputOut", skip_serializing_if = "Option::is_none")]
    pub msg_throughput_out: Option<f64>,
    #[serde(rename = "nonContiguousDeletedMessagesRanges", skip_serializing_if = "Option::is_none")]
    pub non_contiguous_deleted_messages_ranges: Option<i32>,
    #[serde(rename = "nonContiguousDeletedMessagesRangesSerializedSize", skip_serializing_if = "Option::is_none")]
    pub non_contiguous_deleted_messages_ranges_serialized_size: Option<i32>,
    #[serde(rename = "replicated", skip_serializing_if = "Option::is_none")]
    pub replicated: Option<bool>,
    #[serde(rename = "subscriptionProperties", skip_serializing_if = "Option::is_none")]
    pub subscription_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "totalMsgExpired", skip_serializing_if = "Option::is_none")]
    pub total_msg_expired: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "unackedMessages", skip_serializing_if = "Option::is_none")]
    pub unacked_messages: Option<i64>,
}

impl SubscriptionStats {
    pub fn new() -> SubscriptionStats {
        SubscriptionStats {
            active_consumer_name: None,
            allow_out_of_order_delivery: None,
            backlog_size: None,
            blocked_subscription_on_unacked_msgs: None,
            bytes_out_counter: None,
            chunked_message_rate: None,
            consumers: None,
            consumers_after_mark_delete_position: None,
            delayed_message_index_size_in_bytes: None,
            durable: None,
            earliest_msg_publish_time_in_backlog: None,
            filter_accepted_msg_count: None,
            filter_processed_msg_count: None,
            filter_rejected_msg_count: None,
            filter_rescheduled_msg_count: None,
            key_shared_mode: None,
            last_acked_timestamp: None,
            last_consumed_flow_timestamp: None,
            last_consumed_timestamp: None,
            last_expire_timestamp: None,
            last_mark_delete_advanced_timestamp: None,
            message_ack_rate: None,
            msg_backlog: None,
            msg_backlog_no_delayed: None,
            msg_delayed: None,
            msg_out_counter: None,
            msg_rate_expired: None,
            msg_rate_out: None,
            msg_rate_redeliver: None,
            msg_throughput_out: None,
            non_contiguous_deleted_messages_ranges: None,
            non_contiguous_deleted_messages_ranges_serialized_size: None,
            replicated: None,
            subscription_properties: None,
            total_msg_expired: None,
            r#type: None,
            unacked_messages: None,
        }
    }
}

