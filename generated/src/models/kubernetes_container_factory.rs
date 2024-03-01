/*
 * Pulsar Admin REST API
 *
 * This provides the REST API for admin operations
 *
 * The version of the OpenAPI document: v2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KubernetesContainerFactory {
    #[serde(rename = "changeConfigMap", skip_serializing_if = "Option::is_none")]
    pub change_config_map: Option<String>,
    #[serde(rename = "changeConfigMapNamespace", skip_serializing_if = "Option::is_none")]
    pub change_config_map_namespace: Option<String>,
    #[serde(rename = "configAdminCLI", skip_serializing_if = "Option::is_none")]
    pub config_admin_cli: Option<String>,
    #[serde(rename = "cpuOverCommitRatio", skip_serializing_if = "Option::is_none")]
    pub cpu_over_commit_ratio: Option<f64>,
    #[serde(rename = "customLabels", skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "expectedMetricsCollectionInterval", skip_serializing_if = "Option::is_none")]
    pub expected_metrics_collection_interval: Option<i32>,
    #[serde(rename = "extraFunctionDependenciesDir", skip_serializing_if = "Option::is_none")]
    pub extra_function_dependencies_dir: Option<String>,
    #[serde(rename = "functionDockerImages", skip_serializing_if = "Option::is_none")]
    pub function_docker_images: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "functionInstanceClassPath", skip_serializing_if = "Option::is_none")]
    pub function_instance_class_path: Option<String>,
    #[serde(rename = "gracePeriodSeconds", skip_serializing_if = "Option::is_none")]
    pub grace_period_seconds: Option<i32>,
    #[serde(rename = "grpcPort", skip_serializing_if = "Option::is_none")]
    pub grpc_port: Option<i32>,
    #[serde(rename = "imagePullPolicy", skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    #[serde(rename = "installUserCodeDependencies", skip_serializing_if = "Option::is_none")]
    pub install_user_code_dependencies: Option<bool>,
    #[serde(rename = "jobName", skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobNamespace", skip_serializing_if = "Option::is_none")]
    pub job_namespace: Option<String>,
    #[serde(rename = "k8Uri", skip_serializing_if = "Option::is_none")]
    pub k8_uri: Option<String>,
    #[serde(rename = "kubernetesFunctionAuthProviderConfig", skip_serializing_if = "Option::is_none")]
    pub kubernetes_function_auth_provider_config: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "memoryOverCommitRatio", skip_serializing_if = "Option::is_none")]
    pub memory_over_commit_ratio: Option<f64>,
    #[serde(rename = "metricsPort", skip_serializing_if = "Option::is_none")]
    pub metrics_port: Option<i32>,
    #[serde(rename = "narExtractionDirectory", skip_serializing_if = "Option::is_none")]
    pub nar_extraction_directory: Option<String>,
    #[serde(rename = "percentMemoryPadding", skip_serializing_if = "Option::is_none")]
    pub percent_memory_padding: Option<i32>,
    #[serde(rename = "pulsarAdminUrl", skip_serializing_if = "Option::is_none")]
    pub pulsar_admin_url: Option<String>,
    #[serde(rename = "pulsarDockerImageName", skip_serializing_if = "Option::is_none")]
    pub pulsar_docker_image_name: Option<String>,
    #[serde(rename = "pulsarRootDir", skip_serializing_if = "Option::is_none")]
    pub pulsar_root_dir: Option<String>,
    #[serde(rename = "pulsarServiceUrl", skip_serializing_if = "Option::is_none")]
    pub pulsar_service_url: Option<String>,
    #[serde(rename = "pythonDependencyRepository", skip_serializing_if = "Option::is_none")]
    pub python_dependency_repository: Option<String>,
    #[serde(rename = "pythonExtraDependencyRepository", skip_serializing_if = "Option::is_none")]
    pub python_extra_dependency_repository: Option<String>,
    #[serde(rename = "submittingInsidePod", skip_serializing_if = "Option::is_none")]
    pub submitting_inside_pod: Option<bool>,
}

impl KubernetesContainerFactory {
    pub fn new() -> KubernetesContainerFactory {
        KubernetesContainerFactory {
            change_config_map: None,
            change_config_map_namespace: None,
            config_admin_cli: None,
            cpu_over_commit_ratio: None,
            custom_labels: None,
            expected_metrics_collection_interval: None,
            extra_function_dependencies_dir: None,
            function_docker_images: None,
            function_instance_class_path: None,
            grace_period_seconds: None,
            grpc_port: None,
            image_pull_policy: None,
            install_user_code_dependencies: None,
            job_name: None,
            job_namespace: None,
            k8_uri: None,
            kubernetes_function_auth_provider_config: None,
            memory_over_commit_ratio: None,
            metrics_port: None,
            nar_extraction_directory: None,
            percent_memory_padding: None,
            pulsar_admin_url: None,
            pulsar_docker_image_name: None,
            pulsar_root_dir: None,
            pulsar_service_url: None,
            python_dependency_repository: None,
            python_extra_dependency_repository: None,
            submitting_inside_pod: None,
        }
    }
}

