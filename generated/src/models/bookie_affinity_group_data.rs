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
pub struct BookieAffinityGroupData {
    #[serde(rename = "bookkeeperAffinityGroupPrimary", skip_serializing_if = "Option::is_none")]
    pub bookkeeper_affinity_group_primary: Option<String>,
    #[serde(rename = "bookkeeperAffinityGroupSecondary", skip_serializing_if = "Option::is_none")]
    pub bookkeeper_affinity_group_secondary: Option<String>,
}

impl BookieAffinityGroupData {
    pub fn new() -> BookieAffinityGroupData {
        BookieAffinityGroupData {
            bookkeeper_affinity_group_primary: None,
            bookkeeper_affinity_group_secondary: None,
        }
    }
}

