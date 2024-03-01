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
pub struct CompletableFuturePairBooleanSchemaCompatibilityStrategy {
    #[serde(rename = "cancelled", skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<bool>,
    #[serde(rename = "completedExceptionally", skip_serializing_if = "Option::is_none")]
    pub completed_exceptionally: Option<bool>,
    #[serde(rename = "done", skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
    #[serde(rename = "numberOfDependents", skip_serializing_if = "Option::is_none")]
    pub number_of_dependents: Option<i32>,
}

impl CompletableFuturePairBooleanSchemaCompatibilityStrategy {
    pub fn new() -> CompletableFuturePairBooleanSchemaCompatibilityStrategy {
        CompletableFuturePairBooleanSchemaCompatibilityStrategy {
            cancelled: None,
            completed_exceptionally: None,
            done: None,
            number_of_dependents: None,
        }
    }
}
