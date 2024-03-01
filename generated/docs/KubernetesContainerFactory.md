# KubernetesContainerFactory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change_config_map** | Option<**String**> |  | [optional]
**change_config_map_namespace** | Option<**String**> |  | [optional]
**config_admin_cli** | Option<**String**> |  | [optional]
**cpu_over_commit_ratio** | Option<**f64**> |  | [optional]
**custom_labels** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**expected_metrics_collection_interval** | Option<**i32**> |  | [optional]
**extra_function_dependencies_dir** | Option<**String**> |  | [optional]
**function_docker_images** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**function_instance_class_path** | Option<**String**> |  | [optional]
**grace_period_seconds** | Option<**i32**> |  | [optional]
**grpc_port** | Option<**i32**> |  | [optional]
**image_pull_policy** | Option<**String**> |  | [optional]
**install_user_code_dependencies** | Option<**bool**> |  | [optional]
**job_name** | Option<**String**> |  | [optional]
**job_namespace** | Option<**String**> |  | [optional]
**k8_uri** | Option<**String**> |  | [optional]
**kubernetes_function_auth_provider_config** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**memory_over_commit_ratio** | Option<**f64**> |  | [optional]
**metrics_port** | Option<**i32**> |  | [optional]
**nar_extraction_directory** | Option<**String**> |  | [optional]
**percent_memory_padding** | Option<**i32**> |  | [optional]
**pulsar_admin_url** | Option<**String**> |  | [optional]
**pulsar_docker_image_name** | Option<**String**> |  | [optional]
**pulsar_root_dir** | Option<**String**> |  | [optional]
**pulsar_service_url** | Option<**String**> |  | [optional]
**python_dependency_repository** | Option<**String**> |  | [optional]
**python_extra_dependency_repository** | Option<**String**> |  | [optional]
**submitting_inside_pod** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


