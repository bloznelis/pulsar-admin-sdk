# \DefaultApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**worker_drain**](DefaultApi.md#worker_drain) | **PUT** /worker/drain | Drains this worker, i.e., moves its work-assignments to other workers
[**worker_drain_at_leader**](DefaultApi.md#worker_drain_at_leader) | **PUT** /worker/leader/drain | Drains the specified worker, i.e., moves its work-assignments to other workers
[**worker_get_assignments**](DefaultApi.md#worker_get_assignments) | **GET** /worker/assignments | Fetches information about which Pulsar Functions are assigned to which Pulsar clusters
[**worker_get_cluster**](DefaultApi.md#worker_get_cluster) | **GET** /worker/cluster | Fetches information about the Pulsar cluster running Pulsar Functions
[**worker_get_cluster_leader**](DefaultApi.md#worker_get_cluster_leader) | **GET** /worker/cluster/leader | Fetches info about the leader node of the Pulsar cluster running Pulsar Functions
[**worker_get_connectors_list**](DefaultApi.md#worker_get_connectors_list) | **GET** /worker/connectors | Fetches a list of supported Pulsar IO connectors currently running in cluster mode
[**worker_get_drain_status**](DefaultApi.md#worker_get_drain_status) | **GET** /worker/drain | Get the status of any ongoing drain operation at this worker
[**worker_get_drain_status_from_leader**](DefaultApi.md#worker_get_drain_status_from_leader) | **GET** /worker/leader/drain | Get the status of any ongoing drain operation at the specified worker
[**worker_is_leader_ready**](DefaultApi.md#worker_is_leader_ready) | **GET** /worker/cluster/leader/ready | Checks if this node is the leader and is ready to service requests
[**worker_rebalance**](DefaultApi.md#worker_rebalance) | **PUT** /worker/rebalance | Triggers a rebalance of functions to workers
[**worker_stats_get_metrics**](DefaultApi.md#worker_stats_get_metrics) | **GET** /worker-stats/metrics | Gets the metrics for Monitoring
[**worker_stats_get_stats**](DefaultApi.md#worker_stats_get_stats) | **GET** /worker-stats/functionsmetrics | Get metrics for all functions owned by worker



## worker_drain

> worker_drain()
Drains this worker, i.e., moves its work-assignments to other workers

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_drain_at_leader

> worker_drain_at_leader(worker_id)
Drains the specified worker, i.e., moves its work-assignments to other workers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**worker_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_get_assignments

> std::collections::HashMap<String, serde_json::Value> worker_get_assignments()
Fetches information about which Pulsar Functions are assigned to which Pulsar clusters

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_get_cluster

> Vec<models::WorkerInfo> worker_get_cluster()
Fetches information about the Pulsar cluster running Pulsar Functions

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::WorkerInfo>**](WorkerInfo.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_get_cluster_leader

> models::WorkerInfo worker_get_cluster_leader()
Fetches info about the leader node of the Pulsar cluster running Pulsar Functions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WorkerInfo**](WorkerInfo.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_get_connectors_list

> Vec<serde_json::Value> worker_get_connectors_list()
Fetches a list of supported Pulsar IO connectors currently running in cluster mode

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_get_drain_status

> models::LongRunningProcessStatus worker_get_drain_status()
Get the status of any ongoing drain operation at this worker

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LongRunningProcessStatus**](LongRunningProcessStatus.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_get_drain_status_from_leader

> models::LongRunningProcessStatus worker_get_drain_status_from_leader(worker_id)
Get the status of any ongoing drain operation at the specified worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**worker_id** | Option<**String**> |  |  |

### Return type

[**models::LongRunningProcessStatus**](LongRunningProcessStatus.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_is_leader_ready

> bool worker_is_leader_ready()
Checks if this node is the leader and is ready to service requests

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_rebalance

> worker_rebalance()
Triggers a rebalance of functions to workers

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_stats_get_metrics

> Vec<models::Metrics> worker_stats_get_metrics()
Gets the metrics for Monitoring

Request should be executed by Monitoring agent on each worker to fetch the worker-metrics

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Metrics>**](Metrics.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## worker_stats_get_stats

> Vec<models::WorkerFunctionInstanceStats> worker_stats_get_stats()
Get metrics for all functions owned by worker

Requested should be executed by Monitoring agent on each worker to fetch the metrics

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::WorkerFunctionInstanceStats>**](WorkerFunctionInstanceStats.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

