# \BrokersApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**brokers_base_backlog_quota_check**](BrokersApi.md#brokers_base_backlog_quota_check) | **GET** /brokers/backlog-quota-check | An REST endpoint to trigger backlogQuotaCheck
[**brokers_base_delete_dynamic_configuration**](BrokersApi.md#brokers_base_delete_dynamic_configuration) | **DELETE** /brokers/configuration/{configName} | Delete dynamic ServiceConfiguration into metadata only. This operation requires Pulsar super-user privileges.
[**brokers_base_get_active_brokers**](BrokersApi.md#brokers_base_get_active_brokers) | **GET** /brokers | Get the list of active brokers (broker ids) in the local cluster.If authorization is not enabled
[**brokers_base_get_active_brokers_0**](BrokersApi.md#brokers_base_get_active_brokers_0) | **GET** /brokers/{cluster} | Get the list of active brokers (broker ids) in the cluster.If authorization is not enabled, any cluster name is valid.
[**brokers_base_get_all_dynamic_configurations**](BrokersApi.md#brokers_base_get_all_dynamic_configurations) | **GET** /brokers/configuration/values | Get value of all dynamic configurations' value overridden on local config
[**brokers_base_get_dynamic_configuration_name**](BrokersApi.md#brokers_base_get_dynamic_configuration_name) | **GET** /brokers/configuration | Get all updatable dynamic configurations's name
[**brokers_base_get_internal_configuration_data**](BrokersApi.md#brokers_base_get_internal_configuration_data) | **GET** /brokers/internal-configuration | Get the internal configuration data
[**brokers_base_get_leader_broker**](BrokersApi.md#brokers_base_get_leader_broker) | **GET** /brokers/leaderBroker | Get the information of the leader broker.
[**brokers_base_get_owned_namespaces**](BrokersApi.md#brokers_base_get_owned_namespaces) | **GET** /brokers/{clusterName}/{brokerId}/ownedNamespaces | Get the list of namespaces served by the specific broker id
[**brokers_base_get_runtime_configuration**](BrokersApi.md#brokers_base_get_runtime_configuration) | **GET** /brokers/configuration/runtime | Get all runtime configurations. This operation requires Pulsar super-user privileges.
[**brokers_base_health_check**](BrokersApi.md#brokers_base_health_check) | **GET** /brokers/health | Run a healthCheck against the broker
[**brokers_base_is_ready**](BrokersApi.md#brokers_base_is_ready) | **GET** /brokers/ready | Check if the broker is fully initialized
[**brokers_base_shut_down_broker_gracefully**](BrokersApi.md#brokers_base_shut_down_broker_gracefully) | **POST** /brokers/shutdown | Shutdown broker gracefully.
[**brokers_base_update_dynamic_configuration**](BrokersApi.md#brokers_base_update_dynamic_configuration) | **POST** /brokers/configuration/{configName}/{configValue} | Update dynamic serviceconfiguration into zk only. This operation requires Pulsar super-user privileges.
[**brokers_base_version**](BrokersApi.md#brokers_base_version) | **GET** /brokers/version | Get version of current broker



## brokers_base_backlog_quota_check

> brokers_base_backlog_quota_check()
An REST endpoint to trigger backlogQuotaCheck

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_delete_dynamic_configuration

> brokers_base_delete_dynamic_configuration(config_name)
Delete dynamic ServiceConfiguration into metadata only. This operation requires Pulsar super-user privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_get_active_brokers

> Vec<String> brokers_base_get_active_brokers()
Get the list of active brokers (broker ids) in the local cluster.If authorization is not enabled

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_get_active_brokers_0

> Vec<String> brokers_base_get_active_brokers_0(cluster)
Get the list of active brokers (broker ids) in the cluster.If authorization is not enabled, any cluster name is valid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** |  | [required] |

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_get_all_dynamic_configurations

> brokers_base_get_all_dynamic_configurations()
Get value of all dynamic configurations' value overridden on local config

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_get_dynamic_configuration_name

> brokers_base_get_dynamic_configuration_name()
Get all updatable dynamic configurations's name

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_get_internal_configuration_data

> models::InternalConfigurationData brokers_base_get_internal_configuration_data()
Get the internal configuration data

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InternalConfigurationData**](InternalConfigurationData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_get_leader_broker

> models::BrokerInfo brokers_base_get_leader_broker()
Get the information of the leader broker.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::BrokerInfo**](BrokerInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_get_owned_namespaces

> std::collections::HashMap<String, models::NamespaceOwnershipStatus> brokers_base_get_owned_namespaces(cluster_name, broker_id)
Get the list of namespaces served by the specific broker id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_name** | **String** |  | [required] |
**broker_id** | **String** |  | [required] |

### Return type

[**std::collections::HashMap<String, models::NamespaceOwnershipStatus>**](NamespaceOwnershipStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_get_runtime_configuration

> brokers_base_get_runtime_configuration()
Get all runtime configurations. This operation requires Pulsar super-user privileges.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_health_check

> brokers_base_health_check(topic_version)
Run a healthCheck against the broker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_version** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_is_ready

> brokers_base_is_ready()
Check if the broker is fully initialized

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_shut_down_broker_gracefully

> brokers_base_shut_down_broker_gracefully(max_concurrent_unload_per_sec, forced_terminate_topic)
Shutdown broker gracefully.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_concurrent_unload_per_sec** | Option<**i32**> | if the value absent(value=0) means no concurrent limitation. |  |
**forced_terminate_topic** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_update_dynamic_configuration

> brokers_base_update_dynamic_configuration(config_name, config_value)
Update dynamic serviceconfiguration into zk only. This operation requires Pulsar super-user privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config_name** | **String** |  | [required] |
**config_value** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brokers_base_version

> String brokers_base_version()
Get version of current broker

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

