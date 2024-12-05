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
pub struct EventRankingRankingsInner {
    /// Number of matches played by this team.
    #[serde(rename = "matches_played")]
    pub matches_played: i32,
    /// The average match score during qualifications. Year specific. May be null if not relevant for a given year.
    #[serde(rename = "qual_average", deserialize_with = "Option::deserialize")]
    pub qual_average: Option<i32>,
    /// Additional special data on the team's performance calculated by TBA.
    #[serde(rename = "extra_stats")]
    pub extra_stats: Vec<f64>,
    /// Additional year-specific information, may be null. See parent `sort_order_info` for details.
    #[serde(rename = "sort_orders", deserialize_with = "Option::deserialize")]
    pub sort_orders: Option<Vec<f64>>,
    #[serde(rename = "record", deserialize_with = "Option::deserialize")]
    pub record: Option<Box<models::WltRecord>>,
    /// The team's rank at the event as provided by FIRST.
    #[serde(rename = "rank")]
    pub rank: i32,
    /// Number of times disqualified.
    #[serde(rename = "dq")]
    pub dq: i32,
    /// The team with this rank.
    #[serde(rename = "team_key")]
    pub team_key: String,
}

impl EventRankingRankingsInner {
    pub fn new(matches_played: i32, qual_average: Option<i32>, extra_stats: Vec<f64>, sort_orders: Option<Vec<f64>>, record: Option<models::WltRecord>, rank: i32, dq: i32, team_key: String) -> EventRankingRankingsInner {
        EventRankingRankingsInner {
            matches_played,
            qual_average,
            extra_stats,
            sort_orders,
            record: if let Some(x) = record {Some(Box::new(x))} else {None},
            rank,
            dq,
            team_key,
        }
    }
}

