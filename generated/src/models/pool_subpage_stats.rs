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
pub struct PoolSubpageStats {
    #[serde(rename = "elementSize", skip_serializing_if = "Option::is_none")]
    pub element_size: Option<i32>,
    #[serde(rename = "maxNumElements", skip_serializing_if = "Option::is_none")]
    pub max_num_elements: Option<i32>,
    #[serde(rename = "numAvailable", skip_serializing_if = "Option::is_none")]
    pub num_available: Option<i32>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

impl PoolSubpageStats {
    pub fn new() -> PoolSubpageStats {
        PoolSubpageStats {
            element_size: None,
            max_num_elements: None,
            num_available: None,
            page_size: None,
        }
    }
}

