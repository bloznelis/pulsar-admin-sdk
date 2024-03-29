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
pub struct CursorStats {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "cursorLedger", skip_serializing_if = "Option::is_none")]
    pub cursor_ledger: Option<i64>,
    #[serde(rename = "cursorLedgerLastEntry", skip_serializing_if = "Option::is_none")]
    pub cursor_ledger_last_entry: Option<i64>,
    #[serde(rename = "individuallyDeletedMessages", skip_serializing_if = "Option::is_none")]
    pub individually_deleted_messages: Option<String>,
    #[serde(rename = "lastLedgerSwitchTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_ledger_switch_timestamp: Option<String>,
    #[serde(rename = "markDeletePosition", skip_serializing_if = "Option::is_none")]
    pub mark_delete_position: Option<String>,
    #[serde(rename = "messagesConsumedCounter", skip_serializing_if = "Option::is_none")]
    pub messages_consumed_counter: Option<i64>,
    #[serde(rename = "numberOfEntriesSinceFirstNotAckedMessage", skip_serializing_if = "Option::is_none")]
    pub number_of_entries_since_first_not_acked_message: Option<i64>,
    #[serde(rename = "pendingReadOps", skip_serializing_if = "Option::is_none")]
    pub pending_read_ops: Option<i32>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, i64>>,
    #[serde(rename = "readPosition", skip_serializing_if = "Option::is_none")]
    pub read_position: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "subscriptionHavePendingRead", skip_serializing_if = "Option::is_none")]
    pub subscription_have_pending_read: Option<bool>,
    #[serde(rename = "subscriptionHavePendingReplayRead", skip_serializing_if = "Option::is_none")]
    pub subscription_have_pending_replay_read: Option<bool>,
    #[serde(rename = "totalNonContiguousDeletedMessagesRange", skip_serializing_if = "Option::is_none")]
    pub total_non_contiguous_deleted_messages_range: Option<i32>,
    #[serde(rename = "waitingReadOp", skip_serializing_if = "Option::is_none")]
    pub waiting_read_op: Option<bool>,
}

impl CursorStats {
    pub fn new() -> CursorStats {
        CursorStats {
            active: None,
            cursor_ledger: None,
            cursor_ledger_last_entry: None,
            individually_deleted_messages: None,
            last_ledger_switch_timestamp: None,
            mark_delete_position: None,
            messages_consumed_counter: None,
            number_of_entries_since_first_not_acked_message: None,
            pending_read_ops: None,
            properties: None,
            read_position: None,
            state: None,
            subscription_have_pending_read: None,
            subscription_have_pending_replay_read: None,
            total_non_contiguous_deleted_messages_range: None,
            waiting_read_op: None,
        }
    }
}

