# SubscriptionStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_consumer_name** | Option<**String**> |  | [optional]
**allow_out_of_order_delivery** | Option<**bool**> |  | [optional]
**backlog_size** | Option<**i64**> |  | [optional]
**blocked_subscription_on_unacked_msgs** | Option<**bool**> |  | [optional]
**bytes_out_counter** | Option<**i64**> |  | [optional]
**chunked_message_rate** | Option<**i32**> |  | [optional]
**consumers** | Option<[**Vec<models::ConsumerStats>**](ConsumerStats.md)> |  | [optional]
**consumers_after_mark_delete_position** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**delayed_message_index_size_in_bytes** | Option<**i64**> |  | [optional]
**durable** | Option<**bool**> |  | [optional]
**earliest_msg_publish_time_in_backlog** | Option<**i64**> |  | [optional]
**filter_accepted_msg_count** | Option<**i64**> |  | [optional]
**filter_processed_msg_count** | Option<**i64**> |  | [optional]
**filter_rejected_msg_count** | Option<**i64**> |  | [optional]
**filter_rescheduled_msg_count** | Option<**i64**> |  | [optional]
**key_shared_mode** | Option<**String**> |  | [optional]
**last_acked_timestamp** | Option<**i64**> |  | [optional]
**last_consumed_flow_timestamp** | Option<**i64**> |  | [optional]
**last_consumed_timestamp** | Option<**i64**> |  | [optional]
**last_expire_timestamp** | Option<**i64**> |  | [optional]
**last_mark_delete_advanced_timestamp** | Option<**i64**> |  | [optional]
**message_ack_rate** | Option<**f64**> |  | [optional]
**msg_backlog** | Option<**i64**> |  | [optional]
**msg_backlog_no_delayed** | Option<**i64**> |  | [optional]
**msg_delayed** | Option<**i64**> |  | [optional]
**msg_out_counter** | Option<**i64**> |  | [optional]
**msg_rate_expired** | Option<**f64**> |  | [optional]
**msg_rate_out** | Option<**f64**> |  | [optional]
**msg_rate_redeliver** | Option<**f64**> |  | [optional]
**msg_throughput_out** | Option<**f64**> |  | [optional]
**non_contiguous_deleted_messages_ranges** | Option<**i32**> |  | [optional]
**non_contiguous_deleted_messages_ranges_serialized_size** | Option<**i32**> |  | [optional]
**replicated** | Option<**bool**> |  | [optional]
**subscription_properties** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**total_msg_expired** | Option<**i64**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**unacked_messages** | Option<**i64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


