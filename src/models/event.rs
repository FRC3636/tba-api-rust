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
pub struct Event {
    /// TBA event key with the format yyyy[EVENT_CODE], where yyyy is the year, and EVENT_CODE is the event code of the event.
    #[serde(rename = "key")]
    pub key: String,
    /// Official name of event on record either provided by FIRST or organizers of offseason event.
    #[serde(rename = "name")]
    pub name: String,
    /// Event short code, as provided by FIRST.
    #[serde(rename = "event_code")]
    pub event_code: String,
    /// Event Type, as defined here: https://github.com/the-blue-alliance/the-blue-alliance/blob/master/consts/event_type.py#L2
    #[serde(rename = "event_type")]
    pub event_type: i32,
    #[serde(rename = "district", deserialize_with = "Option::deserialize")]
    pub district: Option<Box<models::DistrictList>>,
    /// City, town, village, etc. the event is located in.
    #[serde(rename = "city", deserialize_with = "Option::deserialize")]
    pub city: Option<String>,
    /// State or Province the event is located in.
    #[serde(rename = "state_prov", deserialize_with = "Option::deserialize")]
    pub state_prov: Option<String>,
    /// Country the event is located in.
    #[serde(rename = "country", deserialize_with = "Option::deserialize")]
    pub country: Option<String>,
    /// Event start date in `yyyy-mm-dd` format.
    #[serde(rename = "start_date")]
    pub start_date: String,
    /// Event end date in `yyyy-mm-dd` format.
    #[serde(rename = "end_date")]
    pub end_date: String,
    /// Year the event data is for.
    #[serde(rename = "year")]
    pub year: i32,
    /// Same as `name` but doesn't include event specifiers, such as 'Regional' or 'District'. May be null.
    #[serde(rename = "short_name", deserialize_with = "Option::deserialize")]
    pub short_name: Option<String>,
    /// Event Type, eg Regional, District, or Offseason.
    #[serde(rename = "event_type_string")]
    pub event_type_string: String,
    /// Week of the event relative to the first official season event, zero-indexed. Only valid for Regionals, Districts, and District Championships. Null otherwise. (Eg. A season with a week 0 'preseason' event does not count, and week 1 events will show 0 here. Seasons with a week 0.5 regional event will show week 0 for those event(s) and week 1 for week 1 events and so on.)
    #[serde(rename = "week", deserialize_with = "Option::deserialize")]
    pub week: Option<i32>,
    /// Address of the event's venue, if available.
    #[serde(rename = "address", deserialize_with = "Option::deserialize")]
    pub address: Option<String>,
    /// Postal code from the event address.
    #[serde(rename = "postal_code", deserialize_with = "Option::deserialize")]
    pub postal_code: Option<String>,
    /// Google Maps Place ID for the event address.
    #[serde(rename = "gmaps_place_id", deserialize_with = "Option::deserialize")]
    pub gmaps_place_id: Option<String>,
    /// Link to address location on Google Maps.
    #[serde(rename = "gmaps_url", deserialize_with = "Option::deserialize")]
    pub gmaps_url: Option<String>,
    /// Latitude for the event address.
    #[serde(rename = "lat", deserialize_with = "Option::deserialize")]
    pub lat: Option<f64>,
    /// Longitude for the event address.
    #[serde(rename = "lng", deserialize_with = "Option::deserialize")]
    pub lng: Option<f64>,
    /// Name of the location at the address for the event, eg. Blue Alliance High School.
    #[serde(rename = "location_name", deserialize_with = "Option::deserialize")]
    pub location_name: Option<String>,
    /// Timezone name.
    #[serde(rename = "timezone")]
    pub timezone: String,
    /// The event's website, if any.
    #[serde(rename = "website", deserialize_with = "Option::deserialize")]
    pub website: Option<String>,
    /// The FIRST internal Event ID, used to link to the event on the FRC webpage.
    #[serde(rename = "first_event_id", deserialize_with = "Option::deserialize")]
    pub first_event_id: Option<String>,
    /// Public facing event code used by FIRST (on frc-events.firstinspires.org, for example)
    #[serde(rename = "first_event_code", deserialize_with = "Option::deserialize")]
    pub first_event_code: Option<String>,
    #[serde(rename = "webcasts")]
    pub webcasts: Vec<models::Webcast>,
    /// An array of event keys for the divisions at this event.
    #[serde(rename = "division_keys")]
    pub division_keys: Vec<String>,
    /// The TBA Event key that represents the event's parent. Used to link back to the event from a division event. It is also the inverse relation of `divison_keys`.
    #[serde(rename = "parent_event_key", deserialize_with = "Option::deserialize")]
    pub parent_event_key: Option<String>,
    /// Playoff Type, as defined here: https://github.com/the-blue-alliance/the-blue-alliance/blob/master/consts/playoff_type.py#L4, or null.
    #[serde(rename = "playoff_type", deserialize_with = "Option::deserialize")]
    pub playoff_type: Option<i32>,
    /// String representation of the `playoff_type`, or null.
    #[serde(rename = "playoff_type_string", deserialize_with = "Option::deserialize")]
    pub playoff_type_string: Option<String>,
}

impl Event {
    pub fn new(key: String, name: String, event_code: String, event_type: i32, district: Option<models::DistrictList>, city: Option<String>, state_prov: Option<String>, country: Option<String>, start_date: String, end_date: String, year: i32, short_name: Option<String>, event_type_string: String, week: Option<i32>, address: Option<String>, postal_code: Option<String>, gmaps_place_id: Option<String>, gmaps_url: Option<String>, lat: Option<f64>, lng: Option<f64>, location_name: Option<String>, timezone: String, website: Option<String>, first_event_id: Option<String>, first_event_code: Option<String>, webcasts: Vec<models::Webcast>, division_keys: Vec<String>, parent_event_key: Option<String>, playoff_type: Option<i32>, playoff_type_string: Option<String>) -> Event {
        Event {
            key,
            name,
            event_code,
            event_type,
            district: if let Some(x) = district {Some(Box::new(x))} else {None},
            city,
            state_prov,
            country,
            start_date,
            end_date,
            year,
            short_name,
            event_type_string,
            week,
            address,
            postal_code,
            gmaps_place_id,
            gmaps_url,
            lat,
            lng,
            location_name,
            timezone,
            website,
            first_event_id,
            first_event_code,
            webcasts,
            division_keys,
            parent_event_key,
            playoff_type,
            playoff_type_string,
        }
    }
}

