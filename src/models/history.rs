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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct History {
    #[serde(rename = "events")]
    pub events: Vec<models::Event>,
    #[serde(rename = "awards")]
    pub awards: Vec<models::Award>,
}

impl History {
    pub fn new(events: Vec<models::Event>, awards: Vec<models::Award>) -> History {
        History {
            events,
            awards,
        }
    }
}

