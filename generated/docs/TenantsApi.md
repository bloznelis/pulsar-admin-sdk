# \TenantsApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tenants_base_create_tenant**](TenantsApi.md#tenants_base_create_tenant) | **PUT** /tenants/{tenant} | Create a new tenant.
[**tenants_base_delete_tenant**](TenantsApi.md#tenants_base_delete_tenant) | **DELETE** /tenants/{tenant} | Delete a tenant and all namespaces and topics under it.
[**tenants_base_get_tenant_admin**](TenantsApi.md#tenants_base_get_tenant_admin) | **GET** /tenants/{tenant} | Get the admin configuration for a given tenant.
[**tenants_base_get_tenants**](TenantsApi.md#tenants_base_get_tenants) | **GET** /tenants | Get the list of existing tenants.
[**tenants_base_update_tenant**](TenantsApi.md#tenants_base_update_tenant) | **POST** /tenants/{tenant} | Update the admins for a tenant.



## tenants_base_create_tenant

> tenants_base_create_tenant(tenant, body)
Create a new tenant.

This operation requires Pulsar super-user privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | The tenant name | [required] |
**body** | Option<[**TenantInfo**](TenantInfo.md)> | TenantInfo |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_base_delete_tenant

> tenants_base_delete_tenant(tenant, force)
Delete a tenant and all namespaces and topics under it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | The tenant name | [required] |
**force** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_base_get_tenant_admin

> models::TenantInfo tenants_base_get_tenant_admin(tenant)
Get the admin configuration for a given tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | The tenant name | [required] |

### Return type

[**models::TenantInfo**](TenantInfo.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_base_get_tenants

> Vec<String> tenants_base_get_tenants()
Get the list of existing tenants.

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


## tenants_base_update_tenant

> tenants_base_update_tenant(tenant, body)
Update the admins for a tenant.

This operation requires Pulsar super-user privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | The tenant name | [required] |
**body** | Option<[**TenantInfo**](TenantInfo.md)> | TenantInfo |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

