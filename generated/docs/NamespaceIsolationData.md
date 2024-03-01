# NamespaceIsolationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_failover_policy** | Option<[**models::AutoFailoverPolicyData**](AutoFailoverPolicyData.md)> |  | [optional]
**namespaces** | Option<**Vec<String>**> | The list of namespaces to apply this namespace isolation data | [optional]
**primary** | Option<**Vec<String>**> | The list of primary brokers for serving the list of namespaces in this isolation policy | [optional]
**secondary** | Option<**Vec<String>**> | The list of secondary brokers for serving the list of namespaces in this isolation policy | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


