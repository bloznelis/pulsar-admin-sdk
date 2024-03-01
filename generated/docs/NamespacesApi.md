# \NamespacesApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**namespaces_clear_namespace_backlog**](NamespacesApi.md#namespaces_clear_namespace_backlog) | **POST** /namespaces/{tenant}/{namespace}/clearBacklog | Clear backlog for all topics on a namespace.
[**namespaces_clear_namespace_backlog_for_subscription**](NamespacesApi.md#namespaces_clear_namespace_backlog_for_subscription) | **POST** /namespaces/{tenant}/{namespace}/clearBacklog/{subscription} | Clear backlog for a given subscription on all topics on a namespace.
[**namespaces_clear_namespace_bundle_backlog**](NamespacesApi.md#namespaces_clear_namespace_bundle_backlog) | **POST** /namespaces/{tenant}/{namespace}/{bundle}/clearBacklog | Clear backlog for all topics on a namespace bundle.
[**namespaces_clear_namespace_bundle_backlog_for_subscription**](NamespacesApi.md#namespaces_clear_namespace_bundle_backlog_for_subscription) | **POST** /namespaces/{tenant}/{namespace}/{bundle}/clearBacklog/{subscription} | Clear backlog for a given subscription on all topics on a namespace bundle.
[**namespaces_clear_offload_deletion_lag**](NamespacesApi.md#namespaces_clear_offload_deletion_lag) | **DELETE** /namespaces/{tenant}/{namespace}/offloadDeletionLagMs | Clear the namespace configured offload deletion lag. The topics in the namespace will fallback to using the default configured deletion lag for the broker
[**namespaces_clear_properties**](NamespacesApi.md#namespaces_clear_properties) | **DELETE** /namespaces/{tenant}/{namespace}/properties | Clear properties on a given namespace.
[**namespaces_create_namespace**](NamespacesApi.md#namespaces_create_namespace) | **PUT** /namespaces/{tenant}/{namespace} | Creates a new namespace with the specified policies
[**namespaces_delete_bookie_affinity_group**](NamespacesApi.md#namespaces_delete_bookie_affinity_group) | **DELETE** /namespaces/{property}/{namespace}/persistence/bookieAffinity | Delete the bookie-affinity-group from namespace-local policy.
[**namespaces_delete_compaction_threshold**](NamespacesApi.md#namespaces_delete_compaction_threshold) | **DELETE** /namespaces/{tenant}/{namespace}/compactionThreshold | Delete maximum number of uncompacted bytes in a topic before compaction is triggered.
[**namespaces_delete_dispatch_rate**](NamespacesApi.md#namespaces_delete_dispatch_rate) | **DELETE** /namespaces/{tenant}/{namespace}/dispatchRate | Delete dispatch-rate throttling for all topics of the namespace
[**namespaces_delete_namespace**](NamespacesApi.md#namespaces_delete_namespace) | **DELETE** /namespaces/{tenant}/{namespace} | Delete a namespace and all the topics under it.
[**namespaces_delete_namespace_bundle**](NamespacesApi.md#namespaces_delete_namespace_bundle) | **DELETE** /namespaces/{tenant}/{namespace}/{bundle} | Delete a namespace bundle and all the topics under it.
[**namespaces_delete_persistence**](NamespacesApi.md#namespaces_delete_persistence) | **DELETE** /namespaces/{tenant}/{namespace}/persistence | Delete the persistence configuration for all topics on a namespace
[**namespaces_delete_subscribe_rate**](NamespacesApi.md#namespaces_delete_subscribe_rate) | **DELETE** /namespaces/{tenant}/{namespace}/subscribeRate | Delete subscribe-rate throttling for all topics of the namespace
[**namespaces_delete_subscription_dispatch_rate**](NamespacesApi.md#namespaces_delete_subscription_dispatch_rate) | **DELETE** /namespaces/{tenant}/{namespace}/subscriptionDispatchRate | Delete Subscription dispatch-rate throttling for all topics of the namespace
[**namespaces_get_anti_affinity_namespaces**](NamespacesApi.md#namespaces_get_anti_affinity_namespaces) | **GET** /namespaces/{cluster}/antiAffinity/{group} | Get all namespaces that are grouped by given anti-affinity group in a given cluster. api can be only accessed by admin of any of the existing tenant
[**namespaces_get_auto_subscription_creation**](NamespacesApi.md#namespaces_get_auto_subscription_creation) | **GET** /namespaces/{tenant}/{namespace}/autoSubscriptionCreation | Get autoSubscriptionCreation info in a namespace
[**namespaces_get_auto_topic_creation**](NamespacesApi.md#namespaces_get_auto_topic_creation) | **GET** /namespaces/{tenant}/{namespace}/autoTopicCreation | Get autoTopicCreation info in a namespace
[**namespaces_get_backlog_quota_map**](NamespacesApi.md#namespaces_get_backlog_quota_map) | **GET** /namespaces/{tenant}/{namespace}/backlogQuotaMap | Get backlog quota map on a namespace.
[**namespaces_get_bookie_affinity_group**](NamespacesApi.md#namespaces_get_bookie_affinity_group) | **GET** /namespaces/{property}/{namespace}/persistence/bookieAffinity | Get the bookie-affinity-group from namespace-local policy.
[**namespaces_get_bundles_data**](NamespacesApi.md#namespaces_get_bundles_data) | **GET** /namespaces/{tenant}/{namespace}/bundles | Get the bundles split data.
[**namespaces_get_compaction_threshold**](NamespacesApi.md#namespaces_get_compaction_threshold) | **GET** /namespaces/{tenant}/{namespace}/compactionThreshold | Maximum number of uncompacted bytes in topics before compaction is triggered.
[**namespaces_get_deduplication**](NamespacesApi.md#namespaces_get_deduplication) | **GET** /namespaces/{tenant}/{namespace}/deduplication | Get broker side deduplication for all topics in a namespace
[**namespaces_get_deduplication_snapshot_interval**](NamespacesApi.md#namespaces_get_deduplication_snapshot_interval) | **GET** /namespaces/{tenant}/{namespace}/deduplicationSnapshotInterval | Get deduplicationSnapshotInterval config on a namespace.
[**namespaces_get_delayed_delivery_policies**](NamespacesApi.md#namespaces_get_delayed_delivery_policies) | **GET** /namespaces/{tenant}/{namespace}/delayedDelivery | Get delayed delivery messages config on a namespace.
[**namespaces_get_dispatch_rate**](NamespacesApi.md#namespaces_get_dispatch_rate) | **GET** /namespaces/{tenant}/{namespace}/dispatchRate | Get dispatch-rate configured for the namespace, null means dispatch-rate not configured, -1 means msg-dispatch-rate or byte-dispatch-rate not configured in dispatch-rate yet
[**namespaces_get_dispatcher_pause_on_ack_state_persistent**](NamespacesApi.md#namespaces_get_dispatcher_pause_on_ack_state_persistent) | **GET** /namespaces/{tenant}/{namespace}/dispatcherPauseOnAckStatePersistent | Get dispatcher pause on ack state persistent config on a namespace.
[**namespaces_get_encryption_required**](NamespacesApi.md#namespaces_get_encryption_required) | **GET** /namespaces/{tenant}/{namespace}/encryptionRequired | Get message encryption required status in a namespace
[**namespaces_get_entry_filters_per_topic**](NamespacesApi.md#namespaces_get_entry_filters_per_topic) | **GET** /namespaces/{tenant}/{namespace}/entryFilters | Get maxConsumersPerSubscription config on a namespace.
[**namespaces_get_inactive_topic_policies**](NamespacesApi.md#namespaces_get_inactive_topic_policies) | **GET** /namespaces/{tenant}/{namespace}/inactiveTopicPolicies | Get inactive topic policies config on a namespace.
[**namespaces_get_is_allow_auto_update_schema**](NamespacesApi.md#namespaces_get_is_allow_auto_update_schema) | **GET** /namespaces/{tenant}/{namespace}/isAllowAutoUpdateSchema | The flag of whether allow auto update schema
[**namespaces_get_max_consumers_per_subscription**](NamespacesApi.md#namespaces_get_max_consumers_per_subscription) | **GET** /namespaces/{tenant}/{namespace}/maxConsumersPerSubscription | Get maxConsumersPerSubscription config on a namespace.
[**namespaces_get_max_consumers_per_topic**](NamespacesApi.md#namespaces_get_max_consumers_per_topic) | **GET** /namespaces/{tenant}/{namespace}/maxConsumersPerTopic | Get maxConsumersPerTopic config on a namespace.
[**namespaces_get_max_producers_per_topic**](NamespacesApi.md#namespaces_get_max_producers_per_topic) | **GET** /namespaces/{tenant}/{namespace}/maxProducersPerTopic | Get maxProducersPerTopic config on a namespace.
[**namespaces_get_max_subscriptions_per_topic**](NamespacesApi.md#namespaces_get_max_subscriptions_per_topic) | **GET** /namespaces/{tenant}/{namespace}/maxSubscriptionsPerTopic | Get maxSubscriptionsPerTopic config on a namespace.
[**namespaces_get_max_topics_per_namespace**](NamespacesApi.md#namespaces_get_max_topics_per_namespace) | **GET** /namespaces/{tenant}/{namespace}/maxTopicsPerNamespace | Get maxTopicsPerNamespace config on a namespace.
[**namespaces_get_max_unacked_messages_per_consumer**](NamespacesApi.md#namespaces_get_max_unacked_messages_per_consumer) | **GET** /namespaces/{tenant}/{namespace}/maxUnackedMessagesPerConsumer | Get maxUnackedMessagesPerConsumer config on a namespace.
[**namespaces_get_max_unackedmessages_per_subscription**](NamespacesApi.md#namespaces_get_max_unackedmessages_per_subscription) | **GET** /namespaces/{tenant}/{namespace}/maxUnackedMessagesPerSubscription | Get maxUnackedMessagesPerSubscription config on a namespace.
[**namespaces_get_namespace_anti_affinity_group**](NamespacesApi.md#namespaces_get_namespace_anti_affinity_group) | **GET** /namespaces/{tenant}/{namespace}/antiAffinity | Get anti-affinity group of a namespace.
[**namespaces_get_namespace_message_ttl**](NamespacesApi.md#namespaces_get_namespace_message_ttl) | **GET** /namespaces/{tenant}/{namespace}/messageTTL | Get the message TTL for the namespace
[**namespaces_get_namespace_replication_clusters**](NamespacesApi.md#namespaces_get_namespace_replication_clusters) | **GET** /namespaces/{tenant}/{namespace}/replication | Get the replication clusters for a namespace.
[**namespaces_get_namespace_resource_group**](NamespacesApi.md#namespaces_get_namespace_resource_group) | **GET** /namespaces/{tenant}/{namespace}/resourcegroup | Get the resource group attached to the namespace
[**namespaces_get_offload_deletion_lag**](NamespacesApi.md#namespaces_get_offload_deletion_lag) | **GET** /namespaces/{tenant}/{namespace}/offloadDeletionLagMs | Number of milliseconds to wait before deleting a ledger segment which has been offloaded from the Pulsar cluster's local storage (i.e. BookKeeper)
[**namespaces_get_offload_policies**](NamespacesApi.md#namespaces_get_offload_policies) | **GET** /namespaces/{tenant}/{namespace}/offloadPolicies | Get offload configuration on a namespace.
[**namespaces_get_offload_threshold**](NamespacesApi.md#namespaces_get_offload_threshold) | **GET** /namespaces/{tenant}/{namespace}/offloadThreshold | Maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage
[**namespaces_get_offload_threshold_in_seconds**](NamespacesApi.md#namespaces_get_offload_threshold_in_seconds) | **GET** /namespaces/{tenant}/{namespace}/offloadThresholdInSeconds | Maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage
[**namespaces_get_permission_on_subscription**](NamespacesApi.md#namespaces_get_permission_on_subscription) | **GET** /namespaces/{tenant}/{namespace}/permissions/subscription | Retrieve the permissions for a subscription.
[**namespaces_get_permissions**](NamespacesApi.md#namespaces_get_permissions) | **GET** /namespaces/{tenant}/{namespace}/permissions | Retrieve the permissions for a namespace.
[**namespaces_get_persistence**](NamespacesApi.md#namespaces_get_persistence) | **GET** /namespaces/{tenant}/{namespace}/persistence | Get the persistence configuration for a namespace.
[**namespaces_get_policies**](NamespacesApi.md#namespaces_get_policies) | **GET** /namespaces/{tenant}/{namespace} | Get the dump all the policies specified for a namespace.
[**namespaces_get_properties**](NamespacesApi.md#namespaces_get_properties) | **GET** /namespaces/{tenant}/{namespace}/properties | Get key value pair properties for a given namespace.
[**namespaces_get_property**](NamespacesApi.md#namespaces_get_property) | **GET** /namespaces/{tenant}/{namespace}/property/{key} | Get property value for a given key on a namespace.
[**namespaces_get_replicator_dispatch_rate**](NamespacesApi.md#namespaces_get_replicator_dispatch_rate) | **GET** /namespaces/{tenant}/{namespace}/replicatorDispatchRate | Get replicator dispatch-rate configured for the namespace, null means replicator dispatch-rate not configured, -1 means msg-dispatch-rate or byte-dispatch-rate not configured in dispatch-rate yet
[**namespaces_get_retention**](NamespacesApi.md#namespaces_get_retention) | **GET** /namespaces/{tenant}/{namespace}/retention | Get retention config on a namespace.
[**namespaces_get_schema_auto_update_compatibility_strategy**](NamespacesApi.md#namespaces_get_schema_auto_update_compatibility_strategy) | **GET** /namespaces/{tenant}/{namespace}/schemaAutoUpdateCompatibilityStrategy | The strategy used to check the compatibility of new schemas, provided by producers, before automatically updating the schema
[**namespaces_get_schema_compatibility_strategy**](NamespacesApi.md#namespaces_get_schema_compatibility_strategy) | **GET** /namespaces/{tenant}/{namespace}/schemaCompatibilityStrategy | The strategy of the namespace schema compatibility 
[**namespaces_get_schema_validtion_enforced**](NamespacesApi.md#namespaces_get_schema_validtion_enforced) | **GET** /namespaces/{tenant}/{namespace}/schemaValidationEnforced | Get schema validation enforced flag for namespace.
[**namespaces_get_subscribe_rate**](NamespacesApi.md#namespaces_get_subscribe_rate) | **GET** /namespaces/{tenant}/{namespace}/subscribeRate | Get subscribe-rate configured for the namespace
[**namespaces_get_subscription_auth_mode**](NamespacesApi.md#namespaces_get_subscription_auth_mode) | **GET** /namespaces/{tenant}/{namespace}/subscriptionAuthMode | Get subscription auth mode in a namespace
[**namespaces_get_subscription_dispatch_rate**](NamespacesApi.md#namespaces_get_subscription_dispatch_rate) | **GET** /namespaces/{tenant}/{namespace}/subscriptionDispatchRate | Get subscription dispatch-rate configured for the namespace, null means subscription dispatch-rate not configured, -1 means msg-dispatch-rate or byte-dispatch-rate not configured in dispatch-rate yet
[**namespaces_get_subscription_expiration_time**](NamespacesApi.md#namespaces_get_subscription_expiration_time) | **GET** /namespaces/{tenant}/{namespace}/subscriptionExpirationTime | Get the subscription expiration time for the namespace
[**namespaces_get_subscription_types_enabled**](NamespacesApi.md#namespaces_get_subscription_types_enabled) | **GET** /namespaces/{tenant}/{namespace}/subscriptionTypesEnabled | The set of whether allow subscription types
[**namespaces_get_tenant_namespaces**](NamespacesApi.md#namespaces_get_tenant_namespaces) | **GET** /namespaces/{tenant} | Get the list of all the namespaces for a certain tenant.
[**namespaces_get_topic_hash_positions**](NamespacesApi.md#namespaces_get_topic_hash_positions) | **GET** /namespaces/{tenant}/{namespace}/{bundle}/topicHashPositions | Get hash positions for topics
[**namespaces_get_topics**](NamespacesApi.md#namespaces_get_topics) | **GET** /namespaces/{tenant}/{namespace}/topics | Get the list of all the topics under a certain namespace.
[**namespaces_grant_permission_on_namespace**](NamespacesApi.md#namespaces_grant_permission_on_namespace) | **POST** /namespaces/{tenant}/{namespace}/permissions/{role} | Grant a new permission to a role on a namespace.
[**namespaces_modify_deduplication**](NamespacesApi.md#namespaces_modify_deduplication) | **POST** /namespaces/{tenant}/{namespace}/deduplication | Enable or disable broker side deduplication for all topics in a namespace
[**namespaces_modify_encryption_required**](NamespacesApi.md#namespaces_modify_encryption_required) | **POST** /namespaces/{tenant}/{namespace}/encryptionRequired | Message encryption is required or not for all topics in a namespace
[**namespaces_remove_auto_subscription_creation**](NamespacesApi.md#namespaces_remove_auto_subscription_creation) | **DELETE** /namespaces/{tenant}/{namespace}/autoSubscriptionCreation | Remove override of broker's allowAutoSubscriptionCreation in a namespace
[**namespaces_remove_auto_topic_creation**](NamespacesApi.md#namespaces_remove_auto_topic_creation) | **DELETE** /namespaces/{tenant}/{namespace}/autoTopicCreation | Remove override of broker's allowAutoTopicCreation in a namespace
[**namespaces_remove_backlog_quota**](NamespacesApi.md#namespaces_remove_backlog_quota) | **DELETE** /namespaces/{tenant}/{namespace}/backlogQuota | Remove a backlog quota policy from a namespace.
[**namespaces_remove_deduplication**](NamespacesApi.md#namespaces_remove_deduplication) | **DELETE** /namespaces/{tenant}/{namespace}/deduplication | Remove broker side deduplication for all topics in a namespace
[**namespaces_remove_delayed_delivery_policies**](NamespacesApi.md#namespaces_remove_delayed_delivery_policies) | **DELETE** /namespaces/{tenant}/{namespace}/delayedDelivery | Delete delayed delivery messages config on a namespace.
[**namespaces_remove_dispatcher_pause_on_ack_state_persistent**](NamespacesApi.md#namespaces_remove_dispatcher_pause_on_ack_state_persistent) | **DELETE** /namespaces/{tenant}/{namespace}/dispatcherPauseOnAckStatePersistent | Remove dispatcher pause on ack state persistent configuration for specified namespace.
[**namespaces_remove_inactive_topic_policies**](NamespacesApi.md#namespaces_remove_inactive_topic_policies) | **DELETE** /namespaces/{tenant}/{namespace}/inactiveTopicPolicies | Remove inactive topic policies from a namespace.
[**namespaces_remove_max_consumers_per_subscription**](NamespacesApi.md#namespaces_remove_max_consumers_per_subscription) | **DELETE** /namespaces/{tenant}/{namespace}/maxConsumersPerSubscription |  Set maxConsumersPerSubscription configuration on a namespace.
[**namespaces_remove_max_consumers_per_topic**](NamespacesApi.md#namespaces_remove_max_consumers_per_topic) | **DELETE** /namespaces/{tenant}/{namespace}/maxConsumersPerTopic | Remove maxConsumersPerTopic configuration on a namespace.
[**namespaces_remove_max_producers_per_topic**](NamespacesApi.md#namespaces_remove_max_producers_per_topic) | **DELETE** /namespaces/{tenant}/{namespace}/maxProducersPerTopic | Remove maxProducersPerTopic configuration on a namespace.
[**namespaces_remove_max_subscriptions_per_topic**](NamespacesApi.md#namespaces_remove_max_subscriptions_per_topic) | **DELETE** /namespaces/{tenant}/{namespace}/maxSubscriptionsPerTopic | Remove maxSubscriptionsPerTopic configuration on a namespace.
[**namespaces_remove_max_topics_per_namespace**](NamespacesApi.md#namespaces_remove_max_topics_per_namespace) | **DELETE** /namespaces/{tenant}/{namespace}/maxTopicsPerNamespace | Remove maxTopicsPerNamespace config on a namespace.
[**namespaces_remove_max_unackedmessages_per_consumer**](NamespacesApi.md#namespaces_remove_max_unackedmessages_per_consumer) | **DELETE** /namespaces/{tenant}/{namespace}/maxUnackedMessagesPerConsumer | Remove maxUnackedMessagesPerConsumer config on a namespace.
[**namespaces_remove_max_unackedmessages_per_subscription**](NamespacesApi.md#namespaces_remove_max_unackedmessages_per_subscription) | **DELETE** /namespaces/{tenant}/{namespace}/maxUnackedMessagesPerSubscription | Remove maxUnackedMessagesPerSubscription config on a namespace.
[**namespaces_remove_namespace_anti_affinity_group**](NamespacesApi.md#namespaces_remove_namespace_anti_affinity_group) | **DELETE** /namespaces/{tenant}/{namespace}/antiAffinity | Remove anti-affinity group of a namespace.
[**namespaces_remove_namespace_entry_filters**](NamespacesApi.md#namespaces_remove_namespace_entry_filters) | **DELETE** /namespaces/{tenant}/{namespace}/entryFilters | Remove entry filters for namespace
[**namespaces_remove_namespace_message_ttl**](NamespacesApi.md#namespaces_remove_namespace_message_ttl) | **DELETE** /namespaces/{tenant}/{namespace}/messageTTL | Remove message TTL in seconds for namespace
[**namespaces_remove_namespace_resource_group**](NamespacesApi.md#namespaces_remove_namespace_resource_group) | **DELETE** /namespaces/{tenant}/{namespace}/resourcegroup | Delete resourcegroup for a namespace
[**namespaces_remove_offload_policies**](NamespacesApi.md#namespaces_remove_offload_policies) | **DELETE** /namespaces/{tenant}/{namespace}/removeOffloadPolicies |  Set offload configuration on a namespace.
[**namespaces_remove_property**](NamespacesApi.md#namespaces_remove_property) | **DELETE** /namespaces/{tenant}/{namespace}/property/{key} | Remove property value for a given key on a namespace.
[**namespaces_remove_replicator_dispatch_rate**](NamespacesApi.md#namespaces_remove_replicator_dispatch_rate) | **DELETE** /namespaces/{tenant}/{namespace}/replicatorDispatchRate | Remove replicator dispatch-rate throttling for all topics of the namespace
[**namespaces_remove_retention**](NamespacesApi.md#namespaces_remove_retention) | **DELETE** /namespaces/{tenant}/{namespace}/retention |  Remove retention configuration on a namespace.
[**namespaces_remove_subscription_expiration_time**](NamespacesApi.md#namespaces_remove_subscription_expiration_time) | **DELETE** /namespaces/{tenant}/{namespace}/subscriptionExpirationTime | Remove subscription expiration time for namespace
[**namespaces_remove_subscription_types_enabled**](NamespacesApi.md#namespaces_remove_subscription_types_enabled) | **DELETE** /namespaces/{tenant}/{namespace}/subscriptionTypesEnabled |  Remove subscription types enabled on a namespace.
[**namespaces_revoke_permissions_on_namespace**](NamespacesApi.md#namespaces_revoke_permissions_on_namespace) | **DELETE** /namespaces/{tenant}/{namespace}/permissions/{role} | Revoke all permissions to a role on a namespace.
[**namespaces_scan_offloaded_ledgers**](NamespacesApi.md#namespaces_scan_offloaded_ledgers) | **GET** /namespaces/{tenant}/{namespace}/scanOffloadedLedgers | Trigger the scan of offloaded Ledgers on the LedgerOffloader for the given namespace
[**namespaces_set_auto_subscription_creation**](NamespacesApi.md#namespaces_set_auto_subscription_creation) | **POST** /namespaces/{tenant}/{namespace}/autoSubscriptionCreation | Override broker's allowAutoSubscriptionCreation setting for a namespace
[**namespaces_set_auto_topic_creation**](NamespacesApi.md#namespaces_set_auto_topic_creation) | **POST** /namespaces/{tenant}/{namespace}/autoTopicCreation | Override broker's allowAutoTopicCreation setting for a namespace
[**namespaces_set_backlog_quota**](NamespacesApi.md#namespaces_set_backlog_quota) | **POST** /namespaces/{tenant}/{namespace}/backlogQuota |  Set a backlog quota for all the topics on a namespace.
[**namespaces_set_bookie_affinity_group**](NamespacesApi.md#namespaces_set_bookie_affinity_group) | **POST** /namespaces/{tenant}/{namespace}/persistence/bookieAffinity | Set the bookie-affinity-group to namespace-persistent policy.
[**namespaces_set_compaction_threshold**](NamespacesApi.md#namespaces_set_compaction_threshold) | **PUT** /namespaces/{tenant}/{namespace}/compactionThreshold | Set maximum number of uncompacted bytes in a topic before compaction is triggered.
[**namespaces_set_deduplication_snapshot_interval**](NamespacesApi.md#namespaces_set_deduplication_snapshot_interval) | **POST** /namespaces/{tenant}/{namespace}/deduplicationSnapshotInterval | Set deduplicationSnapshotInterval config on a namespace.
[**namespaces_set_delayed_delivery_policies**](NamespacesApi.md#namespaces_set_delayed_delivery_policies) | **POST** /namespaces/{tenant}/{namespace}/delayedDelivery | Set delayed delivery messages config on a namespace.
[**namespaces_set_dispatch_rate**](NamespacesApi.md#namespaces_set_dispatch_rate) | **POST** /namespaces/{tenant}/{namespace}/dispatchRate | Set dispatch-rate throttling for all topics of the namespace
[**namespaces_set_dispatcher_pause_on_ack_state_persistent**](NamespacesApi.md#namespaces_set_dispatcher_pause_on_ack_state_persistent) | **POST** /namespaces/{tenant}/{namespace}/dispatcherPauseOnAckStatePersistent | Set dispatcher pause on ack state persistent configuration for specified namespace.
[**namespaces_set_entry_filters_per_topic**](NamespacesApi.md#namespaces_set_entry_filters_per_topic) | **POST** /namespaces/{tenant}/{namespace}/entryFilters | Set entry filters for namespace
[**namespaces_set_inactive_topic_policies**](NamespacesApi.md#namespaces_set_inactive_topic_policies) | **POST** /namespaces/{tenant}/{namespace}/inactiveTopicPolicies | Set inactive topic policies config on a namespace.
[**namespaces_set_is_allow_auto_update_schema**](NamespacesApi.md#namespaces_set_is_allow_auto_update_schema) | **POST** /namespaces/{tenant}/{namespace}/isAllowAutoUpdateSchema | Update flag of whether allow auto update schema
[**namespaces_set_max_consumers_per_subscription**](NamespacesApi.md#namespaces_set_max_consumers_per_subscription) | **POST** /namespaces/{tenant}/{namespace}/maxConsumersPerSubscription |  Set maxConsumersPerSubscription configuration on a namespace.
[**namespaces_set_max_consumers_per_topic**](NamespacesApi.md#namespaces_set_max_consumers_per_topic) | **POST** /namespaces/{tenant}/{namespace}/maxConsumersPerTopic |  Set maxConsumersPerTopic configuration on a namespace.
[**namespaces_set_max_producers_per_topic**](NamespacesApi.md#namespaces_set_max_producers_per_topic) | **POST** /namespaces/{tenant}/{namespace}/maxProducersPerTopic |  Set maxProducersPerTopic configuration on a namespace.
[**namespaces_set_max_subscriptions_per_topic**](NamespacesApi.md#namespaces_set_max_subscriptions_per_topic) | **POST** /namespaces/{tenant}/{namespace}/maxSubscriptionsPerTopic |  Set maxSubscriptionsPerTopic configuration on a namespace.
[**namespaces_set_max_topics_per_namespace**](NamespacesApi.md#namespaces_set_max_topics_per_namespace) | **POST** /namespaces/{tenant}/{namespace}/maxTopicsPerNamespace | Set maxTopicsPerNamespace config on a namespace.
[**namespaces_set_max_unacked_messages_per_consumer**](NamespacesApi.md#namespaces_set_max_unacked_messages_per_consumer) | **POST** /namespaces/{tenant}/{namespace}/maxUnackedMessagesPerConsumer |  Set maxConsumersPerTopic configuration on a namespace.
[**namespaces_set_max_unacked_messages_per_subscription**](NamespacesApi.md#namespaces_set_max_unacked_messages_per_subscription) | **POST** /namespaces/{tenant}/{namespace}/maxUnackedMessagesPerSubscription |  Set maxUnackedMessagesPerSubscription configuration on a namespace.
[**namespaces_set_namespace_anti_affinity_group**](NamespacesApi.md#namespaces_set_namespace_anti_affinity_group) | **POST** /namespaces/{tenant}/{namespace}/antiAffinity | Set anti-affinity group for a namespace
[**namespaces_set_namespace_message_ttl**](NamespacesApi.md#namespaces_set_namespace_message_ttl) | **POST** /namespaces/{tenant}/{namespace}/messageTTL | Set message TTL in seconds for namespace
[**namespaces_set_namespace_replication_clusters**](NamespacesApi.md#namespaces_set_namespace_replication_clusters) | **POST** /namespaces/{tenant}/{namespace}/replication | Set the replication clusters for a namespace.
[**namespaces_set_namespace_resource_group**](NamespacesApi.md#namespaces_set_namespace_resource_group) | **POST** /namespaces/{tenant}/{namespace}/resourcegroup/{resourcegroup} | Set resourcegroup for a namespace
[**namespaces_set_offload_deletion_lag**](NamespacesApi.md#namespaces_set_offload_deletion_lag) | **PUT** /namespaces/{tenant}/{namespace}/offloadDeletionLagMs | Set number of milliseconds to wait before deleting a ledger segment which has been offloaded from the Pulsar cluster's local storage (i.e. BookKeeper)
[**namespaces_set_offload_policies**](NamespacesApi.md#namespaces_set_offload_policies) | **POST** /namespaces/{tenant}/{namespace}/offloadPolicies |  Set offload configuration on a namespace.
[**namespaces_set_offload_threshold**](NamespacesApi.md#namespaces_set_offload_threshold) | **PUT** /namespaces/{tenant}/{namespace}/offloadThreshold | Set maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage
[**namespaces_set_offload_threshold_in_seconds**](NamespacesApi.md#namespaces_set_offload_threshold_in_seconds) | **PUT** /namespaces/{tenant}/{namespace}/offloadThresholdInSeconds | Set maximum number of seconds stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage
[**namespaces_set_persistence**](NamespacesApi.md#namespaces_set_persistence) | **POST** /namespaces/{tenant}/{namespace}/persistence | Set the persistence configuration for all the topics on a namespace.
[**namespaces_set_properties**](NamespacesApi.md#namespaces_set_properties) | **PUT** /namespaces/{tenant}/{namespace}/properties | Put key value pairs property on a namespace.
[**namespaces_set_property**](NamespacesApi.md#namespaces_set_property) | **PUT** /namespaces/{tenant}/{namespace}/property/{key}/{value} | Put a key value pair property on a namespace.
[**namespaces_set_replicator_dispatch_rate**](NamespacesApi.md#namespaces_set_replicator_dispatch_rate) | **POST** /namespaces/{tenant}/{namespace}/replicatorDispatchRate | Set replicator dispatch-rate throttling for all topics of the namespace
[**namespaces_set_retention**](NamespacesApi.md#namespaces_set_retention) | **POST** /namespaces/{tenant}/{namespace}/retention |  Set retention configuration on a namespace.
[**namespaces_set_schema_auto_update_compatibility_strategy**](NamespacesApi.md#namespaces_set_schema_auto_update_compatibility_strategy) | **PUT** /namespaces/{tenant}/{namespace}/schemaAutoUpdateCompatibilityStrategy | Update the strategy used to check the compatibility of new schemas, provided by producers, before automatically updating the schema
[**namespaces_set_schema_compatibility_strategy**](NamespacesApi.md#namespaces_set_schema_compatibility_strategy) | **PUT** /namespaces/{tenant}/{namespace}/schemaCompatibilityStrategy | Update the strategy used to check the compatibility of new schema
[**namespaces_set_schema_validation_enforced**](NamespacesApi.md#namespaces_set_schema_validation_enforced) | **POST** /namespaces/{tenant}/{namespace}/schemaValidationEnforced | Set schema validation enforced flag on namespace.
[**namespaces_set_subscribe_rate**](NamespacesApi.md#namespaces_set_subscribe_rate) | **POST** /namespaces/{tenant}/{namespace}/subscribeRate | Set subscribe-rate throttling for all topics of the namespace
[**namespaces_set_subscription_auth_mode**](NamespacesApi.md#namespaces_set_subscription_auth_mode) | **POST** /namespaces/{tenant}/{namespace}/subscriptionAuthMode |  Set a subscription auth mode for all the topics on a namespace.
[**namespaces_set_subscription_dispatch_rate**](NamespacesApi.md#namespaces_set_subscription_dispatch_rate) | **POST** /namespaces/{tenant}/{namespace}/subscriptionDispatchRate | Set Subscription dispatch-rate throttling for all topics of the namespace
[**namespaces_set_subscription_expiration_time**](NamespacesApi.md#namespaces_set_subscription_expiration_time) | **POST** /namespaces/{tenant}/{namespace}/subscriptionExpirationTime | Set subscription expiration time in minutes for namespace
[**namespaces_set_subscription_types_enabled**](NamespacesApi.md#namespaces_set_subscription_types_enabled) | **POST** /namespaces/{tenant}/{namespace}/subscriptionTypesEnabled | Update set of whether allow share sub type
[**namespaces_split_namespace_bundle**](NamespacesApi.md#namespaces_split_namespace_bundle) | **PUT** /namespaces/{tenant}/{namespace}/{bundle}/split | Split a namespace bundle
[**namespaces_unload_namespace**](NamespacesApi.md#namespaces_unload_namespace) | **PUT** /namespaces/{tenant}/{namespace}/unload | Unload namespace
[**namespaces_unload_namespace_bundle**](NamespacesApi.md#namespaces_unload_namespace_bundle) | **PUT** /namespaces/{tenant}/{namespace}/{bundle}/unload | Unload a namespace bundle
[**namespaces_unsubscribe_namespace**](NamespacesApi.md#namespaces_unsubscribe_namespace) | **POST** /namespaces/{tenant}/{namespace}/unsubscribe/{subscription} | Unsubscribes the given subscription on all topics on a namespace.
[**namespaces_unsubscribe_namespace_bundle**](NamespacesApi.md#namespaces_unsubscribe_namespace_bundle) | **POST** /namespaces/{tenant}/{namespace}/{bundle}/unsubscribe/{subscription} | Unsubscribes the given subscription on all topics on a namespace bundle.



## namespaces_clear_namespace_backlog

> namespaces_clear_namespace_backlog(tenant, namespace, authoritative)
Clear backlog for all topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_clear_namespace_backlog_for_subscription

> namespaces_clear_namespace_backlog_for_subscription(tenant, namespace, subscription, authoritative)
Clear backlog for a given subscription on all topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**subscription** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_clear_namespace_bundle_backlog

> namespaces_clear_namespace_bundle_backlog(tenant, namespace, bundle, authoritative)
Clear backlog for all topics on a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**bundle** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_clear_namespace_bundle_backlog_for_subscription

> namespaces_clear_namespace_bundle_backlog_for_subscription(tenant, namespace, subscription, bundle, authoritative)
Clear backlog for a given subscription on all topics on a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**subscription** | **String** |  | [required] |
**bundle** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_clear_offload_deletion_lag

> namespaces_clear_offload_deletion_lag(tenant, namespace)
Clear the namespace configured offload deletion lag. The topics in the namespace will fallback to using the default configured deletion lag for the broker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_clear_properties

> namespaces_clear_properties(tenant, namespace)
Clear properties on a given namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_create_namespace

> namespaces_create_namespace(tenant, namespace, body)
Creates a new namespace with the specified policies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**Policies**](Policies.md)> | Policies for the namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_delete_bookie_affinity_group

> namespaces_delete_bookie_affinity_group(property, namespace)
Delete the bookie-affinity-group from namespace-local policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_delete_compaction_threshold

> namespaces_delete_compaction_threshold(tenant, namespace)
Delete maximum number of uncompacted bytes in a topic before compaction is triggered.

The backlog size is compared to the threshold periodically. A threshold of 0 disabled automatic compaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_delete_dispatch_rate

> namespaces_delete_dispatch_rate(tenant, namespace)
Delete dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_delete_namespace

> namespaces_delete_namespace(tenant, namespace, force, authoritative)
Delete a namespace and all the topics under it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**force** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_delete_namespace_bundle

> namespaces_delete_namespace_bundle(tenant, namespace, bundle, force, authoritative)
Delete a namespace bundle and all the topics under it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**bundle** | **String** |  | [required] |
**force** | Option<**bool**> |  |  |[default to false]
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_delete_persistence

> namespaces_delete_persistence(tenant, namespace)
Delete the persistence configuration for all topics on a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_delete_subscribe_rate

> namespaces_delete_subscribe_rate(tenant, namespace)
Delete subscribe-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_delete_subscription_dispatch_rate

> namespaces_delete_subscription_dispatch_rate(tenant, namespace)
Delete Subscription dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_anti_affinity_namespaces

> Vec<String> namespaces_get_anti_affinity_namespaces(cluster, group, tenant)
Get all namespaces that are grouped by given anti-affinity group in a given cluster. api can be only accessed by admin of any of the existing tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** |  | [required] |
**group** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_auto_subscription_creation

> namespaces_get_auto_subscription_creation(tenant, namespace)
Get autoSubscriptionCreation info in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_auto_topic_creation

> namespaces_get_auto_topic_creation(tenant, namespace)
Get autoTopicCreation info in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_backlog_quota_map

> namespaces_get_backlog_quota_map(tenant, namespace)
Get backlog quota map on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_bookie_affinity_group

> models::BookieAffinityGroupData namespaces_get_bookie_affinity_group(property, namespace)
Get the bookie-affinity-group from namespace-local policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**models::BookieAffinityGroupData**](BookieAffinityGroupData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_bundles_data

> namespaces_get_bundles_data(tenant, namespace)
Get the bundles split data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_compaction_threshold

> namespaces_get_compaction_threshold(tenant, namespace)
Maximum number of uncompacted bytes in topics before compaction is triggered.

The backlog size is compared to the threshold periodically. A threshold of 0 disabled automatic compaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_deduplication

> namespaces_get_deduplication(tenant, namespace)
Get broker side deduplication for all topics in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_deduplication_snapshot_interval

> namespaces_get_deduplication_snapshot_interval(tenant, namespace)
Get deduplicationSnapshotInterval config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_delayed_delivery_policies

> namespaces_get_delayed_delivery_policies(tenant, namespace)
Get delayed delivery messages config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_dispatch_rate

> namespaces_get_dispatch_rate(tenant, namespace)
Get dispatch-rate configured for the namespace, null means dispatch-rate not configured, -1 means msg-dispatch-rate or byte-dispatch-rate not configured in dispatch-rate yet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_dispatcher_pause_on_ack_state_persistent

> namespaces_get_dispatcher_pause_on_ack_state_persistent(tenant, namespace)
Get dispatcher pause on ack state persistent config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_encryption_required

> namespaces_get_encryption_required(tenant, namespace)
Get message encryption required status in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_entry_filters_per_topic

> namespaces_get_entry_filters_per_topic(tenant, namespace)
Get maxConsumersPerSubscription config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_inactive_topic_policies

> namespaces_get_inactive_topic_policies(tenant, namespace)
Get inactive topic policies config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_is_allow_auto_update_schema

> namespaces_get_is_allow_auto_update_schema(tenant, namespace)
The flag of whether allow auto update schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_max_consumers_per_subscription

> namespaces_get_max_consumers_per_subscription(tenant, namespace)
Get maxConsumersPerSubscription config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_max_consumers_per_topic

> namespaces_get_max_consumers_per_topic(tenant, namespace)
Get maxConsumersPerTopic config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_max_producers_per_topic

> namespaces_get_max_producers_per_topic(tenant, namespace)
Get maxProducersPerTopic config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_max_subscriptions_per_topic

> namespaces_get_max_subscriptions_per_topic(tenant, namespace)
Get maxSubscriptionsPerTopic config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_max_topics_per_namespace

> namespaces_get_max_topics_per_namespace(tenant, namespace)
Get maxTopicsPerNamespace config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_max_unacked_messages_per_consumer

> namespaces_get_max_unacked_messages_per_consumer(tenant, namespace)
Get maxUnackedMessagesPerConsumer config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_max_unackedmessages_per_subscription

> namespaces_get_max_unackedmessages_per_subscription(tenant, namespace)
Get maxUnackedMessagesPerSubscription config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_namespace_anti_affinity_group

> String namespaces_get_namespace_anti_affinity_group(tenant, namespace)
Get anti-affinity group of a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_namespace_message_ttl

> i32 namespaces_get_namespace_message_ttl(tenant, namespace)
Get the message TTL for the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_namespace_replication_clusters

> Vec<String> namespaces_get_namespace_replication_clusters(tenant, namespace)
Get the replication clusters for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_namespace_resource_group

> namespaces_get_namespace_resource_group(tenant, namespace)
Get the resource group attached to the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_offload_deletion_lag

> namespaces_get_offload_deletion_lag(tenant, namespace)
Number of milliseconds to wait before deleting a ledger segment which has been offloaded from the Pulsar cluster's local storage (i.e. BookKeeper)

A negative value denotes that deletion has been completely disabled. 'null' denotes that the topics in the namespace will fall back to the broker default for deletion lag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_offload_policies

> namespaces_get_offload_policies(tenant, namespace)
Get offload configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_offload_threshold

> namespaces_get_offload_threshold(tenant, namespace)
Maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage

A negative value disables automatic offloading

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_offload_threshold_in_seconds

> namespaces_get_offload_threshold_in_seconds(tenant, namespace)
Maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage

A negative value disables automatic offloading

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_permission_on_subscription

> namespaces_get_permission_on_subscription(tenant, namespace)
Retrieve the permissions for a subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_permissions

> namespaces_get_permissions(tenant, namespace)
Retrieve the permissions for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_persistence

> namespaces_get_persistence(tenant, namespace)
Get the persistence configuration for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_policies

> models::Policies namespaces_get_policies(tenant, namespace)
Get the dump all the policies specified for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**models::Policies**](Policies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_properties

> namespaces_get_properties(tenant, namespace)
Get key value pair properties for a given namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_property

> namespaces_get_property(tenant, namespace, key)
Get property value for a given key on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_replicator_dispatch_rate

> namespaces_get_replicator_dispatch_rate(tenant, namespace)
Get replicator dispatch-rate configured for the namespace, null means replicator dispatch-rate not configured, -1 means msg-dispatch-rate or byte-dispatch-rate not configured in dispatch-rate yet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_retention

> namespaces_get_retention(tenant, namespace)
Get retention config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_schema_auto_update_compatibility_strategy

> String namespaces_get_schema_auto_update_compatibility_strategy(tenant, namespace)
The strategy used to check the compatibility of new schemas, provided by producers, before automatically updating the schema

The value AutoUpdateDisabled prevents producers from updating the schema.  If set to AutoUpdateDisabled, schemas must be updated through the REST api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_schema_compatibility_strategy

> namespaces_get_schema_compatibility_strategy(tenant, namespace)
The strategy of the namespace schema compatibility 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_schema_validtion_enforced

> namespaces_get_schema_validtion_enforced(tenant, namespace, applied)
Get schema validation enforced flag for namespace.

If the flag is set to true, when a producer without a schema attempts to produce to a topic with schema in this namespace, the producer will be failed to connect. PLEASE be carefully on using this, since non-java clients don't support schema.if you enable this setting, it will cause non-java clients failed to produce.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**applied** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_subscribe_rate

> namespaces_get_subscribe_rate(tenant, namespace)
Get subscribe-rate configured for the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_subscription_auth_mode

> namespaces_get_subscription_auth_mode(tenant, namespace)
Get subscription auth mode in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_subscription_dispatch_rate

> namespaces_get_subscription_dispatch_rate(tenant, namespace)
Get subscription dispatch-rate configured for the namespace, null means subscription dispatch-rate not configured, -1 means msg-dispatch-rate or byte-dispatch-rate not configured in dispatch-rate yet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_subscription_expiration_time

> namespaces_get_subscription_expiration_time(tenant, namespace)
Get the subscription expiration time for the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_subscription_types_enabled

> namespaces_get_subscription_types_enabled(tenant, namespace)
The set of whether allow subscription types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_tenant_namespaces

> Vec<String> namespaces_get_tenant_namespaces(tenant)
Get the list of all the namespaces for a certain tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_topic_hash_positions

> namespaces_get_topic_hash_positions(tenant, namespace, bundle, topics)
Get hash positions for topics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**bundle** | **String** |  | [required] |
**topics** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_get_topics

> Vec<String> namespaces_get_topics(tenant, namespace, mode, include_system_topic)
Get the list of all the topics under a certain namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**mode** | Option<**String**> |  |  |[default to PERSISTENT]
**include_system_topic** | Option<**bool**> | Include system topic |  |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_grant_permission_on_namespace

> namespaces_grant_permission_on_namespace(tenant, namespace, role, body)
Grant a new permission to a role on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**role** | **String** |  | [required] |
**body** | Option<[**Vec<String>**](String.md)> | List of permissions for the specified role |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_modify_deduplication

> namespaces_modify_deduplication(tenant, namespace, body)
Enable or disable broker side deduplication for all topics in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **bool** | Flag for disabling or enabling broker side deduplication for all topics in the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_modify_encryption_required

> namespaces_modify_encryption_required(tenant, namespace, body)
Message encryption is required or not for all topics in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **bool** | Flag defining if message encryption is required | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_auto_subscription_creation

> namespaces_remove_auto_subscription_creation(tenant, namespace)
Remove override of broker's allowAutoSubscriptionCreation in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_auto_topic_creation

> namespaces_remove_auto_topic_creation(tenant, namespace)
Remove override of broker's allowAutoTopicCreation in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_backlog_quota

> namespaces_remove_backlog_quota(tenant, namespace, backlog_quota_type)
Remove a backlog quota policy from a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**backlog_quota_type** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_deduplication

> namespaces_remove_deduplication(tenant, namespace)
Remove broker side deduplication for all topics in a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_delayed_delivery_policies

> namespaces_remove_delayed_delivery_policies(tenant, namespace)
Delete delayed delivery messages config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_dispatcher_pause_on_ack_state_persistent

> namespaces_remove_dispatcher_pause_on_ack_state_persistent(tenant, namespace)
Remove dispatcher pause on ack state persistent configuration for specified namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_inactive_topic_policies

> namespaces_remove_inactive_topic_policies(tenant, namespace)
Remove inactive topic policies from a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_max_consumers_per_subscription

> namespaces_remove_max_consumers_per_subscription(tenant, namespace)
 Set maxConsumersPerSubscription configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_max_consumers_per_topic

> namespaces_remove_max_consumers_per_topic(tenant, namespace)
Remove maxConsumersPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_max_producers_per_topic

> namespaces_remove_max_producers_per_topic(tenant, namespace)
Remove maxProducersPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_max_subscriptions_per_topic

> namespaces_remove_max_subscriptions_per_topic(tenant, namespace)
Remove maxSubscriptionsPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_max_topics_per_namespace

> namespaces_remove_max_topics_per_namespace(tenant, namespace)
Remove maxTopicsPerNamespace config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_max_unackedmessages_per_consumer

> namespaces_remove_max_unackedmessages_per_consumer(tenant, namespace)
Remove maxUnackedMessagesPerConsumer config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_max_unackedmessages_per_subscription

> namespaces_remove_max_unackedmessages_per_subscription(tenant, namespace)
Remove maxUnackedMessagesPerSubscription config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_namespace_anti_affinity_group

> namespaces_remove_namespace_anti_affinity_group(tenant, namespace)
Remove anti-affinity group of a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_namespace_entry_filters

> namespaces_remove_namespace_entry_filters(tenant, namespace)
Remove entry filters for namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_namespace_message_ttl

> namespaces_remove_namespace_message_ttl(tenant, namespace)
Remove message TTL in seconds for namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_namespace_resource_group

> namespaces_remove_namespace_resource_group(tenant, namespace)
Delete resourcegroup for a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_offload_policies

> namespaces_remove_offload_policies(tenant, namespace)
 Set offload configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_property

> namespaces_remove_property(tenant, namespace, key)
Remove property value for a given key on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_replicator_dispatch_rate

> namespaces_remove_replicator_dispatch_rate(tenant, namespace)
Remove replicator dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_retention

> namespaces_remove_retention(tenant, namespace, body)
 Remove retention configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**RetentionPolicies**](RetentionPolicies.md)> | Retention policies for the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_subscription_expiration_time

> namespaces_remove_subscription_expiration_time(tenant, namespace)
Remove subscription expiration time for namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_remove_subscription_types_enabled

> namespaces_remove_subscription_types_enabled(tenant, namespace)
 Remove subscription types enabled on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_revoke_permissions_on_namespace

> namespaces_revoke_permissions_on_namespace(tenant, namespace, role)
Revoke all permissions to a role on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**role** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_scan_offloaded_ledgers

> namespaces_scan_offloaded_ledgers(tenant, namespace)
Trigger the scan of offloaded Ledgers on the LedgerOffloader for the given namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_auto_subscription_creation

> namespaces_set_auto_subscription_creation(tenant, namespace, body)
Override broker's allowAutoSubscriptionCreation setting for a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**AutoSubscriptionCreationOverride**](AutoSubscriptionCreationOverride.md)> | Settings for automatic subscription creation |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_auto_topic_creation

> namespaces_set_auto_topic_creation(tenant, namespace, body)
Override broker's allowAutoTopicCreation setting for a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | [**AutoTopicCreationOverride**](AutoTopicCreationOverride.md) | Settings for automatic topic creation | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_backlog_quota

> namespaces_set_backlog_quota(tenant, namespace, backlog_quota_type, body)
 Set a backlog quota for all the topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**backlog_quota_type** | Option<**String**> |  |  |
**body** | Option<[**BacklogQuota**](BacklogQuota.md)> | Backlog quota for all topics of the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_bookie_affinity_group

> namespaces_set_bookie_affinity_group(tenant, namespace, body)
Set the bookie-affinity-group to namespace-persistent policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**BookieAffinityGroupData**](BookieAffinityGroupData.md)> | Bookie affinity group for the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_compaction_threshold

> namespaces_set_compaction_threshold(tenant, namespace, body)
Set maximum number of uncompacted bytes in a topic before compaction is triggered.

The backlog size is compared to the threshold periodically. A threshold of 0 disabled automatic compaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i64** | Maximum number of uncompacted bytes in a topic of the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_deduplication_snapshot_interval

> namespaces_set_deduplication_snapshot_interval(tenant, namespace, body)
Set deduplicationSnapshotInterval config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Interval to take deduplication snapshot per topic | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_delayed_delivery_policies

> namespaces_set_delayed_delivery_policies(tenant, namespace, body)
Set delayed delivery messages config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**DelayedDeliveryPolicies**](DelayedDeliveryPolicies.md)> | Delayed delivery policies for the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_dispatch_rate

> namespaces_set_dispatch_rate(tenant, namespace, body)
Set dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**DispatchRateImpl**](DispatchRateImpl.md)> | Dispatch rate for all topics of the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_dispatcher_pause_on_ack_state_persistent

> namespaces_set_dispatcher_pause_on_ack_state_persistent(tenant, namespace)
Set dispatcher pause on ack state persistent configuration for specified namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_entry_filters_per_topic

> namespaces_set_entry_filters_per_topic(tenant, namespace, body)
Set entry filters for namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | [**EntryFilters**](EntryFilters.md) | entry filters | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_inactive_topic_policies

> namespaces_set_inactive_topic_policies(tenant, namespace, body)
Set inactive topic policies config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**InactiveTopicPolicies**](InactiveTopicPolicies.md)> | Inactive topic policies for the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_is_allow_auto_update_schema

> namespaces_set_is_allow_auto_update_schema(tenant, namespace, body)
Update flag of whether allow auto update schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **bool** | Flag of whether to allow auto update schema | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_max_consumers_per_subscription

> namespaces_set_max_consumers_per_subscription(tenant, namespace, body)
 Set maxConsumersPerSubscription configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Number of maximum consumers per subscription | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_max_consumers_per_topic

> namespaces_set_max_consumers_per_topic(tenant, namespace, body)
 Set maxConsumersPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Number of maximum consumers per topic | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_max_producers_per_topic

> namespaces_set_max_producers_per_topic(tenant, namespace, body)
 Set maxProducersPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Number of maximum producers per topic | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_max_subscriptions_per_topic

> namespaces_set_max_subscriptions_per_topic(tenant, namespace, body)
 Set maxSubscriptionsPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Number of maximum subscriptions per topic | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_max_topics_per_namespace

> namespaces_set_max_topics_per_namespace(tenant, namespace, body)
Set maxTopicsPerNamespace config on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Number of maximum topics for specific namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_max_unacked_messages_per_consumer

> namespaces_set_max_unacked_messages_per_consumer(tenant, namespace, body)
 Set maxConsumersPerTopic configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Number of maximum unacked messages per consumer | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_max_unacked_messages_per_subscription

> namespaces_set_max_unacked_messages_per_subscription(tenant, namespace, body)
 Set maxUnackedMessagesPerSubscription configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Number of maximum unacked messages per subscription | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_namespace_anti_affinity_group

> namespaces_set_namespace_anti_affinity_group(tenant, namespace, body)
Set anti-affinity group for a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **String** | Anti-affinity group for the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_namespace_message_ttl

> namespaces_set_namespace_message_ttl(tenant, namespace, body)
Set message TTL in seconds for namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | TTL in seconds for the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_namespace_replication_clusters

> namespaces_set_namespace_replication_clusters(tenant, namespace, body)
Set the replication clusters for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | [**Vec<String>**](String.md) | List of replication clusters | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_namespace_resource_group

> namespaces_set_namespace_resource_group(tenant, namespace, resourcegroup)
Set resourcegroup for a namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**resourcegroup** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_offload_deletion_lag

> namespaces_set_offload_deletion_lag(tenant, namespace, body)
Set number of milliseconds to wait before deleting a ledger segment which has been offloaded from the Pulsar cluster's local storage (i.e. BookKeeper)

A negative value disables the deletion completely.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i64** | New number of milliseconds to wait before deleting a ledger segment which has been offloaded | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_offload_policies

> namespaces_set_offload_policies(tenant, namespace, body)
 Set offload configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | [**OffloadPoliciesImpl**](OffloadPoliciesImpl.md) | Offload policies for the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_offload_threshold

> namespaces_set_offload_threshold(tenant, namespace, body)
Set maximum number of bytes stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage

-1 will revert to using the cluster default. A negative value disables automatic offloading. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i64** | Maximum number of bytes stored on the pulsar cluster for a topic of the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_offload_threshold_in_seconds

> namespaces_set_offload_threshold_in_seconds(tenant, namespace)
Set maximum number of seconds stored on the pulsar cluster for a topic, before the broker will start offloading to longterm storage

A negative value disables automatic offloading

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_persistence

> namespaces_set_persistence(tenant, namespace, body)
Set the persistence configuration for all the topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | [**PersistencePolicies**](PersistencePolicies.md) | Persistence policies for the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_properties

> namespaces_set_properties(tenant, namespace, body)
Put key value pairs property on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | [**std::collections::HashMap<String, String>**](String.md) | Key value pair properties for the namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_property

> namespaces_set_property(tenant, namespace, key, value)
Put a key value pair property on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**key** | **String** |  | [required] |
**value** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_replicator_dispatch_rate

> namespaces_set_replicator_dispatch_rate(tenant, namespace, body)
Set replicator dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**DispatchRateImpl**](DispatchRateImpl.md)> | Replicator dispatch rate for all topics of the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_retention

> namespaces_set_retention(tenant, namespace, body)
 Set retention configuration on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**RetentionPolicies**](RetentionPolicies.md)> | Retention policies for the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_schema_auto_update_compatibility_strategy

> namespaces_set_schema_auto_update_compatibility_strategy(tenant, namespace, body)
Update the strategy used to check the compatibility of new schemas, provided by producers, before automatically updating the schema

The value AutoUpdateDisabled prevents producers from updating the schema.  If set to AutoUpdateDisabled, schemas must be updated through the REST api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<**String**> | Strategy used to check the compatibility of new schemas |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_schema_compatibility_strategy

> namespaces_set_schema_compatibility_strategy(tenant, namespace, body)
Update the strategy used to check the compatibility of new schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<**String**> | Strategy used to check the compatibility of new schema |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_schema_validation_enforced

> namespaces_set_schema_validation_enforced(tenant, namespace, body)
Set schema validation enforced flag on namespace.

If the flag is set to true, when a producer without a schema attempts to produce to a topic with schema in this namespace, the producer will be failed to connect. PLEASE be carefully on using this, since non-java clients don't support schema.if you enable this setting, it will cause non-java clients failed to produce.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **bool** | Flag of whether validation is enforced on the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_subscribe_rate

> namespaces_set_subscribe_rate(tenant, namespace, body)
Set subscribe-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**SubscribeRate**](SubscribeRate.md)> | Subscribe rate for all topics of the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_subscription_auth_mode

> namespaces_set_subscription_auth_mode(tenant, namespace, body)
 Set a subscription auth mode for all the topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<**String**> | Subscription auth mode for all topics of the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_subscription_dispatch_rate

> namespaces_set_subscription_dispatch_rate(tenant, namespace, body)
Set Subscription dispatch-rate throttling for all topics of the namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | Option<[**DispatchRateImpl**](DispatchRateImpl.md)> | Subscription dispatch rate for all topics of the specified namespace |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_subscription_expiration_time

> namespaces_set_subscription_expiration_time(tenant, namespace, body)
Set subscription expiration time in minutes for namespace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | **i32** | Expiration time in minutes for the specified namespace | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_set_subscription_types_enabled

> namespaces_set_subscription_types_enabled(tenant, namespace, body)
Update set of whether allow share sub type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**body** | [**Vec<String>**](String.md) | Set of whether allow subscription types | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_split_namespace_bundle

> namespaces_split_namespace_bundle(tenant, namespace, bundle, authoritative, unload, split_algorithm_name, body)
Split a namespace bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**bundle** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]
**unload** | Option<**bool**> |  |  |[default to false]
**split_algorithm_name** | Option<**String**> |  |  |
**body** | Option<[**Vec<i64>**](i64.md)> | splitBoundaries |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_unload_namespace

> namespaces_unload_namespace(tenant, namespace)
Unload namespace

Unload an active namespace from the current broker serving it. Performing this operation will let the brokerremoves all producers, consumers, and connections using this namespace, and close all topics (includingtheir persistent store). During that operation, the namespace is marked as tentatively unavailable until thebroker completes the unloading action. This operation requires strictly super user privileges, since it wouldresult in non-persistent message loss and unexpected connection closure to the clients.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_unload_namespace_bundle

> namespaces_unload_namespace_bundle(tenant, namespace, bundle, authoritative, destination_broker)
Unload a namespace bundle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**bundle** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]
**destination_broker** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_unsubscribe_namespace

> namespaces_unsubscribe_namespace(tenant, namespace, subscription, authoritative)
Unsubscribes the given subscription on all topics on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**subscription** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## namespaces_unsubscribe_namespace_bundle

> namespaces_unsubscribe_namespace_bundle(tenant, namespace, subscription, bundle, authoritative)
Unsubscribes the given subscription on all topics on a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**subscription** | **String** |  | [required] |
**bundle** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

