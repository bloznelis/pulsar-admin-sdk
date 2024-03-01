# \NonPersistentTopicApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**non_persistent_topics_create_partitioned_topic**](NonPersistentTopicApi.md#non_persistent_topics_create_partitioned_topic) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/partitions | Create a partitioned topic.
[**non_persistent_topics_get_entry_filters**](NonPersistentTopicApi.md#non_persistent_topics_get_entry_filters) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/entryFilters | Get entry filters for a topic.
[**non_persistent_topics_get_internal_stats**](NonPersistentTopicApi.md#non_persistent_topics_get_internal_stats) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/internalStats | Get the internal stats for the topic.
[**non_persistent_topics_get_list**](NonPersistentTopicApi.md#non_persistent_topics_get_list) | **GET** /non-persistent/{tenant}/{namespace} | Get the list of non-persistent topics under a namespace.
[**non_persistent_topics_get_list_from_bundle**](NonPersistentTopicApi.md#non_persistent_topics_get_list_from_bundle) | **GET** /non-persistent/{tenant}/{namespace}/{bundle} | Get the list of non-persistent topics under a namespace bundle.
[**non_persistent_topics_get_partitioned_metadata**](NonPersistentTopicApi.md#non_persistent_topics_get_partitioned_metadata) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/partitions | Get partitioned topic metadata.
[**non_persistent_topics_get_partitioned_stats**](NonPersistentTopicApi.md#non_persistent_topics_get_partitioned_stats) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/partitioned-stats | Get the stats for the partitioned topic.
[**non_persistent_topics_remove_entry_filters**](NonPersistentTopicApi.md#non_persistent_topics_remove_entry_filters) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/entryFilters | Remove entry filters for specified topic.
[**non_persistent_topics_set_entry_filters**](NonPersistentTopicApi.md#non_persistent_topics_set_entry_filters) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/entryFilters | Set entry filters for specified topic
[**non_persistent_topics_truncate_topic**](NonPersistentTopicApi.md#non_persistent_topics_truncate_topic) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/truncate | Truncate a topic.
[**non_persistent_topics_unload_topic**](NonPersistentTopicApi.md#non_persistent_topics_unload_topic) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/unload | Unload a topic
[**persistent_topics_analyze_subscription_backlog**](NonPersistentTopicApi.md#persistent_topics_analyze_subscription_backlog) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/analyzeBacklog | Analyse a subscription, by scanning all the unprocessed messages
[**persistent_topics_compact**](NonPersistentTopicApi.md#persistent_topics_compact) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/compaction | Trigger a compaction operation on a topic.
[**persistent_topics_compaction_status**](NonPersistentTopicApi.md#persistent_topics_compaction_status) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/compaction | Get the status of a compaction operation for a topic.
[**persistent_topics_create_missed_partitions**](NonPersistentTopicApi.md#persistent_topics_create_missed_partitions) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/createMissedPartitions | Create missed partitions of an existing partitioned topic.
[**persistent_topics_create_non_partitioned_topic**](NonPersistentTopicApi.md#persistent_topics_create_non_partitioned_topic) | **PUT** /non-persistent/{tenant}/{namespace}/{topic} | Create a non-partitioned topic.
[**persistent_topics_create_subscription**](NonPersistentTopicApi.md#persistent_topics_create_subscription) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subscriptionName} | Create a subscription on the topic.
[**persistent_topics_delete_deduplication_snapshot_interval**](NonPersistentTopicApi.md#persistent_topics_delete_deduplication_snapshot_interval) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/deduplicationSnapshotInterval | Delete deduplicationSnapshotInterval config on a topic.
[**persistent_topics_delete_delayed_delivery_policies**](NonPersistentTopicApi.md#persistent_topics_delete_delayed_delivery_policies) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/delayedDelivery | Set delayed delivery messages config on a topic.
[**persistent_topics_delete_inactive_topic_policies**](NonPersistentTopicApi.md#persistent_topics_delete_inactive_topic_policies) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/inactiveTopicPolicies | Delete inactive topic policies on a topic.
[**persistent_topics_delete_max_unacked_messages_on_consumer**](NonPersistentTopicApi.md#persistent_topics_delete_max_unacked_messages_on_consumer) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/maxUnackedMessagesOnConsumer | Delete max unacked messages per consumer config on a topic.
[**persistent_topics_delete_max_unacked_messages_on_subscription**](NonPersistentTopicApi.md#persistent_topics_delete_max_unacked_messages_on_subscription) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/maxUnackedMessagesOnSubscription | Delete max unacked messages per subscription config on a topic.
[**persistent_topics_delete_partitioned_topic**](NonPersistentTopicApi.md#persistent_topics_delete_partitioned_topic) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/partitions | Delete a partitioned topic.
[**persistent_topics_delete_shadow_topics**](NonPersistentTopicApi.md#persistent_topics_delete_shadow_topics) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/shadowTopics | Delete shadow topics for a topic
[**persistent_topics_delete_subscription**](NonPersistentTopicApi.md#persistent_topics_delete_subscription) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName} | Delete a subscription.
[**persistent_topics_delete_topic**](NonPersistentTopicApi.md#persistent_topics_delete_topic) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic} | Delete a topic.
[**persistent_topics_examine_message**](NonPersistentTopicApi.md#persistent_topics_examine_message) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/examinemessage | Examine a specific message on a topic by position relative to the earliest or the latest message.
[**persistent_topics_expire_messages_for_all_subscriptions**](NonPersistentTopicApi.md#persistent_topics_expire_messages_for_all_subscriptions) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/all_subscription/expireMessages/{expireTimeInSeconds} | Expiry messages on all subscriptions of topic.
[**persistent_topics_expire_topic_messages**](NonPersistentTopicApi.md#persistent_topics_expire_topic_messages) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/expireMessages | Expiry messages on a topic subscription.
[**persistent_topics_expire_topic_messages_0**](NonPersistentTopicApi.md#persistent_topics_expire_topic_messages_0) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/expireMessages/{expireTimeInSeconds} | Expiry messages on a topic subscription.
[**persistent_topics_get_auto_subscription_creation**](NonPersistentTopicApi.md#persistent_topics_get_auto_subscription_creation) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/autoSubscriptionCreation | Get autoSubscriptionCreation info in a topic
[**persistent_topics_get_backlog**](NonPersistentTopicApi.md#persistent_topics_get_backlog) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/backlog | Get estimated backlog for offline topic.
[**persistent_topics_get_backlog_quota_map**](NonPersistentTopicApi.md#persistent_topics_get_backlog_quota_map) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/backlogQuotaMap | Get backlog quota map on a topic.
[**persistent_topics_get_backlog_size_by_message_id**](NonPersistentTopicApi.md#persistent_topics_get_backlog_size_by_message_id) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/backlogSize | Calculate backlog size by a message ID (in bytes).
[**persistent_topics_get_compaction_threshold**](NonPersistentTopicApi.md#persistent_topics_get_compaction_threshold) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/compactionThreshold | Get compaction threshold configuration for specified topic.
[**persistent_topics_get_deduplication**](NonPersistentTopicApi.md#persistent_topics_get_deduplication) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/deduplicationEnabled | Get deduplication configuration of a topic.
[**persistent_topics_get_deduplication_snapshot_interval**](NonPersistentTopicApi.md#persistent_topics_get_deduplication_snapshot_interval) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/deduplicationSnapshotInterval | Get deduplicationSnapshotInterval config on a topic.
[**persistent_topics_get_delayed_delivery_policies**](NonPersistentTopicApi.md#persistent_topics_get_delayed_delivery_policies) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/delayedDelivery | Get delayed delivery messages config on a topic.
[**persistent_topics_get_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_get_dispatch_rate) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/dispatchRate | Get dispatch rate configuration for specified topic.
[**persistent_topics_get_dispatcher_pause_on_ack_state_persistent**](NonPersistentTopicApi.md#persistent_topics_get_dispatcher_pause_on_ack_state_persistent) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/dispatcherPauseOnAckStatePersistent | Get dispatcher pause on ack state persistent config on a topic.
[**persistent_topics_get_inactive_topic_policies**](NonPersistentTopicApi.md#persistent_topics_get_inactive_topic_policies) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/inactiveTopicPolicies | Get inactive topic policies on a topic.
[**persistent_topics_get_last_message_id**](NonPersistentTopicApi.md#persistent_topics_get_last_message_id) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/lastMessageId | Return the last commit message id of topic
[**persistent_topics_get_managed_ledger_info**](NonPersistentTopicApi.md#persistent_topics_get_managed_ledger_info) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/internal-info | Get the stored topic metadata.
[**persistent_topics_get_max_consumers**](NonPersistentTopicApi.md#persistent_topics_get_max_consumers) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/maxConsumers | Get maxConsumers config for specified topic.
[**persistent_topics_get_max_consumers_per_subscription**](NonPersistentTopicApi.md#persistent_topics_get_max_consumers_per_subscription) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/maxConsumersPerSubscription | Get max consumers per subscription configuration for specified topic.
[**persistent_topics_get_max_message_size**](NonPersistentTopicApi.md#persistent_topics_get_max_message_size) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/maxMessageSize | Get maxMessageSize config for specified topic.
[**persistent_topics_get_max_producers**](NonPersistentTopicApi.md#persistent_topics_get_max_producers) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/maxProducers | Get maxProducers config for specified topic.
[**persistent_topics_get_max_subscriptions_per_topic**](NonPersistentTopicApi.md#persistent_topics_get_max_subscriptions_per_topic) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/maxSubscriptionsPerTopic | Get maxSubscriptionsPerTopic config for specified topic.
[**persistent_topics_get_max_unacked_messages_on_consumer**](NonPersistentTopicApi.md#persistent_topics_get_max_unacked_messages_on_consumer) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/maxUnackedMessagesOnConsumer | Get max unacked messages per consumer config on a topic.
[**persistent_topics_get_max_unacked_messages_on_subscription**](NonPersistentTopicApi.md#persistent_topics_get_max_unacked_messages_on_subscription) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/maxUnackedMessagesOnSubscription | Get max unacked messages per subscription config on a topic.
[**persistent_topics_get_message_by_id**](NonPersistentTopicApi.md#persistent_topics_get_message_by_id) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/ledger/{ledgerId}/entry/{entryId} | Get message by its messageId.
[**persistent_topics_get_message_id_by_timestamp**](NonPersistentTopicApi.md#persistent_topics_get_message_id_by_timestamp) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/messageid/{timestamp} | Get message ID published at or just after this absolute timestamp (in ms).
[**persistent_topics_get_message_ttl**](NonPersistentTopicApi.md#persistent_topics_get_message_ttl) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/messageTTL | Get message TTL in seconds for a topic
[**persistent_topics_get_offload_policies**](NonPersistentTopicApi.md#persistent_topics_get_offload_policies) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/offloadPolicies | Get offload policies on a topic.
[**persistent_topics_get_partitioned_stats_internal**](NonPersistentTopicApi.md#persistent_topics_get_partitioned_stats_internal) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/partitioned-internalStats | Get the stats-internal for the partitioned topic.
[**persistent_topics_get_partitioned_topic_list**](NonPersistentTopicApi.md#persistent_topics_get_partitioned_topic_list) | **GET** /non-persistent/{tenant}/{namespace}/partitioned | Get the list of partitioned topics under a namespace.
[**persistent_topics_get_permissions_on_topic**](NonPersistentTopicApi.md#persistent_topics_get_permissions_on_topic) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/permissions | Get permissions on a topic.
[**persistent_topics_get_persistence**](NonPersistentTopicApi.md#persistent_topics_get_persistence) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/persistence | Get configuration of persistence policies for specified topic.
[**persistent_topics_get_properties**](NonPersistentTopicApi.md#persistent_topics_get_properties) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/properties | Get topic properties.
[**persistent_topics_get_publish_rate**](NonPersistentTopicApi.md#persistent_topics_get_publish_rate) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/publishRate | Get publish rate configuration for specified topic.
[**persistent_topics_get_replicated_subscription_status**](NonPersistentTopicApi.md#persistent_topics_get_replicated_subscription_status) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/replicatedSubscriptionStatus | Get replicated subscription status on a topic.
[**persistent_topics_get_replication_clusters**](NonPersistentTopicApi.md#persistent_topics_get_replication_clusters) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/replication | Get the replication clusters for a topic
[**persistent_topics_get_replicator_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_get_replicator_dispatch_rate) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/replicatorDispatchRate | Get replicatorDispatchRate config for specified topic.
[**persistent_topics_get_retention**](NonPersistentTopicApi.md#persistent_topics_get_retention) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/retention | Get retention configuration for specified topic.
[**persistent_topics_get_schema_compatibility_strategy**](NonPersistentTopicApi.md#persistent_topics_get_schema_compatibility_strategy) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/schemaCompatibilityStrategy | Get schema compatibility strategy on a topic
[**persistent_topics_get_schema_validation_enforced**](NonPersistentTopicApi.md#persistent_topics_get_schema_validation_enforced) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/schemaValidationEnforced | Get schema validation enforced flag for topic.
[**persistent_topics_get_shadow_topics**](NonPersistentTopicApi.md#persistent_topics_get_shadow_topics) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/shadowTopics | Get the shadow topic list for a topic
[**persistent_topics_get_stats**](NonPersistentTopicApi.md#persistent_topics_get_stats) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/stats | Get the stats for the topic.
[**persistent_topics_get_subscribe_rate**](NonPersistentTopicApi.md#persistent_topics_get_subscribe_rate) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/subscribeRate | Get subscribe rate configuration for specified topic.
[**persistent_topics_get_subscription_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_get_subscription_dispatch_rate) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/subscriptionDispatchRate | Get subscription message dispatch rate configuration for specified topic.
[**persistent_topics_get_subscription_level_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_get_subscription_level_dispatch_rate) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/{subName}/dispatchRate | Get message dispatch rate configuration for specified subscription.
[**persistent_topics_get_subscription_properties**](NonPersistentTopicApi.md#persistent_topics_get_subscription_properties) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/properties | Return all the properties on the given subscription
[**persistent_topics_get_subscription_types_enabled**](NonPersistentTopicApi.md#persistent_topics_get_subscription_types_enabled) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/subscriptionTypesEnabled | Get is enable sub type fors specified topic.
[**persistent_topics_get_subscriptions**](NonPersistentTopicApi.md#persistent_topics_get_subscriptions) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/subscriptions | Get the list of persistent subscriptions for a given topic.
[**persistent_topics_grant_permissions_on_topic**](NonPersistentTopicApi.md#persistent_topics_grant_permissions_on_topic) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/permissions/{role} | Grant a new permission to a role on a single topic.
[**persistent_topics_offload_status**](NonPersistentTopicApi.md#persistent_topics_offload_status) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/offload | Offload a prefix of a topic to long term storage
[**persistent_topics_peek_nth_message**](NonPersistentTopicApi.md#persistent_topics_peek_nth_message) | **GET** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/position/{messagePosition} | Peek nth message on a topic subscription.
[**persistent_topics_remove_auto_subscription_creation**](NonPersistentTopicApi.md#persistent_topics_remove_auto_subscription_creation) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/autoSubscriptionCreation | Remove autoSubscriptionCreation ina a topic.
[**persistent_topics_remove_backlog_quota**](NonPersistentTopicApi.md#persistent_topics_remove_backlog_quota) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/backlogQuota | Remove a backlog quota policy from a topic.
[**persistent_topics_remove_compaction_threshold**](NonPersistentTopicApi.md#persistent_topics_remove_compaction_threshold) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/compactionThreshold | Remove compaction threshold configuration for specified topic.
[**persistent_topics_remove_deduplication**](NonPersistentTopicApi.md#persistent_topics_remove_deduplication) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/deduplicationEnabled | Remove deduplication configuration for specified topic.
[**persistent_topics_remove_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_remove_dispatch_rate) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/dispatchRate | Remove message dispatch rate configuration for specified topic.
[**persistent_topics_remove_dispatcher_pause_on_ack_state_persistent**](NonPersistentTopicApi.md#persistent_topics_remove_dispatcher_pause_on_ack_state_persistent) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/dispatcherPauseOnAckStatePersistent | Remove dispatcher pause on ack state persistent configuration for specified topic.
[**persistent_topics_remove_max_consumers**](NonPersistentTopicApi.md#persistent_topics_remove_max_consumers) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/maxConsumers | Remove maxConsumers config for specified topic.
[**persistent_topics_remove_max_consumers_per_subscription**](NonPersistentTopicApi.md#persistent_topics_remove_max_consumers_per_subscription) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/maxConsumersPerSubscription | Remove max consumers per subscription configuration for specified topic.
[**persistent_topics_remove_max_message_size**](NonPersistentTopicApi.md#persistent_topics_remove_max_message_size) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/maxMessageSize | Remove maxMessageSize config for specified topic.
[**persistent_topics_remove_max_producers**](NonPersistentTopicApi.md#persistent_topics_remove_max_producers) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/maxProducers | Remove maxProducers config for specified topic.
[**persistent_topics_remove_max_subscriptions_per_topic**](NonPersistentTopicApi.md#persistent_topics_remove_max_subscriptions_per_topic) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/maxSubscriptionsPerTopic | Remove maxSubscriptionsPerTopic config for specified topic.
[**persistent_topics_remove_message_ttl**](NonPersistentTopicApi.md#persistent_topics_remove_message_ttl) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/messageTTL | Remove message TTL in seconds for a topic
[**persistent_topics_remove_offload_policies**](NonPersistentTopicApi.md#persistent_topics_remove_offload_policies) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/offloadPolicies | Delete offload policies on a topic.
[**persistent_topics_remove_persistence**](NonPersistentTopicApi.md#persistent_topics_remove_persistence) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/persistence | Remove configuration of persistence policies for specified topic.
[**persistent_topics_remove_properties**](NonPersistentTopicApi.md#persistent_topics_remove_properties) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/properties | Remove the key in properties on the given topic.
[**persistent_topics_remove_publish_rate**](NonPersistentTopicApi.md#persistent_topics_remove_publish_rate) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/publishRate | Remove message publish rate configuration for specified topic.
[**persistent_topics_remove_replication_clusters**](NonPersistentTopicApi.md#persistent_topics_remove_replication_clusters) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/replication | Remove the replication clusters from a topic.
[**persistent_topics_remove_replicator_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_remove_replicator_dispatch_rate) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/replicatorDispatchRate | Remove replicatorDispatchRate config for specified topic.
[**persistent_topics_remove_retention**](NonPersistentTopicApi.md#persistent_topics_remove_retention) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/retention | Remove retention configuration for specified topic.
[**persistent_topics_remove_schema_compatibility_strategy**](NonPersistentTopicApi.md#persistent_topics_remove_schema_compatibility_strategy) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/schemaCompatibilityStrategy | Remove schema compatibility strategy on a topic
[**persistent_topics_remove_subscribe_rate**](NonPersistentTopicApi.md#persistent_topics_remove_subscribe_rate) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/subscribeRate | Remove subscribe rate configuration for specified topic.
[**persistent_topics_remove_subscription_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_remove_subscription_dispatch_rate) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/subscriptionDispatchRate | Remove subscription message dispatch rate configuration for specified topic.
[**persistent_topics_remove_subscription_level_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_remove_subscription_level_dispatch_rate) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/{subName}/dispatchRate | Remove message dispatch rate configuration for specified subscription.
[**persistent_topics_remove_subscription_types_enabled**](NonPersistentTopicApi.md#persistent_topics_remove_subscription_types_enabled) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/subscriptionTypesEnabled | Remove subscription types enabled for specified topic.
[**persistent_topics_reset_cursor**](NonPersistentTopicApi.md#persistent_topics_reset_cursor) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/resetcursor/{timestamp} | Reset subscription to message position closest to absolute timestamp (in ms).
[**persistent_topics_reset_cursor_on_position**](NonPersistentTopicApi.md#persistent_topics_reset_cursor_on_position) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/resetcursor | Reset subscription to message position closest to given position.
[**persistent_topics_revoke_permissions_on_topic**](NonPersistentTopicApi.md#persistent_topics_revoke_permissions_on_topic) | **DELETE** /non-persistent/{tenant}/{namespace}/{topic}/permissions/{role} | Revoke permissions on a topic.
[**persistent_topics_set_auto_subscription_creation**](NonPersistentTopicApi.md#persistent_topics_set_auto_subscription_creation) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/autoSubscriptionCreation | Override namespace's allowAutoSubscriptionCreation setting for a topic
[**persistent_topics_set_backlog_quota**](NonPersistentTopicApi.md#persistent_topics_set_backlog_quota) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/backlogQuota | Set a backlog quota for a topic.
[**persistent_topics_set_compaction_threshold**](NonPersistentTopicApi.md#persistent_topics_set_compaction_threshold) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/compactionThreshold | Set compaction threshold configuration for specified topic.
[**persistent_topics_set_deduplication**](NonPersistentTopicApi.md#persistent_topics_set_deduplication) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/deduplicationEnabled | Set deduplication enabled on a topic.
[**persistent_topics_set_deduplication_snapshot_interval**](NonPersistentTopicApi.md#persistent_topics_set_deduplication_snapshot_interval) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/deduplicationSnapshotInterval | Set deduplicationSnapshotInterval config on a topic.
[**persistent_topics_set_delayed_delivery_policies**](NonPersistentTopicApi.md#persistent_topics_set_delayed_delivery_policies) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/delayedDelivery | Set delayed delivery messages config on a topic.
[**persistent_topics_set_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_set_dispatch_rate) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/dispatchRate | Set message dispatch rate configuration for specified topic.
[**persistent_topics_set_dispatcher_pause_on_ack_state_persistent**](NonPersistentTopicApi.md#persistent_topics_set_dispatcher_pause_on_ack_state_persistent) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/dispatcherPauseOnAckStatePersistent | Set dispatcher pause on ack state persistent configuration for specified topic.
[**persistent_topics_set_inactive_topic_policies**](NonPersistentTopicApi.md#persistent_topics_set_inactive_topic_policies) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/inactiveTopicPolicies | Set inactive topic policies on a topic.
[**persistent_topics_set_max_consumers**](NonPersistentTopicApi.md#persistent_topics_set_max_consumers) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/maxConsumers | Set maxConsumers config for specified topic.
[**persistent_topics_set_max_consumers_per_subscription**](NonPersistentTopicApi.md#persistent_topics_set_max_consumers_per_subscription) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/maxConsumersPerSubscription | Set max consumers per subscription configuration for specified topic.
[**persistent_topics_set_max_message_size**](NonPersistentTopicApi.md#persistent_topics_set_max_message_size) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/maxMessageSize | Set maxMessageSize config for specified topic.
[**persistent_topics_set_max_producers**](NonPersistentTopicApi.md#persistent_topics_set_max_producers) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/maxProducers | Set maxProducers config for specified topic.
[**persistent_topics_set_max_subscriptions_per_topic**](NonPersistentTopicApi.md#persistent_topics_set_max_subscriptions_per_topic) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/maxSubscriptionsPerTopic | Set maxSubscriptionsPerTopic config for specified topic.
[**persistent_topics_set_max_unacked_messages_on_consumer**](NonPersistentTopicApi.md#persistent_topics_set_max_unacked_messages_on_consumer) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/maxUnackedMessagesOnConsumer | Set max unacked messages per consumer config on a topic.
[**persistent_topics_set_max_unacked_messages_on_subscription**](NonPersistentTopicApi.md#persistent_topics_set_max_unacked_messages_on_subscription) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/maxUnackedMessagesOnSubscription | Set max unacked messages per subscription config on a topic.
[**persistent_topics_set_message_ttl**](NonPersistentTopicApi.md#persistent_topics_set_message_ttl) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/messageTTL | Set message TTL in seconds for a topic
[**persistent_topics_set_offload_policies**](NonPersistentTopicApi.md#persistent_topics_set_offload_policies) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/offloadPolicies | Set offload policies on a topic.
[**persistent_topics_set_persistence**](NonPersistentTopicApi.md#persistent_topics_set_persistence) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/persistence | Set configuration of persistence policies for specified topic.
[**persistent_topics_set_publish_rate**](NonPersistentTopicApi.md#persistent_topics_set_publish_rate) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/publishRate | Set message publish rate configuration for specified topic.
[**persistent_topics_set_replicated_subscription_status**](NonPersistentTopicApi.md#persistent_topics_set_replicated_subscription_status) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/replicatedSubscriptionStatus | Enable or disable a replicated subscription on a topic.
[**persistent_topics_set_replication_clusters**](NonPersistentTopicApi.md#persistent_topics_set_replication_clusters) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/replication | Set the replication clusters for a topic.
[**persistent_topics_set_replicator_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_set_replicator_dispatch_rate) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/replicatorDispatchRate | Set replicatorDispatchRate config for specified topic.
[**persistent_topics_set_retention**](NonPersistentTopicApi.md#persistent_topics_set_retention) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/retention | Set retention configuration for specified topic.
[**persistent_topics_set_schema_compatibility_strategy**](NonPersistentTopicApi.md#persistent_topics_set_schema_compatibility_strategy) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/schemaCompatibilityStrategy | Set schema compatibility strategy on a topic
[**persistent_topics_set_schema_validation_enforced**](NonPersistentTopicApi.md#persistent_topics_set_schema_validation_enforced) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/schemaValidationEnforced | Set schema validation enforced flag on topic.
[**persistent_topics_set_shadow_topics**](NonPersistentTopicApi.md#persistent_topics_set_shadow_topics) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/shadowTopics | Set shadow topic list for a topic
[**persistent_topics_set_subscribe_rate**](NonPersistentTopicApi.md#persistent_topics_set_subscribe_rate) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscribeRate | Set subscribe rate configuration for specified topic.
[**persistent_topics_set_subscription_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_set_subscription_dispatch_rate) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscriptionDispatchRate | Set subscription message dispatch rate configuration for specified topic.
[**persistent_topics_set_subscription_level_dispatch_rate**](NonPersistentTopicApi.md#persistent_topics_set_subscription_level_dispatch_rate) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/{subName}/dispatchRate | Set message dispatch rate configuration for specified subscription.
[**persistent_topics_set_subscription_types_enabled**](NonPersistentTopicApi.md#persistent_topics_set_subscription_types_enabled) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscriptionTypesEnabled | Set is enable sub types for specified topic
[**persistent_topics_skip_all_messages**](NonPersistentTopicApi.md#persistent_topics_skip_all_messages) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/skip_all | Skip all messages on a topic subscription.
[**persistent_topics_skip_messages**](NonPersistentTopicApi.md#persistent_topics_skip_messages) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/skip/{numMessages} | Skipping messages on a topic subscription.
[**persistent_topics_terminate**](NonPersistentTopicApi.md#persistent_topics_terminate) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/terminate | Terminate a topic. A topic that is terminated will not accept any more messages to be published and will let consumer to drain existing messages in backlog
[**persistent_topics_terminate_partitioned_topic**](NonPersistentTopicApi.md#persistent_topics_terminate_partitioned_topic) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/terminate/partitions | Terminate all partitioned topic. A topic that is terminated will not accept any more messages to be published and will let consumer to drain existing messages in backlog
[**persistent_topics_trigger_offload**](NonPersistentTopicApi.md#persistent_topics_trigger_offload) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/offload | Offload a prefix of a topic to long term storage
[**persistent_topics_trim_topic**](NonPersistentTopicApi.md#persistent_topics_trim_topic) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/trim |  Trim a topic
[**persistent_topics_update_partitioned_topic**](NonPersistentTopicApi.md#persistent_topics_update_partitioned_topic) | **POST** /non-persistent/{tenant}/{namespace}/{topic}/partitions | Increment partitions of an existing partitioned topic.
[**persistent_topics_update_properties**](NonPersistentTopicApi.md#persistent_topics_update_properties) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/properties | Update the properties on the given topic.
[**persistent_topics_update_subscription_properties**](NonPersistentTopicApi.md#persistent_topics_update_subscription_properties) | **PUT** /non-persistent/{tenant}/{namespace}/{topic}/subscription/{subName}/properties | Replace all the properties on the given subscription



## non_persistent_topics_create_partitioned_topic

> non_persistent_topics_create_partitioned_topic(tenant, namespace, topic, body, create_local_topic_only)
Create a partitioned topic.

It needs to be called before creating a producer on a partitioned topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**body** | **i32** | The number of partitions for the topic | [required] |
**create_local_topic_only** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_get_entry_filters

> non_persistent_topics_get_entry_filters(tenant, namespace, topic, applied, is_global, authoritative)
Get entry filters for a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_get_internal_stats

> non_persistent_topics_get_internal_stats(tenant, namespace, topic, authoritative, metadata)
Get the internal stats for the topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**metadata** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_get_list

> Vec<String> non_persistent_topics_get_list(tenant, namespace, bundle, include_system_topic)
Get the list of non-persistent topics under a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**bundle** | Option<**String**> | Specify the bundle name |  |
**include_system_topic** | Option<**bool**> | Include system topic |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_get_list_from_bundle

> Vec<String> non_persistent_topics_get_list_from_bundle(tenant, namespace, bundle)
Get the list of non-persistent topics under a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**bundle** | **String** | Bundle range of a topic | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_get_partitioned_metadata

> non_persistent_topics_get_partitioned_metadata(tenant, namespace, topic, authoritative, check_allow_auto_creation)
Get partitioned topic metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**check_allow_auto_creation** | Option<**bool**> | Is check configuration required to automatically create topic |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_get_partitioned_stats

> models::NonPersistentPartitionedTopicStatsImpl non_persistent_topics_get_partitioned_stats(tenant, namespace, topic, per_partition, authoritative, get_precise_backlog, subscription_backlog_size, get_earliest_time_in_backlog, exclude_publishers, exclude_consumers)
Get the stats for the partitioned topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**per_partition** | Option<**bool**> | Get per partition stats |  |[default to true]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**get_precise_backlog** | Option<**bool**> | If return precise backlog or imprecise backlog |  |[default to false]
**subscription_backlog_size** | Option<**bool**> | If return backlog size for each subscription, require locking on ledger so be careful not to use when there's heavy traffic. |  |[default to false]
**get_earliest_time_in_backlog** | Option<**bool**> | If return the earliest time in backlog |  |[default to false]
**exclude_publishers** | Option<**bool**> | If exclude the publishers |  |[default to false]
**exclude_consumers** | Option<**bool**> | If exclude the consumers |  |[default to false]

### Return type

[**models::NonPersistentPartitionedTopicStatsImpl**](NonPersistentPartitionedTopicStatsImpl.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_remove_entry_filters

> non_persistent_topics_remove_entry_filters(tenant, namespace, topic, is_global, authoritative)
Remove entry filters for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected thiscall to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_set_entry_filters

> non_persistent_topics_set_entry_filters(tenant, namespace, topic, is_global, authoritative, body)
Set entry filters for specified topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**EntryFilters**](EntryFilters.md)> | Enable sub types for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_truncate_topic

> non_persistent_topics_truncate_topic(tenant, namespace, topic, authoritative)
Truncate a topic.

NonPersistentTopic does not support truncate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## non_persistent_topics_unload_topic

> non_persistent_topics_unload_topic(tenant, namespace, topic, authoritative)
Unload a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_analyze_subscription_backlog

> persistent_topics_analyze_subscription_backlog(tenant, namespace, topic, sub_name, authoritative, position)
Analyse a subscription, by scanning all the unprocessed messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscription | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**position** | Option<[**ResetCursorData**](ResetCursorData.md)> | messageId to start the analysis |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_compact

> persistent_topics_compact(tenant, namespace, topic, authoritative)
Trigger a compaction operation on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_compaction_status

> persistent_topics_compaction_status(tenant, namespace, topic, authoritative)
Get the status of a compaction operation for a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_create_missed_partitions

> persistent_topics_create_missed_partitions(tenant, namespace, topic)
Create missed partitions of an existing partitioned topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_create_non_partitioned_topic

> persistent_topics_create_non_partitioned_topic(tenant, namespace, topic, authoritative, body)
Create a non-partitioned topic.

This is the only REST endpoint from which non-partitioned topics could be created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**std::collections::HashMap<String, String>**](String.md)> | Key value pair properties for the topic metadata |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_create_subscription

> persistent_topics_create_subscription(tenant, namespace, topic, subscription_name, authoritative, replicated, message_id)
Create a subscription on the topic.

Creates a subscription on the topic at the specified message id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**subscription_name** | **String** | Name of subscription to be created | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**replicated** | Option<**bool**> | Is replicated required to perform this operation |  |
**message_id** | Option<[**ResetCursorData**](ResetCursorData.md)> | messageId where to create the subscription. It can be 'latest', 'earliest' or (ledgerId:entryId) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_deduplication_snapshot_interval

> persistent_topics_delete_deduplication_snapshot_interval(tenant, namespace, topic, is_global, authoritative)
Delete deduplicationSnapshotInterval config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_delayed_delivery_policies

> persistent_topics_delete_delayed_delivery_policies(tenant, namespace, topic, is_global, authoritative)
Set delayed delivery messages config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_inactive_topic_policies

> persistent_topics_delete_inactive_topic_policies(tenant, namespace, topic, is_global, authoritative)
Delete inactive topic policies on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_max_unacked_messages_on_consumer

> persistent_topics_delete_max_unacked_messages_on_consumer(tenant, namespace, topic, is_global, authoritative)
Delete max unacked messages per consumer config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_max_unacked_messages_on_subscription

> persistent_topics_delete_max_unacked_messages_on_subscription(tenant, namespace, topic, is_global, authoritative)
Delete max unacked messages per subscription config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_partitioned_topic

> persistent_topics_delete_partitioned_topic(tenant, namespace, topic, force, authoritative)
Delete a partitioned topic.

It will also delete all the partitions of the topic if it exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**force** | Option<**bool**> | Stop all producer/consumer/replicator and delete topic forcefully |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_shadow_topics

> persistent_topics_delete_shadow_topics(tenant, namespace, topic, authoritative)
Delete shadow topics for a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_subscription

> persistent_topics_delete_subscription(tenant, namespace, topic, sub_name, force, authoritative)
Delete a subscription.

The subscription cannot be deleted if delete is not forcefully and there are any active consumers attached to it. Force delete ignores connected consumers and deletes subscription by explicitly closing them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscription to be deleted | [required] |
**force** | Option<**bool**> | Disconnect and close all consumers and delete subscription forcefully |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_delete_topic

> persistent_topics_delete_topic(tenant, namespace, topic, force, authoritative)
Delete a topic.

The topic cannot be deleted if delete is not forcefully and there's any active subscription or producer connected to the it. Force delete ignores connected clients and deletes topic by explicitly closing them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**force** | Option<**bool**> | Stop all producer/consumer/replicator and delete topic forcefully |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_examine_message

> persistent_topics_examine_message(tenant, namespace, topic, initial_position, message_position, authoritative)
Examine a specific message on a topic by position relative to the earliest or the latest message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**initial_position** | Option<**String**> | Relative start position to examine message.It can be 'latest' or 'earliest' |  |[default to latest]
**message_position** | Option<**i64**> | The position of messages (default 1) |  |[default to 1]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_expire_messages_for_all_subscriptions

> persistent_topics_expire_messages_for_all_subscriptions(tenant, namespace, topic, expire_time_in_seconds, authoritative)
Expiry messages on all subscriptions of topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**expire_time_in_seconds** | **i32** | Expires beyond the specified number of seconds | [required] |[default to 0]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_expire_topic_messages

> persistent_topics_expire_topic_messages(tenant, namespace, topic, sub_name, authoritative, message_id)
Expiry messages on a topic subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscription to be Expiry messages on | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**message_id** | Option<[**ResetCursorData**](ResetCursorData.md)> | messageId to reset back to (ledgerId:entryId) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_expire_topic_messages_0

> persistent_topics_expire_topic_messages_0(tenant, namespace, topic, sub_name, expire_time_in_seconds, authoritative)
Expiry messages on a topic subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscription to be Expiry messages on | [required] |
**expire_time_in_seconds** | **i32** | Expires beyond the specified number of seconds | [required] |[default to 0]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_auto_subscription_creation

> persistent_topics_get_auto_subscription_creation(tenant, namespace, topic, applied, is_global, authoritative)
Get autoSubscriptionCreation info in a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_backlog

> models::PersistentOfflineTopicStats persistent_topics_get_backlog(tenant, namespace, topic, authoritative)
Get estimated backlog for offline topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::PersistentOfflineTopicStats**](PersistentOfflineTopicStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_backlog_quota_map

> std::collections::HashMap<String, models::BacklogQuota> persistent_topics_get_backlog_quota_map(tenant, namespace, topic, applied, authoritative, is_global)
Get backlog quota map on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]

### Return type

[**std::collections::HashMap<String, models::BacklogQuota>**](BacklogQuota.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_backlog_size_by_message_id

> i64 persistent_topics_get_backlog_size_by_message_id(tenant, namespace, topic, authoritative)
Calculate backlog size by a message ID (in bytes).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_compaction_threshold

> i64 persistent_topics_get_compaction_threshold(tenant, namespace, topic, applied, is_global, authoritative)
Get compaction threshold configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_deduplication

> bool persistent_topics_get_deduplication(tenant, namespace, topic, applied, is_global, authoritative)
Get deduplication configuration of a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_deduplication_snapshot_interval

> i32 persistent_topics_get_deduplication_snapshot_interval(tenant, namespace, topic, is_global, authoritative)
Get deduplicationSnapshotInterval config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_delayed_delivery_policies

> models::DelayedDeliveryPolicies persistent_topics_get_delayed_delivery_policies(tenant, namespace, topic, is_global, applied, authoritative)
Get delayed delivery messages config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**applied** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::DelayedDeliveryPolicies**](DelayedDeliveryPolicies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_dispatch_rate

> persistent_topics_get_dispatch_rate(tenant, namespace, topic, applied, is_global, authoritative)
Get dispatch rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_dispatcher_pause_on_ack_state_persistent

> i32 persistent_topics_get_dispatcher_pause_on_ack_state_persistent(tenant, namespace, topic, applied, is_global, authoritative)
Get dispatcher pause on ack state persistent config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_inactive_topic_policies

> models::InactiveTopicPolicies persistent_topics_get_inactive_topic_policies(tenant, namespace, topic, applied, is_global, authoritative)
Get inactive topic policies on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::InactiveTopicPolicies**](InactiveTopicPolicies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_last_message_id

> persistent_topics_get_last_message_id(tenant, namespace, topic, authoritative)
Return the last commit message id of topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_managed_ledger_info

> models::PartitionedManagedLedgerInfo persistent_topics_get_managed_ledger_info(tenant, namespace, topic, authoritative)
Get the stored topic metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::PartitionedManagedLedgerInfo**](PartitionedManagedLedgerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_max_consumers

> i32 persistent_topics_get_max_consumers(tenant, namespace, topic, is_global, applied, authoritative)
Get maxConsumers config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**applied** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_max_consumers_per_subscription

> i32 persistent_topics_get_max_consumers_per_subscription(tenant, namespace, topic, is_global, authoritative)
Get max consumers per subscription configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_max_message_size

> i32 persistent_topics_get_max_message_size(tenant, namespace, topic, is_global, authoritative)
Get maxMessageSize config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_max_producers

> i32 persistent_topics_get_max_producers(tenant, namespace, topic, applied, is_global, authoritative)
Get maxProducers config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_max_subscriptions_per_topic

> i32 persistent_topics_get_max_subscriptions_per_topic(tenant, namespace, topic, is_global, authoritative)
Get maxSubscriptionsPerTopic config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_max_unacked_messages_on_consumer

> i32 persistent_topics_get_max_unacked_messages_on_consumer(tenant, namespace, topic, applied, is_global, authoritative)
Get max unacked messages per consumer config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_max_unacked_messages_on_subscription

> i32 persistent_topics_get_max_unacked_messages_on_subscription(tenant, namespace, topic, applied, is_global, authoritative)
Get max unacked messages per subscription config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_message_by_id

> persistent_topics_get_message_by_id(tenant, namespace, topic, ledger_id, entry_id, authoritative)
Get message by its messageId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**ledger_id** | **i64** | The ledger id | [required] |
**entry_id** | **i64** | The entry id | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_message_id_by_timestamp

> persistent_topics_get_message_id_by_timestamp(tenant, namespace, topic, timestamp, authoritative)
Get message ID published at or just after this absolute timestamp (in ms).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**timestamp** | **i64** | Specify the timestamp | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_message_ttl

> i32 persistent_topics_get_message_ttl(tenant, namespace, topic, applied, is_global, authoritative)
Get message TTL in seconds for a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_offload_policies

> persistent_topics_get_offload_policies(tenant, namespace, topic, applied, is_global, authoritative)
Get offload policies on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_partitioned_stats_internal

> models::PartitionedTopicInternalStats persistent_topics_get_partitioned_stats_internal(tenant, namespace, topic, authoritative)
Get the stats-internal for the partitioned topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::PartitionedTopicInternalStats**](PartitionedTopicInternalStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_partitioned_topic_list

> Vec<String> persistent_topics_get_partitioned_topic_list(tenant, namespace, include_system_topic)
Get the list of partitioned topics under a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**include_system_topic** | Option<**bool**> | Include system topic |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_permissions_on_topic

> persistent_topics_get_permissions_on_topic(tenant, namespace, topic)
Get permissions on a topic.

Retrieve the effective permissions for a topic. These permissions are defined by the permissions set at thenamespace level combined (union) with any eventual specific permission set on the topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_persistence

> models::PersistencePolicies persistent_topics_get_persistence(tenant, namespace, topic, applied, is_global, authoritative)
Get configuration of persistence policies for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::PersistencePolicies**](PersistencePolicies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_properties

> std::collections::HashMap<String, String> persistent_topics_get_properties(tenant, namespace, topic, authoritative)
Get topic properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_publish_rate

> models::PublishRate persistent_topics_get_publish_rate(tenant, namespace, topic, is_global, authoritative)
Get publish rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::PublishRate**](PublishRate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_replicated_subscription_status

> std::collections::HashMap<String, bool> persistent_topics_get_replicated_subscription_status(tenant, namespace, topic, sub_name, authoritative)
Get replicated subscription status on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Name of subscription | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**std::collections::HashMap<String, bool>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_replication_clusters

> Vec<String> persistent_topics_get_replication_clusters(tenant, namespace, topic, applied, authoritative)
Get the replication clusters for a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_replicator_dispatch_rate

> models::DispatchRate persistent_topics_get_replicator_dispatch_rate(tenant, namespace, topic, is_global, applied, authoritative)
Get replicatorDispatchRate config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**applied** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::DispatchRate**](DispatchRate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_retention

> models::RetentionPolicies persistent_topics_get_retention(tenant, namespace, topic, is_global, applied, authoritative)
Get retention configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**applied** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::RetentionPolicies**](RetentionPolicies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_schema_compatibility_strategy

> String persistent_topics_get_schema_compatibility_strategy(tenant, namespace, topic, applied, authoritative)
Get schema compatibility strategy on a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the cluster | [required] |
**topic** | **String** | Specify topic name | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_schema_validation_enforced

> persistent_topics_get_schema_validation_enforced(tenant, namespace, topic, applied, authoritative)
Get schema validation enforced flag for topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_shadow_topics

> persistent_topics_get_shadow_topics(tenant, namespace, topic, authoritative)
Get the shadow topic list for a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_stats

> models::PersistentTopicStats persistent_topics_get_stats(tenant, namespace, topic, authoritative, get_precise_backlog, subscription_backlog_size, get_earliest_time_in_backlog, exclude_publishers, exclude_consumers)
Get the stats for the topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**get_precise_backlog** | Option<**bool**> | If return precise backlog or imprecise backlog |  |[default to false]
**subscription_backlog_size** | Option<**bool**> | If return backlog size for each subscription, require locking on ledger so be careful not to use when there's heavy traffic. |  |[default to true]
**get_earliest_time_in_backlog** | Option<**bool**> | If return time of the earliest message in backlog |  |[default to false]
**exclude_publishers** | Option<**bool**> | If exclude the publishers |  |[default to false]
**exclude_consumers** | Option<**bool**> | If exclude the consumers |  |[default to false]

### Return type

[**models::PersistentTopicStats**](PersistentTopicStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_subscribe_rate

> models::SubscribeRate persistent_topics_get_subscribe_rate(tenant, namespace, topic, applied, is_global, authoritative)
Get subscribe rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::SubscribeRate**](SubscribeRate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_subscription_dispatch_rate

> models::DispatchRate persistent_topics_get_subscription_dispatch_rate(tenant, namespace, topic, applied, is_global, authoritative)
Get subscription message dispatch rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

[**models::DispatchRate**](DispatchRate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_subscription_level_dispatch_rate

> persistent_topics_get_subscription_level_dispatch_rate(tenant, namespace, topic, sub_name, applied, is_global, authoritative)
Get message dispatch rate configuration for specified subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**sub_name** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_subscription_properties

> persistent_topics_get_subscription_properties(tenant, namespace, topic, sub_name, authoritative)
Return all the properties on the given subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscription | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_subscription_types_enabled

> Vec<String> persistent_topics_get_subscription_types_enabled(tenant, namespace, topic, is_global, authoritative)
Get is enable sub type fors specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_get_subscriptions

> Vec<String> persistent_topics_get_subscriptions(tenant, namespace, topic, authoritative)
Get the list of persistent subscriptions for a given topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_grant_permissions_on_topic

> persistent_topics_grant_permissions_on_topic(tenant, namespace, topic, role, body)
Grant a new permission to a role on a single topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**role** | **String** | Client role to which grant permissions | [required] |
**body** | Option<[**Vec<String>**](String.md)> | Actions to be granted (produce,functions,consume) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_offload_status

> persistent_topics_offload_status(tenant, namespace, topic, authoritative)
Offload a prefix of a topic to long term storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_peek_nth_message

> persistent_topics_peek_nth_message(tenant, namespace, topic, sub_name, message_position, authoritative)
Peek nth message on a topic subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscribed message expired | [required] |
**message_position** | **i32** | The number of messages (default 1) | [required] |[default to 1]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_auto_subscription_creation

> persistent_topics_remove_auto_subscription_creation(tenant, namespace, topic, is_global, authoritative)
Remove autoSubscriptionCreation ina a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_backlog_quota

> persistent_topics_remove_backlog_quota(tenant, namespace, topic, backlog_quota_type, authoritative, is_global)
Remove a backlog quota policy from a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**backlog_quota_type** | Option<**String**> |  |  |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_compaction_threshold

> persistent_topics_remove_compaction_threshold(tenant, namespace, topic, is_global, authoritative)
Remove compaction threshold configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_deduplication

> persistent_topics_remove_deduplication(tenant, namespace, topic, is_global, authoritative)
Remove deduplication configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_dispatch_rate

> persistent_topics_remove_dispatch_rate(tenant, namespace, topic, is_global, authoritative)
Remove message dispatch rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_dispatcher_pause_on_ack_state_persistent

> persistent_topics_remove_dispatcher_pause_on_ack_state_persistent(tenant, namespace, topic, is_global, authoritative)
Remove dispatcher pause on ack state persistent configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_max_consumers

> persistent_topics_remove_max_consumers(tenant, namespace, topic, is_global, authoritative)
Remove maxConsumers config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_max_consumers_per_subscription

> persistent_topics_remove_max_consumers_per_subscription(tenant, namespace, topic, is_global, authoritative)
Remove max consumers per subscription configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_max_message_size

> persistent_topics_remove_max_message_size(tenant, namespace, topic, is_global, authoritative)
Remove maxMessageSize config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_max_producers

> persistent_topics_remove_max_producers(tenant, namespace, topic, is_global, authoritative)
Remove maxProducers config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_max_subscriptions_per_topic

> persistent_topics_remove_max_subscriptions_per_topic(tenant, namespace, topic, is_global, authoritative)
Remove maxSubscriptionsPerTopic config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_message_ttl

> persistent_topics_remove_message_ttl(tenant, namespace, topic, is_global, authoritative)
Remove message TTL in seconds for a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_offload_policies

> persistent_topics_remove_offload_policies(tenant, namespace, topic, is_global, authoritative)
Delete offload policies on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_persistence

> persistent_topics_remove_persistence(tenant, namespace, topic, is_global, authoritative)
Remove configuration of persistence policies for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_properties

> persistent_topics_remove_properties(tenant, namespace, topic, key, authoritative)
Remove the key in properties on the given topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**key** | Option<**String**> |  |  |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_publish_rate

> persistent_topics_remove_publish_rate(tenant, namespace, topic, is_global, authoritative)
Remove message publish rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_replication_clusters

> persistent_topics_remove_replication_clusters(tenant, namespace, topic, authoritative)
Remove the replication clusters from a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_replicator_dispatch_rate

> persistent_topics_remove_replicator_dispatch_rate(tenant, namespace, topic, is_global, authoritative)
Remove replicatorDispatchRate config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_retention

> persistent_topics_remove_retention(tenant, namespace, topic, is_global, authoritative)
Remove retention configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_schema_compatibility_strategy

> persistent_topics_remove_schema_compatibility_strategy(tenant, namespace, topic, authoritative, body)
Remove schema compatibility strategy on a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**String**> | Strategy used to check the compatibility of new schema |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_subscribe_rate

> persistent_topics_remove_subscribe_rate(tenant, namespace, topic, is_global, authoritative, body)
Remove subscribe rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**SubscribeRate**](SubscribeRate.md)> | Subscribe rate for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_subscription_dispatch_rate

> persistent_topics_remove_subscription_dispatch_rate(tenant, namespace, topic, is_global, authoritative)
Remove subscription message dispatch rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_subscription_level_dispatch_rate

> persistent_topics_remove_subscription_level_dispatch_rate(tenant, namespace, topic, sub_name, is_global, authoritative)
Remove message dispatch rate configuration for specified subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**sub_name** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_remove_subscription_types_enabled

> persistent_topics_remove_subscription_types_enabled(tenant, namespace, topic, is_global, authoritative)
Remove subscription types enabled for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_reset_cursor

> persistent_topics_reset_cursor(tenant, namespace, topic, sub_name, timestamp, authoritative)
Reset subscription to message position closest to absolute timestamp (in ms).

It fence cursor and disconnects all active consumers before resetting cursor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscription to reset position on | [required] |
**timestamp** | **i64** | the timestamp to reset back | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_reset_cursor_on_position

> persistent_topics_reset_cursor_on_position(tenant, namespace, topic, sub_name, authoritative, message_id)
Reset subscription to message position closest to given position.

It fence cursor and disconnects all active consumers before resetting cursor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscription to reset position on | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**message_id** | Option<[**ResetCursorData**](ResetCursorData.md)> | messageId to reset back to (ledgerId:entryId) |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_revoke_permissions_on_topic

> persistent_topics_revoke_permissions_on_topic(tenant, namespace, topic, role)
Revoke permissions on a topic.

Revoke permissions to a role on a single topic. If the permission was not set at the topiclevel, but rather at the namespace level, this operation will return an error (HTTP status code 412).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**role** | **String** | Client role to which grant permissions | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_auto_subscription_creation

> persistent_topics_set_auto_subscription_creation(tenant, namespace, topic, is_global, authoritative, body)
Override namespace's allowAutoSubscriptionCreation setting for a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**AutoSubscriptionCreationOverrideImpl**](AutoSubscriptionCreationOverrideImpl.md)> | Settings for automatic subscription creation |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_backlog_quota

> persistent_topics_set_backlog_quota(tenant, namespace, topic, authoritative, is_global, backlog_quota_type, body)
Set a backlog quota for a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**backlog_quota_type** | Option<**String**> |  |  |
**body** | Option<[**BacklogQuotaImpl**](BacklogQuotaImpl.md)> | backlog quota policies for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_compaction_threshold

> persistent_topics_set_compaction_threshold(tenant, namespace, topic, authoritative, is_global, body)
Set compaction threshold configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<**i64**> | Dispatch rate for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_deduplication

> persistent_topics_set_deduplication(tenant, namespace, topic, is_global, authoritative, body)
Set deduplication enabled on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**bool**> | DeduplicationEnabled policies for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_deduplication_snapshot_interval

> persistent_topics_set_deduplication_snapshot_interval(tenant, namespace, topic, is_global, authoritative, body)
Set deduplicationSnapshotInterval config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**i32**> | Interval to take deduplication snapshot for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_delayed_delivery_policies

> persistent_topics_set_delayed_delivery_policies(tenant, namespace, topic, is_global, authoritative, body)
Set delayed delivery messages config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**DelayedDeliveryPolicies**](DelayedDeliveryPolicies.md)> | Delayed delivery policies for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_dispatch_rate

> persistent_topics_set_dispatch_rate(tenant, namespace, topic, authoritative, is_global, body)
Set message dispatch rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**DispatchRateImpl**](DispatchRateImpl.md)> | Dispatch rate for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_dispatcher_pause_on_ack_state_persistent

> persistent_topics_set_dispatcher_pause_on_ack_state_persistent(tenant, namespace, topic, authoritative, is_global)
Set dispatcher pause on ack state persistent configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_inactive_topic_policies

> persistent_topics_set_inactive_topic_policies(tenant, namespace, topic, authoritative, is_global, body)
Set inactive topic policies on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**InactiveTopicPolicies**](InactiveTopicPolicies.md)> | inactive topic policies for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_max_consumers

> persistent_topics_set_max_consumers(tenant, namespace, topic, is_global, authoritative, body)
Set maxConsumers config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**i32**> | The max consumers of the topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_max_consumers_per_subscription

> persistent_topics_set_max_consumers_per_subscription(tenant, namespace, topic, is_global, authoritative, body)
Set max consumers per subscription configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**i32**> | Dispatch rate for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_max_message_size

> persistent_topics_set_max_message_size(tenant, namespace, topic, is_global, authoritative, body)
Set maxMessageSize config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**i32**> | The max message size of the topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_max_producers

> persistent_topics_set_max_producers(tenant, namespace, topic, authoritative, is_global, body)
Set maxProducers config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<**i32**> | The max producers of the topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_max_subscriptions_per_topic

> persistent_topics_set_max_subscriptions_per_topic(tenant, namespace, topic, is_global, authoritative, body)
Set maxSubscriptionsPerTopic config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**i32**> | The max subscriptions of the topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_max_unacked_messages_on_consumer

> persistent_topics_set_max_unacked_messages_on_consumer(tenant, namespace, topic, is_global, authoritative, body)
Set max unacked messages per consumer config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**i32**> | Max unacked messages on consumer policies for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_max_unacked_messages_on_subscription

> persistent_topics_set_max_unacked_messages_on_subscription(tenant, namespace, topic, is_global, authoritative, body)
Set max unacked messages per subscription config on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**i32**> | Max unacked messages on subscription policies for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_message_ttl

> persistent_topics_set_message_ttl(tenant, namespace, topic, message_ttl, is_global, authoritative)
Set message TTL in seconds for a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**message_ttl** | **i32** | TTL in seconds for the specified topic | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_offload_policies

> persistent_topics_set_offload_policies(tenant, namespace, topic, authoritative, is_global, body)
Set offload policies on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**OffloadPoliciesImpl**](OffloadPoliciesImpl.md)> | Offload policies for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_persistence

> persistent_topics_set_persistence(tenant, namespace, topic, authoritative, is_global, body)
Set configuration of persistence policies for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**PersistencePolicies**](PersistencePolicies.md)> | Bookkeeper persistence policies for specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_publish_rate

> persistent_topics_set_publish_rate(tenant, namespace, topic, is_global, authoritative, body)
Set message publish rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**PublishRate**](PublishRate.md)> | Dispatch rate for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_replicated_subscription_status

> persistent_topics_set_replicated_subscription_status(tenant, namespace, topic, sub_name, body, authoritative)
Enable or disable a replicated subscription on a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Name of subscription | [required] |
**body** | **bool** | Whether to enable replicated subscription | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_replication_clusters

> persistent_topics_set_replication_clusters(tenant, namespace, topic, body, authoritative)
Set the replication clusters for a topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**body** | [**Vec<String>**](String.md) | List of replication clusters | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_replicator_dispatch_rate

> persistent_topics_set_replicator_dispatch_rate(tenant, namespace, topic, is_global, authoritative, body)
Set replicatorDispatchRate config for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**DispatchRateImpl**](DispatchRateImpl.md)> | Replicator dispatch rate of the topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_retention

> persistent_topics_set_retention(tenant, namespace, topic, authoritative, is_global, body)
Set retention configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**RetentionPolicies**](RetentionPolicies.md)> | Retention policies for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_schema_compatibility_strategy

> persistent_topics_set_schema_compatibility_strategy(tenant, namespace, topic, authoritative, body)
Set schema compatibility strategy on a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<**String**> | Strategy used to check the compatibility of new schema |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_schema_validation_enforced

> persistent_topics_set_schema_validation_enforced(tenant, namespace, topic, body, authoritative)
Set schema validation enforced flag on topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**body** | **bool** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_shadow_topics

> persistent_topics_set_shadow_topics(tenant, namespace, topic, body, authoritative)
Set shadow topic list for a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**body** | [**Vec<String>**](String.md) | List of shadow topics | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_subscribe_rate

> persistent_topics_set_subscribe_rate(tenant, namespace, topic, is_global, authoritative, body)
Set subscribe rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**SubscribeRate**](SubscribeRate.md)> | Subscribe rate for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_subscription_dispatch_rate

> persistent_topics_set_subscription_dispatch_rate(tenant, namespace, topic, authoritative, is_global, body)
Set subscription message dispatch rate configuration for specified topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**DispatchRateImpl**](DispatchRateImpl.md)> | Subscription message dispatch rate for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_subscription_level_dispatch_rate

> persistent_topics_set_subscription_level_dispatch_rate(tenant, namespace, topic, sub_name, authoritative, is_global, body)
Set message dispatch rate configuration for specified subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**sub_name** | **String** |  | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**is_global** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**DispatchRateImpl**](DispatchRateImpl.md)> | Subscription message dispatch rate for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_set_subscription_types_enabled

> persistent_topics_set_subscription_types_enabled(tenant, namespace, topic, is_global, authoritative, body)
Set is enable sub types for specified topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**is_global** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**Vec<String>**](String.md)> | Enable sub types for the specified topic |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_skip_all_messages

> persistent_topics_skip_all_messages(tenant, namespace, topic, sub_name, authoritative)
Skip all messages on a topic subscription.

Completely clears the backlog on the subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Name of subscription | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_skip_messages

> persistent_topics_skip_messages(tenant, namespace, topic, sub_name, num_messages, authoritative)
Skipping messages on a topic subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Name of subscription | [required] |
**num_messages** | **i32** | The number of messages to skip | [required] |[default to 0]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_terminate

> persistent_topics_terminate(tenant, namespace, topic, authoritative)
Terminate a topic. A topic that is terminated will not accept any more messages to be published and will let consumer to drain existing messages in backlog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_terminate_partitioned_topic

> persistent_topics_terminate_partitioned_topic(tenant, namespace, topic, authoritative)
Terminate all partitioned topic. A topic that is terminated will not accept any more messages to be published and will let consumer to drain existing messages in backlog

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_trigger_offload

> persistent_topics_trigger_offload(tenant, namespace, topic, authoritative)
Offload a prefix of a topic to long term storage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_trim_topic

> persistent_topics_trim_topic(tenant, namespace, topic, authoritative)
 Trim a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_update_partitioned_topic

> persistent_topics_update_partitioned_topic(tenant, namespace, topic, body, update_local_topic_only, authoritative, force)
Increment partitions of an existing partitioned topic.

It increments partitions of existing partitioned-topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**body** | **i32** | The number of partitions for the topic | [required] |
**update_local_topic_only** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**force** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_update_properties

> persistent_topics_update_properties(tenant, namespace, topic, authoritative, body)
Update the properties on the given topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**std::collections::HashMap<String, String>**](String.md)> | Key value pair properties for the topic metadata |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_topics_update_subscription_properties

> persistent_topics_update_subscription_properties(tenant, namespace, topic, sub_name, authoritative, body)
Replace all the properties on the given subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Specify the tenant | [required] |
**namespace** | **String** | Specify the namespace | [required] |
**topic** | **String** | Specify topic name | [required] |
**sub_name** | **String** | Subscription to update | [required] |
**authoritative** | Option<**bool**> | Whether leader broker redirected this call to this broker. For internal use. |  |[default to false]
**body** | Option<[**std::collections::HashMap<String, String>**](String.md)> | The new properties |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

