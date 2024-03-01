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
pub struct Policies {
    #[serde(rename = "auth_policies", skip_serializing_if = "Option::is_none")]
    pub auth_policies: Option<Box<models::AuthPolicies>>,
    #[serde(rename = "autoSubscriptionCreationOverride", skip_serializing_if = "Option::is_none")]
    pub auto_subscription_creation_override: Option<Box<models::AutoSubscriptionCreationOverride>>,
    #[serde(rename = "autoTopicCreationOverride", skip_serializing_if = "Option::is_none")]
    pub auto_topic_creation_override: Option<Box<models::AutoTopicCreationOverride>>,
    #[serde(rename = "backlog_quota_map", skip_serializing_if = "Option::is_none")]
    pub backlog_quota_map: Option<std::collections::HashMap<String, models::BacklogQuota>>,
    #[serde(rename = "bundles", skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Box<models::BundlesData>>,
    #[serde(rename = "clusterDispatchRate", skip_serializing_if = "Option::is_none")]
    pub cluster_dispatch_rate: Option<std::collections::HashMap<String, models::DispatchRateImpl>>,
    #[serde(rename = "clusterSubscribeRate", skip_serializing_if = "Option::is_none")]
    pub cluster_subscribe_rate: Option<std::collections::HashMap<String, models::SubscribeRate>>,
    #[serde(rename = "compaction_threshold", skip_serializing_if = "Option::is_none")]
    pub compaction_threshold: Option<i64>,
    #[serde(rename = "deduplicationEnabled", skip_serializing_if = "Option::is_none")]
    pub deduplication_enabled: Option<bool>,
    #[serde(rename = "deduplicationSnapshotIntervalSeconds", skip_serializing_if = "Option::is_none")]
    pub deduplication_snapshot_interval_seconds: Option<i32>,
    #[serde(rename = "delayed_delivery_policies", skip_serializing_if = "Option::is_none")]
    pub delayed_delivery_policies: Option<Box<models::DelayedDeliveryPolicies>>,
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "dispatcherPauseOnAckStatePersistentEnabled", skip_serializing_if = "Option::is_none")]
    pub dispatcher_pause_on_ack_state_persistent_enabled: Option<bool>,
    #[serde(rename = "encryption_required", skip_serializing_if = "Option::is_none")]
    pub encryption_required: Option<bool>,
    #[serde(rename = "entryFilters", skip_serializing_if = "Option::is_none")]
    pub entry_filters: Option<Box<models::EntryFilters>>,
    #[serde(rename = "inactive_topic_policies", skip_serializing_if = "Option::is_none")]
    pub inactive_topic_policies: Option<Box<models::InactiveTopicPolicies>>,
    #[serde(rename = "is_allow_auto_update_schema", skip_serializing_if = "Option::is_none")]
    pub is_allow_auto_update_schema: Option<bool>,
    #[serde(rename = "latency_stats_sample_rate", skip_serializing_if = "Option::is_none")]
    pub latency_stats_sample_rate: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "max_consumers_per_subscription", skip_serializing_if = "Option::is_none")]
    pub max_consumers_per_subscription: Option<i32>,
    #[serde(rename = "max_consumers_per_topic", skip_serializing_if = "Option::is_none")]
    pub max_consumers_per_topic: Option<i32>,
    #[serde(rename = "max_producers_per_topic", skip_serializing_if = "Option::is_none")]
    pub max_producers_per_topic: Option<i32>,
    #[serde(rename = "max_subscriptions_per_topic", skip_serializing_if = "Option::is_none")]
    pub max_subscriptions_per_topic: Option<i32>,
    #[serde(rename = "max_topics_per_namespace", skip_serializing_if = "Option::is_none")]
    pub max_topics_per_namespace: Option<i32>,
    #[serde(rename = "max_unacked_messages_per_consumer", skip_serializing_if = "Option::is_none")]
    pub max_unacked_messages_per_consumer: Option<i32>,
    #[serde(rename = "max_unacked_messages_per_subscription", skip_serializing_if = "Option::is_none")]
    pub max_unacked_messages_per_subscription: Option<i32>,
    #[serde(rename = "message_ttl_in_seconds", skip_serializing_if = "Option::is_none")]
    pub message_ttl_in_seconds: Option<i32>,
    #[serde(rename = "migrated", skip_serializing_if = "Option::is_none")]
    pub migrated: Option<bool>,
    #[serde(rename = "offload_deletion_lag_ms", skip_serializing_if = "Option::is_none")]
    pub offload_deletion_lag_ms: Option<i64>,
    #[serde(rename = "offload_policies", skip_serializing_if = "Option::is_none")]
    pub offload_policies: Option<Box<models::OffloadPolicies>>,
    #[serde(rename = "offload_threshold", skip_serializing_if = "Option::is_none")]
    pub offload_threshold: Option<i64>,
    #[serde(rename = "offload_threshold_in_seconds", skip_serializing_if = "Option::is_none")]
    pub offload_threshold_in_seconds: Option<i64>,
    #[serde(rename = "persistence", skip_serializing_if = "Option::is_none")]
    pub persistence: Option<Box<models::PersistencePolicies>>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "publishMaxMessageRate", skip_serializing_if = "Option::is_none")]
    pub publish_max_message_rate: Option<std::collections::HashMap<String, models::PublishRate>>,
    #[serde(rename = "replication_clusters", skip_serializing_if = "Option::is_none")]
    pub replication_clusters: Option<Vec<String>>,
    #[serde(rename = "replicatorDispatchRate", skip_serializing_if = "Option::is_none")]
    pub replicator_dispatch_rate: Option<std::collections::HashMap<String, models::DispatchRateImpl>>,
    #[serde(rename = "resource_group_name", skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[serde(rename = "retention_policies", skip_serializing_if = "Option::is_none")]
    pub retention_policies: Option<Box<models::RetentionPolicies>>,
    #[serde(rename = "schema_auto_update_compatibility_strategy", skip_serializing_if = "Option::is_none")]
    pub schema_auto_update_compatibility_strategy: Option<SchemaAutoUpdateCompatibilityStrategy>,
    #[serde(rename = "schema_compatibility_strategy", skip_serializing_if = "Option::is_none")]
    pub schema_compatibility_strategy: Option<SchemaCompatibilityStrategy>,
    #[serde(rename = "schema_validation_enforced", skip_serializing_if = "Option::is_none")]
    pub schema_validation_enforced: Option<bool>,
    #[serde(rename = "subscriptionDispatchRate", skip_serializing_if = "Option::is_none")]
    pub subscription_dispatch_rate: Option<std::collections::HashMap<String, models::DispatchRateImpl>>,
    #[serde(rename = "subscription_auth_mode", skip_serializing_if = "Option::is_none")]
    pub subscription_auth_mode: Option<SubscriptionAuthMode>,
    #[serde(rename = "subscription_expiration_time_minutes", skip_serializing_if = "Option::is_none")]
    pub subscription_expiration_time_minutes: Option<i32>,
    #[serde(rename = "subscription_types_enabled", skip_serializing_if = "Option::is_none")]
    pub subscription_types_enabled: Option<Vec<String>>,
    #[serde(rename = "topicDispatchRate", skip_serializing_if = "Option::is_none")]
    pub topic_dispatch_rate: Option<std::collections::HashMap<String, models::DispatchRateImpl>>,
}

