# TopicStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**average_msg_size** | Option<**f64**> |  | [optional]
**backlog_quota_limit_size** | Option<**i64**> |  | [optional]
**backlog_quota_limit_time** | Option<**i64**> |  | [optional]
**backlog_size** | Option<**i64**> |  | [optional]
**bytes_in_counter** | Option<**i64**> |  | [optional]
**bytes_out_counter** | Option<**i64**> |  | [optional]
**compaction** | Option<[**models::CompactionStats**](CompactionStats.md)> |  | [optional]
**deduplication_status** | Option<**String**> |  | [optional]
**delayed_message_index_size_in_bytes** | Option<**i64**> |  | [optional]
**earliest_msg_publish_time_in_backlogs** | Option<**i64**> |  | [optional]
**msg_chunk_published** | Option<**bool**> |  | [optional]
**msg_in_counter** | Option<**i64**> |  | [optional]
**msg_out_counter** | Option<**i64**> |  | [optional]
**msg_rate_in** | Option<**f64**> |  | [optional]
**msg_rate_out** | Option<**f64**> |  | [optional]
**msg_throughput_in** | Option<**f64**> |  | [optional]
**msg_throughput_out** | Option<**f64**> |  | [optional]
**non_contiguous_deleted_messages_ranges** | Option<**i32**> |  | [optional]
**non_contiguous_deleted_messages_ranges_serialized_size** | Option<**i32**> |  | [optional]
**offloaded_storage_size** | Option<**i64**> |  | [optional]
**oldest_backlog_message_age_seconds** | Option<**i64**> |  | [optional]
**oldest_backlog_message_subscription_name** | Option<**String**> |  | [optional]
**owner_broker** | Option<**String**> |  | [optional]
**publishers** | Option<[**Vec<models::PublisherStats>**](PublisherStats.md)> |  | [optional]
**replication** | Option<[**std::collections::HashMap<String, models::ReplicatorStats>**](ReplicatorStats.md)> |  | [optional]
**storage_size** | Option<**i64**> |  | [optional]
**subscriptions** | Option<[**std::collections::HashMap<String, models::SubscriptionStats>**](SubscriptionStats.md)> |  | [optional]
**topic_epoch** | Option<**i64**> |  | [optional]
**waiting_publishers** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


