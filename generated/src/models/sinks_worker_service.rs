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
pub struct SinksWorkerService {
    #[serde(rename = "listOfConnectors", skip_serializing_if = "Option::is_none")]
    pub list_of_connectors: Option<Vec<models::ConnectorDefinition>>,
    #[serde(rename = "sinkList", skip_serializing_if = "Option::is_none")]
    pub sink_list: Option<Vec<models::ConnectorDefinition>>,
}

impl SinksWorkerService {
    pub fn new() -> SinksWorkerService {
        SinksWorkerService {
            list_of_connectors: None,
            sink_list: None,
        }
    }
}

