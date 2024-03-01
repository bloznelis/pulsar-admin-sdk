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
pub struct FunctionInstanceStatsDataBase {
    #[serde(rename = "avgProcessLatency", skip_serializing_if = "Option::is_none")]
    pub avg_process_latency: Option<f64>,
    #[serde(rename = "processedSuccessfullyTotal", skip_serializing_if = "Option::is_none")]
    pub processed_successfully_total: Option<i64>,
    #[serde(rename = "receivedTotal", skip_serializing_if = "Option::is_none")]
    pub received_total: Option<i64>,
    #[serde(rename = "systemExceptionsTotal", skip_serializing_if = "Option::is_none")]
    pub system_exceptions_total: Option<i64>,
    #[serde(rename = "userExceptionsTotal", skip_serializing_if = "Option::is_none")]
    pub user_exceptions_total: Option<i64>,
}

impl FunctionInstanceStatsDataBase {
    pub fn new() -> FunctionInstanceStatsDataBase {
        FunctionInstanceStatsDataBase {
            avg_process_latency: None,
            processed_successfully_total: None,
            received_total: None,
            system_exceptions_total: None,
            user_exceptions_total: None,
        }
    }
}
