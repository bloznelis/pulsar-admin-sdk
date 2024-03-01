# \BrokerStatsApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**broker_stats_base_get_allocator_stats**](BrokerStatsApi.md#broker_stats_base_get_allocator_stats) | **GET** /broker-stats/allocator-stats/{allocator} | Get the stats for the Netty allocator. Available allocators are 'default' and 'ml-cache'
[**broker_stats_base_get_load_report**](BrokerStatsApi.md#broker_stats_base_get_load_report) | **GET** /broker-stats/load-report | Get Load for this broker
[**broker_stats_base_get_m_beans**](BrokerStatsApi.md#broker_stats_base_get_m_beans) | **GET** /broker-stats/mbeans | Get all the mbean details of this broker JVM
[**broker_stats_base_get_metrics**](BrokerStatsApi.md#broker_stats_base_get_metrics) | **GET** /broker-stats/metrics | Gets the metrics for Monitoring
[**broker_stats_base_get_pending_bookie_ops_stats**](BrokerStatsApi.md#broker_stats_base_get_pending_bookie_ops_stats) | **GET** /broker-stats/bookieops | Get pending bookie client op stats by namespace
[**broker_stats_get_broker_resource_availability**](BrokerStatsApi.md#broker_stats_get_broker_resource_availability) | **GET** /broker-stats/broker-resource-availability/{tenant}/{namespace} | Broker availability report
[**broker_stats_get_topics2**](BrokerStatsApi.md#broker_stats_get_topics2) | **GET** /broker-stats/topics | Get all the topic stats by namespace



## broker_stats_base_get_allocator_stats

> models::AllocatorStats broker_stats_base_get_allocator_stats(allocator)
Get the stats for the Netty allocator. Available allocators are 'default' and 'ml-cache'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**allocator** | **String** |  | [required] |

### Return type

[**models::AllocatorStats**](AllocatorStats.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broker_stats_base_get_load_report

> models::LoadReport broker_stats_base_get_load_report()
Get Load for this broker

consists of topics stats & systemResourceUsage

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LoadReport**](LoadReport.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broker_stats_base_get_m_beans

> Vec<models::Metrics> broker_stats_base_get_m_beans()
Get all the mbean details of this broker JVM

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


## broker_stats_base_get_metrics

> Vec<models::Metrics> broker_stats_base_get_metrics()
Gets the metrics for Monitoring

Requested should be executed by Monitoring agent on each broker to fetch the metrics

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


## broker_stats_base_get_pending_bookie_ops_stats

> std::collections::HashMap<String, models::PendingBookieOpsStats> broker_stats_base_get_pending_bookie_ops_stats()
Get pending bookie client op stats by namespace

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, models::PendingBookieOpsStats>**](PendingBookieOpsStats.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broker_stats_get_broker_resource_availability

> std::collections::HashMap<String, models::ResourceUnit> broker_stats_get_broker_resource_availability(tenant, namespace)
Broker availability report

This API gives the current broker availability in percent, each resource percentage usage is calculated and thensum of all of the resource usage percent is called broker-resource-availability<br/><br/>THIS API IS ONLY FOR USE BY TESTING FOR CONFIRMING NAMESPACE ALLOCATION ALGORITHM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |

### Return type

[**std::collections::HashMap<String, models::ResourceUnit>**](ResourceUnit.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## broker_stats_get_topics2

> serde_json::Value broker_stats_get_topics2()
Get all the topic stats by namespace

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

