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
pub struct BookiesClusterInfo {
    #[serde(rename = "bookies", skip_serializing_if = "Option::is_none")]
    pub bookies: Option<Vec<models::RawBookieInfo>>,
}

impl BookiesClusterInfo {
    pub fn new() -> BookiesClusterInfo {
        BookiesClusterInfo {
            bookies: None,
        }
    }
}
