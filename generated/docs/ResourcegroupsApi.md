# \ResourcegroupsApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**resource_groups_create_or_update_resource_group**](ResourcegroupsApi.md#resource_groups_create_or_update_resource_group) | **PUT** /resourcegroups/{resourcegroup} | Creates a new resourcegroup with the specified rate limiters
[**resource_groups_delete_resource_group**](ResourcegroupsApi.md#resource_groups_delete_resource_group) | **DELETE** /resourcegroups/{resourcegroup} | Delete a resourcegroup.
[**resource_groups_get_resource_group**](ResourcegroupsApi.md#resource_groups_get_resource_group) | **GET** /resourcegroups/{resourcegroup} | Get the rate limiters specified for a resourcegroup.
[**resource_groups_get_resource_groups**](ResourcegroupsApi.md#resource_groups_get_resource_groups) | **GET** /resourcegroups | Get the list of all the resourcegroups.



## resource_groups_create_or_update_resource_group

> resource_groups_create_or_update_resource_group(resourcegroup, body)
Creates a new resourcegroup with the specified rate limiters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resourcegroup** | **String** |  | [required] |
**body** | Option<[**ResourceGroup**](ResourceGroup.md)> | Rate limiters for the resourcegroup |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_groups_delete_resource_group

> resource_groups_delete_resource_group(resourcegroup)
Delete a resourcegroup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resourcegroup** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_groups_get_resource_group

> models::ResourceGroup resource_groups_get_resource_group(resourcegroup)
Get the rate limiters specified for a resourcegroup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resourcegroup** | **String** |  | [required] |

### Return type

[**models::ResourceGroup**](ResourceGroup.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resource_groups_get_resource_groups

> Vec<String> resource_groups_get_resource_groups()
Get the list of all the resourcegroups.

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

