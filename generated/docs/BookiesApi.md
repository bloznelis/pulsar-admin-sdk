# \BookiesApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bookies_delete_bookie_rack_info**](BookiesApi.md#bookies_delete_bookie_rack_info) | **DELETE** /bookies/racks-info/{bookie} | Removed the rack placement information for a specific bookie in the cluster
[**bookies_get_all_bookies**](BookiesApi.md#bookies_get_all_bookies) | **GET** /bookies/all | Gets raw information for all the bookies in the cluster
[**bookies_get_bookie_rack_info**](BookiesApi.md#bookies_get_bookie_rack_info) | **GET** /bookies/racks-info/{bookie} | Gets the rack placement information for a specific bookie in the cluster
[**bookies_get_bookies_rack_info**](BookiesApi.md#bookies_get_bookies_rack_info) | **GET** /bookies/racks-info | Gets the rack placement information for all the bookies in the cluster
[**bookies_update_bookie_rack_info**](BookiesApi.md#bookies_update_bookie_rack_info) | **POST** /bookies/racks-info/{bookie} | Updates the rack placement information for a specific bookie in the cluster (note. bookie address format:`address:port`)



## bookies_delete_bookie_rack_info

> bookies_delete_bookie_rack_info(bookie)
Removed the rack placement information for a specific bookie in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bookie** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bookies_get_all_bookies

> models::BookiesClusterInfo bookies_get_all_bookies()
Gets raw information for all the bookies in the cluster

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::BookiesClusterInfo**](BookiesClusterInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bookies_get_bookie_rack_info

> models::BookieInfo bookies_get_bookie_rack_info(bookie)
Gets the rack placement information for a specific bookie in the cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bookie** | **String** |  | [required] |

### Return type

[**models::BookieInfo**](BookieInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bookies_get_bookies_rack_info

> std::collections::HashMap<String, std::collections::HashMap<String, models::BookieInfo>> bookies_get_bookies_rack_info()
Gets the rack placement information for all the bookies in the cluster

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, std::collections::HashMap<String, models::BookieInfo>>**](std::collections::HashMap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bookies_update_bookie_rack_info

> bookies_update_bookie_rack_info(bookie, group)
Updates the rack placement information for a specific bookie in the cluster (note. bookie address format:`address:port`)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bookie** | **String** |  | [required] |
**group** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

