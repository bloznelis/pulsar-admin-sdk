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
pub struct ProcessContainerFactory {
    #[serde(rename = "extraFunctionDependenciesDir", skip_serializing_if = "Option::is_none")]
    pub extra_function_dependencies_dir: Option<String>,
    #[serde(rename = "javaInstanceJarLocation", skip_serializing_if = "Option::is_none")]
    pub java_instance_jar_location: Option<String>,
    #[serde(rename = "logDirectory", skip_serializing_if = "Option::is_none")]
    pub log_directory: Option<String>,
    #[serde(rename = "pythonInstanceLocation", skip_serializing_if = "Option::is_none")]
    pub python_instance_location: Option<String>,
}

impl ProcessContainerFactory {
    pub fn new() -> ProcessContainerFactory {
        ProcessContainerFactory {
            extra_function_dependencies_dir: None,
            java_instance_jar_location: None,
            log_directory: None,
            python_instance_location: None,
        }
    }
}

