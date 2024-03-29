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
pub struct PositionInfo {
    #[serde(rename = "entryId", skip_serializing_if = "Option::is_none")]
    pub entry_id: Option<i64>,
    #[serde(rename = "ledgerId", skip_serializing_if = "Option::is_none")]
    pub ledger_id: Option<i64>,
}

impl PositionInfo {
    pub fn new() -> PositionInfo {
        PositionInfo {
            entry_id: None,
            ledger_id: None,
        }
    }
}

