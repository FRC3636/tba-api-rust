/*
 * The Blue Alliance API v3
 *
 * # Overview    Information and statistics about FIRST Robotics Competition teams and events.   # Authentication   All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).
 *
 * The version of the OpenAPI document: 3.9.5
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// MatchScoreBreakdown2024 : See the 2024 FMS API documentation for a description of each value. https://frc-api-docs.firstinspires.org
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MatchScoreBreakdown2024 {
    #[serde(rename = "blue")]
    pub blue: Box<models::MatchScoreBreakdown2024Alliance>,
    #[serde(rename = "red")]
    pub red: Box<models::MatchScoreBreakdown2024Alliance>,
}

impl MatchScoreBreakdown2024 {
    /// See the 2024 FMS API documentation for a description of each value. https://frc-api-docs.firstinspires.org
    pub fn new(blue: models::MatchScoreBreakdown2024Alliance, red: models::MatchScoreBreakdown2024Alliance) -> MatchScoreBreakdown2024 {
        MatchScoreBreakdown2024 {
            blue: Box::new(blue),
            red: Box::new(red),
        }
    }
}

