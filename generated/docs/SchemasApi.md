# \SchemasApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**schemas_resource_delete_schema**](SchemasApi.md#schemas_resource_delete_schema) | **DELETE** /schemas/{tenant}/{namespace}/{topic}/schema | Delete all versions schema of a topic
[**schemas_resource_get_all_schemas**](SchemasApi.md#schemas_resource_get_all_schemas) | **GET** /schemas/{tenant}/{namespace}/{topic}/schemas | Get the all schemas of a topic
[**schemas_resource_get_schema**](SchemasApi.md#schemas_resource_get_schema) | **GET** /schemas/{tenant}/{namespace}/{topic}/schema | Get the schema of a topic
[**schemas_resource_get_schema_0**](SchemasApi.md#schemas_resource_get_schema_0) | **GET** /schemas/{tenant}/{namespace}/{topic}/schema/{version} | Get the schema of a topic at a given version
[**schemas_resource_get_version_by_schema**](SchemasApi.md#schemas_resource_get_version_by_schema) | **POST** /schemas/{tenant}/{namespace}/{topic}/version | get the version of the schema
[**schemas_resource_post_schema**](SchemasApi.md#schemas_resource_post_schema) | **POST** /schemas/{tenant}/{namespace}/{topic}/schema | Update the schema of a topic
[**schemas_resource_test_compatibility**](SchemasApi.md#schemas_resource_test_compatibility) | **POST** /schemas/{tenant}/{namespace}/{topic}/compatibility | test the schema compatibility



## schemas_resource_delete_schema

> models::DeleteSchemaResponse schemas_resource_delete_schema(tenant, namespace, topic, authoritative, force)
Delete all versions schema of a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]
**force** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::DeleteSchemaResponse**](DeleteSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_resource_get_all_schemas

> models::GetAllVersionsSchemaResponse schemas_resource_get_all_schemas(tenant, namespace, topic, authoritative)
Get the all schemas of a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::GetAllVersionsSchemaResponse**](GetAllVersionsSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_resource_get_schema

> models::GetSchemaResponse schemas_resource_get_schema(tenant, namespace, topic, authoritative)
Get the schema of a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::GetSchemaResponse**](GetSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_resource_get_schema_0

> models::GetSchemaResponse schemas_resource_get_schema_0(tenant, namespace, topic, version, authoritative)
Get the schema of a topic at a given version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**version** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::GetSchemaResponse**](GetSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_resource_get_version_by_schema

> models::LongSchemaVersion schemas_resource_get_version_by_schema(tenant, namespace, topic, authoritative, body)
get the version of the schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**PostSchemaPayload**](PostSchemaPayload.md)> | A JSON value presenting a schema payload. An example of the expected schema can be found down here. |  |

### Return type

[**models::LongSchemaVersion**](LongSchemaVersion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_resource_post_schema

> models::PostSchemaResponse schemas_resource_post_schema(tenant, namespace, topic, authoritative, body)
Update the schema of a topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**PostSchemaPayload**](PostSchemaPayload.md)> | A JSON value presenting a schema payload. An example of the expected schema can be found down here. |  |

### Return type

[**models::PostSchemaResponse**](PostSchemaResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemas_resource_test_compatibility

> models::IsCompatibilityResponse schemas_resource_test_compatibility(tenant, namespace, topic, authoritative, body)
test the schema compatibility

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |
**namespace** | **String** |  | [required] |
**topic** | **String** |  | [required] |
**authoritative** | Option<**bool**> |  |  |[default to false]
**body** | Option<[**PostSchemaPayload**](PostSchemaPayload.md)> | A JSON value presenting a schema payload. An example of the expected schema can be found down here. |  |

### Return type

[**models::IsCompatibilityResponse**](IsCompatibilityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