impl Policies {
    pub fn new() -> Policies {
        Policies {
            auth_policies: None,
            auto_subscription_creation_override: None,
            auto_topic_creation_override: None,
            backlog_quota_map: None,
            bundles: None,
            cluster_dispatch_rate: None,
            cluster_subscribe_rate: None,
            compaction_threshold: None,
            deduplication_enabled: None,
            deduplication_snapshot_interval_seconds: None,
            delayed_delivery_policies: None,
            deleted: None,
            dispatcher_pause_on_ack_state_persistent_enabled: None,
            encryption_required: None,
            entry_filters: None,
            inactive_topic_policies: None,
            is_allow_auto_update_schema: None,
            latency_stats_sample_rate: None,
            max_consumers_per_subscription: None,
            max_consumers_per_topic: None,
            max_producers_per_topic: None,
            max_subscriptions_per_topic: None,
            max_topics_per_namespace: None,
            max_unacked_messages_per_consumer: None,
            max_unacked_messages_per_subscription: None,
            message_ttl_in_seconds: None,
            migrated: None,
            offload_deletion_lag_ms: None,
            offload_policies: None,
            offload_threshold: None,
            offload_threshold_in_seconds: None,
            persistence: None,
            properties: None,
            publish_max_message_rate: None,
            replication_clusters: None,
            replicator_dispatch_rate: None,
            resource_group_name: None,
            retention_policies: None,
            schema_auto_update_compatibility_strategy: None,
            schema_compatibility_strategy: None,
            schema_validation_enforced: None,
            subscription_dispatch_rate: None,
            subscription_auth_mode: None,
            subscription_expiration_time_minutes: None,
            subscription_types_enabled: None,
            topic_dispatch_rate: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SchemaAutoUpdateCompatibilityStrategy {
    #[serde(rename = "AutoUpdateDisabled")]
    AutoUpdateDisabled,
    #[serde(rename = "Backward")]
    Backward,
    #[serde(rename = "Forward")]
    Forward,
    #[serde(rename = "Full")]
    Full,
    #[serde(rename = "AlwaysCompatible")]
    AlwaysCompatible,
    #[serde(rename = "BackwardTransitive")]
    BackwardTransitive,
    #[serde(rename = "ForwardTransitive")]
    ForwardTransitive,
    #[serde(rename = "FullTransitive")]
    FullTransitive,
}

impl Default for SchemaAutoUpdateCompatibilityStrategy {
    fn default() -> SchemaAutoUpdateCompatibilityStrategy {
        Self::AutoUpdateDisabled
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SchemaCompatibilityStrategy {
    #[serde(rename = "UNDEFINED")]
    Undefined,
    #[serde(rename = "ALWAYS_INCOMPATIBLE")]
    AlwaysIncompatible,
    #[serde(rename = "ALWAYS_COMPATIBLE")]
    AlwaysCompatible,
    #[serde(rename = "BACKWARD")]
    Backward,
    #[serde(rename = "FORWARD")]
    Forward,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "BACKWARD_TRANSITIVE")]
    BackwardTransitive,
    #[serde(rename = "FORWARD_TRANSITIVE")]
    ForwardTransitive,
    #[serde(rename = "FULL_TRANSITIVE")]
    FullTransitive,
}

impl Default for SchemaCompatibilityStrategy {
    fn default() -> SchemaCompatibilityStrategy {
        Self::Undefined
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubscriptionAuthMode {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Prefix")]
    Prefix,
}

impl Default for SubscriptionAuthMode {
    fn default() -> SubscriptionAuthMode {
        Self::None
    }
}

