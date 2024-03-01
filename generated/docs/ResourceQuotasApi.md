# \ResourceQuotasApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**resource_quotas_get_default_resource_quota**](ResourceQuotasApi.md#resource_quotas_get_default_resource_quota) | **GET** /resource-quotas | Get the default quota
[**resource_quotas_get_namespace_bundle_resource_quota**](ResourceQuotasApi.md#resource_quotas_get_namespace_bundle_resource_quota) | **GET** /resource-quotas/{tenant}/{namespace}/{bundle} | Get resource quota of a namespace bundle.
[**resource_quotas_remove_namespace_bundle_resource_quota**](ResourceQuotasApi.md#resource_quotas_remove_namespace_bundle_resource_quota) | **DELETE** /resource-quotas/{tenant}/{namespace}/{bundle} | Remove resource quota for a namespace.
[**resource_quotas_set_default_resource_quota**](ResourceQuotasApi.md#resource_quotas_set_default_resource_quota) | **POST** /resource-quotas | Set the default quota
[**resource_quotas_set_namespace_bundle_resource_quota**](ResourceQuotasApi.md#resource_quotas_set_namespace_bundle_resource_quota) | **POST** /resource-quotas/{tenant}/{namespace}/{bundle} | Set resource quota on a namespace.



## resource_quotas_get_default_resource_quota

> Vec<String> resource_quotas_get_default_resource_quota()
Get the default quota

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_quotas_get_namespace_bundle_resource_quota

> resource_quotas_get_namespace_bundle_resource_quota(tenant, namespace, bundle)
Get resource quota of a namespace bundle.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Tenant name | [required] |
**namespace** | **String** | Namespace name within the specified tenant | [required] |
**bundle** | **String** | Namespace bundle range | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_quotas_remove_namespace_bundle_resource_quota

> resource_quotas_remove_namespace_bundle_resource_quota(tenant, namespace, bundle)
Remove resource quota for a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Tenant name | [required] |
**namespace** | **String** | Namespace name within the specified tenant | [required] |
**bundle** | **String** | Namespace bundle range | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_quotas_set_default_resource_quota

> Vec<String> resource_quotas_set_default_resource_quota(body)
Set the default quota

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ResourceQuota**](ResourceQuota.md)> | Default resource quota |  |

### Return type

**Vec<String>**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_quotas_set_namespace_bundle_resource_quota

> resource_quotas_set_namespace_bundle_resource_quota(tenant, namespace, bundle, body)
Set resource quota on a namespace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Tenant name | [required] |
**namespace** | **String** | Namespace name within the specified tenant | [required] |
**bundle** | **String** | Namespace bundle range | [required] |
**body** | Option<[**ResourceQuota**](ResourceQuota.md)> | Resource quota for the specified namespace |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

