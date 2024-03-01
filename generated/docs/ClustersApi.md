# \ClustersApi

All URIs are relative to */admin/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clusters_base_create_cluster**](ClustersApi.md#clusters_base_create_cluster) | **PUT** /clusters/{cluster} | Create a new cluster.
[**clusters_base_delete_cluster**](ClustersApi.md#clusters_base_delete_cluster) | **DELETE** /clusters/{cluster} | Delete an existing cluster.
[**clusters_base_delete_failure_domain**](ClustersApi.md#clusters_base_delete_failure_domain) | **DELETE** /clusters/{cluster}/failureDomains/{domainName} | Delete the failure domain of the cluster
[**clusters_base_delete_namespace_isolation_policy**](ClustersApi.md#clusters_base_delete_namespace_isolation_policy) | **DELETE** /clusters/{cluster}/namespaceIsolationPolicies/{policyName} | Delete namespace isolation policy.
[**clusters_base_get_broker_with_namespace_isolation_policy**](ClustersApi.md#clusters_base_get_broker_with_namespace_isolation_policy) | **GET** /clusters/{cluster}/namespaceIsolationPolicies/brokers/{broker} | Get a broker with namespace-isolation policies attached to it.
[**clusters_base_get_brokers_with_namespace_isolation_policy**](ClustersApi.md#clusters_base_get_brokers_with_namespace_isolation_policy) | **GET** /clusters/{cluster}/namespaceIsolationPolicies/brokers | Get list of brokers with namespace-isolation policies attached to them.
[**clusters_base_get_cluster**](ClustersApi.md#clusters_base_get_cluster) | **GET** /clusters/{cluster} | Get the configuration for the specified cluster.
[**clusters_base_get_cluster_migration**](ClustersApi.md#clusters_base_get_cluster_migration) | **GET** /clusters/{cluster}/migrate | Get the cluster migration configuration for the specified cluster.
[**clusters_base_get_clusters**](ClustersApi.md#clusters_base_get_clusters) | **GET** /clusters | Get the list of all the Pulsar clusters.
[**clusters_base_get_domain**](ClustersApi.md#clusters_base_get_domain) | **GET** /clusters/{cluster}/failureDomains/{domainName} | Get a domain in a cluster
[**clusters_base_get_failure_domains**](ClustersApi.md#clusters_base_get_failure_domains) | **GET** /clusters/{cluster}/failureDomains | Get the cluster failure domains.
[**clusters_base_get_namespace_isolation_policies**](ClustersApi.md#clusters_base_get_namespace_isolation_policies) | **GET** /clusters/{cluster}/namespaceIsolationPolicies | Get the namespace isolation policies assigned to the cluster.
[**clusters_base_get_namespace_isolation_policy**](ClustersApi.md#clusters_base_get_namespace_isolation_policy) | **GET** /clusters/{cluster}/namespaceIsolationPolicies/{policyName} | Get the single namespace isolation policy assigned to the cluster.
[**clusters_base_get_peer_cluster**](ClustersApi.md#clusters_base_get_peer_cluster) | **GET** /clusters/{cluster}/peers | Get the peer-cluster data for the specified cluster.
[**clusters_base_set_failure_domain**](ClustersApi.md#clusters_base_set_failure_domain) | **POST** /clusters/{cluster}/failureDomains/{domainName} | Set the failure domain of the cluster.
[**clusters_base_set_namespace_isolation_policy**](ClustersApi.md#clusters_base_set_namespace_isolation_policy) | **POST** /clusters/{cluster}/namespaceIsolationPolicies/{policyName} | Set namespace isolation policy.
[**clusters_base_set_peer_cluster_names**](ClustersApi.md#clusters_base_set_peer_cluster_names) | **POST** /clusters/{cluster}/peers | Update peer-cluster-list for a cluster.
[**clusters_base_update_cluster**](ClustersApi.md#clusters_base_update_cluster) | **POST** /clusters/{cluster} | Update the configuration for a cluster.
[**clusters_base_update_cluster_migration**](ClustersApi.md#clusters_base_update_cluster_migration) | **POST** /clusters/{cluster}/migrate | Update the configuration for a cluster migration.



## clusters_base_create_cluster

> clusters_base_create_cluster(cluster, body)
Create a new cluster.

This operation requires Pulsar superuser privileges, and the name cannot contain the '/' characters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**body** | [**ClusterData**](ClusterData.md) | The cluster data | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_delete_cluster

> clusters_base_delete_cluster(cluster)
Delete an existing cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_delete_failure_domain

> clusters_base_delete_failure_domain(cluster, domain_name)
Delete the failure domain of the cluster

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**domain_name** | **String** | The failure domain name | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_delete_namespace_isolation_policy

> clusters_base_delete_namespace_isolation_policy(cluster, policy_name)
Delete namespace isolation policy.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**policy_name** | **String** | The namespace isolation policy name | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_broker_with_namespace_isolation_policy

> models::BrokerNamespaceIsolationData clusters_base_get_broker_with_namespace_isolation_policy(cluster, broker)
Get a broker with namespace-isolation policies attached to it.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**broker** | **String** | The broker name (<broker-hostname>:<web-service-port>) | [required] |

### Return type

[**models::BrokerNamespaceIsolationData**](BrokerNamespaceIsolationData.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_brokers_with_namespace_isolation_policy

> Vec<models::BrokerNamespaceIsolationData> clusters_base_get_brokers_with_namespace_isolation_policy(cluster)
Get list of brokers with namespace-isolation policies attached to them.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |

### Return type

[**Vec<models::BrokerNamespaceIsolationData>**](BrokerNamespaceIsolationData.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_cluster

> models::ClusterData clusters_base_get_cluster(cluster)
Get the configuration for the specified cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |

### Return type

[**models::ClusterData**](ClusterData.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_cluster_migration

> models::ClusterData clusters_base_get_cluster_migration(cluster)
Get the cluster migration configuration for the specified cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |

### Return type

[**models::ClusterData**](ClusterData.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_clusters

> Vec<String> clusters_base_get_clusters()
Get the list of all the Pulsar clusters.

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


## clusters_base_get_domain

> models::FailureDomain clusters_base_get_domain(cluster, domain_name)
Get a domain in a cluster

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**domain_name** | **String** | The failure domain name | [required] |

### Return type

[**models::FailureDomain**](FailureDomain.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_failure_domains

> std::collections::HashMap<String, models::FailureDomain> clusters_base_get_failure_domains(cluster)
Get the cluster failure domains.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |

### Return type

[**std::collections::HashMap<String, models::FailureDomain>**](FailureDomain.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_namespace_isolation_policies

> std::collections::HashMap<String, models::NamespaceIsolationData> clusters_base_get_namespace_isolation_policies(cluster)
Get the namespace isolation policies assigned to the cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |

### Return type

[**std::collections::HashMap<String, models::NamespaceIsolationData>**](NamespaceIsolationData.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_namespace_isolation_policy

> models::NamespaceIsolationData clusters_base_get_namespace_isolation_policy(cluster, policy_name)
Get the single namespace isolation policy assigned to the cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**policy_name** | **String** | The name of the namespace isolation policy | [required] |

### Return type

[**models::NamespaceIsolationData**](NamespaceIsolationData.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_get_peer_cluster

> Vec<String> clusters_base_get_peer_cluster(cluster)
Get the peer-cluster data for the specified cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |

### Return type

**Vec<String>**

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_set_failure_domain

> clusters_base_set_failure_domain(cluster, domain_name, body)
Set the failure domain of the cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**domain_name** | **String** | The failure domain name | [required] |
**body** | [**FailureDomain**](FailureDomain.md) | The configuration data of a failure domain | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_set_namespace_isolation_policy

> clusters_base_set_namespace_isolation_policy(cluster, policy_name, body)
Set namespace isolation policy.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**policy_name** | **String** | The namespace isolation policy name | [required] |
**body** | [**NamespaceIsolationData**](NamespaceIsolationData.md) | The namespace isolation policy data | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_set_peer_cluster_names

> clusters_base_set_peer_cluster_names(cluster, body)
Update peer-cluster-list for a cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**body** | [**Vec<String>**](String.md) | The list of peer cluster names | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_update_cluster

> clusters_base_update_cluster(cluster, body)
Update the configuration for a cluster.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**body** | [**ClusterData**](ClusterData.md) | The cluster data | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusters_base_update_cluster_migration

> clusters_base_update_cluster_migration(cluster, migrated, body)
Update the configuration for a cluster migration.

This operation requires Pulsar superuser privileges.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster** | **String** | The cluster name | [required] |
**migrated** | **bool** | Is cluster migrated | [required] |
**body** | [**ClusterUrl**](ClusterUrl.md) | The cluster url data | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

