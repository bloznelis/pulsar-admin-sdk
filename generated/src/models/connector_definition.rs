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
pub struct ConnectorDefinition {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sinkClass", skip_serializing_if = "Option::is_none")]
    pub sink_class: Option<String>,
    #[serde(rename = "sinkConfigClass", skip_serializing_if = "Option::is_none")]
    pub sink_config_class: Option<String>,
    #[serde(rename = "sourceClass", skip_serializing_if = "Option::is_none")]
    pub source_class: Option<String>,
    #[serde(rename = "sourceConfigClass", skip_serializing_if = "Option::is_none")]
    pub source_config_class: Option<String>,
}

impl ConnectorDefinition {
    pub fn new() -> ConnectorDefinition {
        ConnectorDefinition {
            description: None,
            name: None,
            sink_class: None,
            sink_config_class: None,
            source_class: None,
            source_config_class: None,
        }
    }
}

