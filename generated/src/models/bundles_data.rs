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
pub struct BundlesData {
    #[serde(rename = "boundaries", skip_serializing_if = "Option::is_none")]
    pub boundaries: Option<Vec<String>>,
    #[serde(rename = "numBundles", skip_serializing_if = "Option::is_none")]
    pub num_bundles: Option<i32>,
}

impl BundlesData {
    pub fn new() -> BundlesData {
        BundlesData {
            boundaries: None,
            num_bundles: None,
        }
    }
}

