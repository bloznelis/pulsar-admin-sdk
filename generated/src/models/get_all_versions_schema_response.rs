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
pub struct GetAllVersionsSchemaResponse {
    #[serde(rename = "getSchemaResponses", skip_serializing_if = "Option::is_none")]
    pub get_schema_responses: Option<Vec<models::GetSchemaResponse>>,
}

impl GetAllVersionsSchemaResponse {
    pub fn new() -> GetAllVersionsSchemaResponse {
        GetAllVersionsSchemaResponse {
            get_schema_responses: None,
        }
    }
}

