# Rust API client for openapi

# Overview 

 Information and statistics about FIRST Robotics Competition teams and events. 

# Authentication 

All endpoints require an Auth Key to be passed in the header `X-TBA-Auth-Key`. If you do not have an auth key yet, you can obtain one from your [Account Page](/account).


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 3.9.5
- Package version: 3.9.5
- Generator version: 7.10.0-SNAPSHOT
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DistrictApi* | [**get_district_events**](docs/DistrictApi.md#get_district_events) | **GET** /district/{district_key}/events | 
*DistrictApi* | [**get_district_events_keys**](docs/DistrictApi.md#get_district_events_keys) | **GET** /district/{district_key}/events/keys | 
*DistrictApi* | [**get_district_events_simple**](docs/DistrictApi.md#get_district_events_simple) | **GET** /district/{district_key}/events/simple | 
*DistrictApi* | [**get_district_rankings**](docs/DistrictApi.md#get_district_rankings) | **GET** /district/{district_key}/rankings | 
*DistrictApi* | [**get_district_teams**](docs/DistrictApi.md#get_district_teams) | **GET** /district/{district_key}/teams | 
*DistrictApi* | [**get_district_teams_keys**](docs/DistrictApi.md#get_district_teams_keys) | **GET** /district/{district_key}/teams/keys | 
*DistrictApi* | [**get_district_teams_simple**](docs/DistrictApi.md#get_district_teams_simple) | **GET** /district/{district_key}/teams/simple | 
*DistrictApi* | [**get_districts_by_year**](docs/DistrictApi.md#get_districts_by_year) | **GET** /districts/{year} | 
*DistrictApi* | [**get_event_district_points_0**](docs/DistrictApi.md#get_event_district_points_0) | **GET** /event/{event_key}/district_points | 
*DistrictApi* | [**get_team_districts_0**](docs/DistrictApi.md#get_team_districts_0) | **GET** /team/{team_key}/districts | 
*EventApi* | [**get_district_events_0**](docs/EventApi.md#get_district_events_0) | **GET** /district/{district_key}/events | 
*EventApi* | [**get_district_events_keys_0**](docs/EventApi.md#get_district_events_keys_0) | **GET** /district/{district_key}/events/keys | 
*EventApi* | [**get_district_events_simple_0**](docs/EventApi.md#get_district_events_simple_0) | **GET** /district/{district_key}/events/simple | 
*EventApi* | [**get_event**](docs/EventApi.md#get_event) | **GET** /event/{event_key} | 
*EventApi* | [**get_event_alliances**](docs/EventApi.md#get_event_alliances) | **GET** /event/{event_key}/alliances | 
*EventApi* | [**get_event_awards**](docs/EventApi.md#get_event_awards) | **GET** /event/{event_key}/awards | 
*EventApi* | [**get_event_coprs**](docs/EventApi.md#get_event_coprs) | **GET** /event/{event_key}/coprs | 
*EventApi* | [**get_event_district_points**](docs/EventApi.md#get_event_district_points) | **GET** /event/{event_key}/district_points | 
*EventApi* | [**get_event_insights**](docs/EventApi.md#get_event_insights) | **GET** /event/{event_key}/insights | 
*EventApi* | [**get_event_match_timeseries**](docs/EventApi.md#get_event_match_timeseries) | **GET** /event/{event_key}/matches/timeseries | 
*EventApi* | [**get_event_matches**](docs/EventApi.md#get_event_matches) | **GET** /event/{event_key}/matches | 
*EventApi* | [**get_event_matches_keys**](docs/EventApi.md#get_event_matches_keys) | **GET** /event/{event_key}/matches/keys | 
*EventApi* | [**get_event_matches_simple**](docs/EventApi.md#get_event_matches_simple) | **GET** /event/{event_key}/matches/simple | 
*EventApi* | [**get_event_oprs**](docs/EventApi.md#get_event_oprs) | **GET** /event/{event_key}/oprs | 
*EventApi* | [**get_event_predictions**](docs/EventApi.md#get_event_predictions) | **GET** /event/{event_key}/predictions | 
*EventApi* | [**get_event_rankings**](docs/EventApi.md#get_event_rankings) | **GET** /event/{event_key}/rankings | 
*EventApi* | [**get_event_simple**](docs/EventApi.md#get_event_simple) | **GET** /event/{event_key}/simple | 
*EventApi* | [**get_event_team_media**](docs/EventApi.md#get_event_team_media) | **GET** /event/{event_key}/team_media | 
*EventApi* | [**get_event_teams**](docs/EventApi.md#get_event_teams) | **GET** /event/{event_key}/teams | 
*EventApi* | [**get_event_teams_keys**](docs/EventApi.md#get_event_teams_keys) | **GET** /event/{event_key}/teams/keys | 
*EventApi* | [**get_event_teams_simple**](docs/EventApi.md#get_event_teams_simple) | **GET** /event/{event_key}/teams/simple | 
*EventApi* | [**get_event_teams_statuses**](docs/EventApi.md#get_event_teams_statuses) | **GET** /event/{event_key}/teams/statuses | 
*EventApi* | [**get_events_by_year**](docs/EventApi.md#get_events_by_year) | **GET** /events/{year} | 
*EventApi* | [**get_events_by_year_keys**](docs/EventApi.md#get_events_by_year_keys) | **GET** /events/{year}/keys | 
*EventApi* | [**get_events_by_year_simple**](docs/EventApi.md#get_events_by_year_simple) | **GET** /events/{year}/simple | 
*EventApi* | [**get_team_event_awards_0**](docs/EventApi.md#get_team_event_awards_0) | **GET** /team/{team_key}/event/{event_key}/awards | 
*EventApi* | [**get_team_event_matches_0**](docs/EventApi.md#get_team_event_matches_0) | **GET** /team/{team_key}/event/{event_key}/matches | 
*EventApi* | [**get_team_event_matches_keys_0**](docs/EventApi.md#get_team_event_matches_keys_0) | **GET** /team/{team_key}/event/{event_key}/matches/keys | 
*EventApi* | [**get_team_event_matches_simple_0**](docs/EventApi.md#get_team_event_matches_simple_0) | **GET** /team/{team_key}/event/{event_key}/matches/simple | 
*EventApi* | [**get_team_event_status_0**](docs/EventApi.md#get_team_event_status_0) | **GET** /team/{team_key}/event/{event_key}/status | 
*EventApi* | [**get_team_events_0**](docs/EventApi.md#get_team_events_0) | **GET** /team/{team_key}/events | 
*EventApi* | [**get_team_events_by_year_0**](docs/EventApi.md#get_team_events_by_year_0) | **GET** /team/{team_key}/events/{year} | 
*EventApi* | [**get_team_events_by_year_keys_0**](docs/EventApi.md#get_team_events_by_year_keys_0) | **GET** /team/{team_key}/events/{year}/keys | 
*EventApi* | [**get_team_events_by_year_simple_0**](docs/EventApi.md#get_team_events_by_year_simple_0) | **GET** /team/{team_key}/events/{year}/simple | 
*EventApi* | [**get_team_events_keys_0**](docs/EventApi.md#get_team_events_keys_0) | **GET** /team/{team_key}/events/keys | 
*EventApi* | [**get_team_events_simple_0**](docs/EventApi.md#get_team_events_simple_0) | **GET** /team/{team_key}/events/simple | 
*EventApi* | [**get_team_events_statuses_by_year_1**](docs/EventApi.md#get_team_events_statuses_by_year_1) | **GET** /team/{team_key}/events/{year}/statuses | 
*InsightApi* | [**get_insights_leaderboards_year**](docs/InsightApi.md#get_insights_leaderboards_year) | **GET** /insights/leaderboards/{year} | 
*ListApi* | [**get_district_events_1**](docs/ListApi.md#get_district_events_1) | **GET** /district/{district_key}/events | 
*ListApi* | [**get_district_events_keys_1**](docs/ListApi.md#get_district_events_keys_1) | **GET** /district/{district_key}/events/keys | 
*ListApi* | [**get_district_events_simple_1**](docs/ListApi.md#get_district_events_simple_1) | **GET** /district/{district_key}/events/simple | 
*ListApi* | [**get_district_rankings_1**](docs/ListApi.md#get_district_rankings_1) | **GET** /district/{district_key}/rankings | 
*ListApi* | [**get_district_teams_1**](docs/ListApi.md#get_district_teams_1) | **GET** /district/{district_key}/teams | 
*ListApi* | [**get_district_teams_keys_1**](docs/ListApi.md#get_district_teams_keys_1) | **GET** /district/{district_key}/teams/keys | 
*ListApi* | [**get_district_teams_simple_1**](docs/ListApi.md#get_district_teams_simple_1) | **GET** /district/{district_key}/teams/simple | 
*ListApi* | [**get_event_teams_1**](docs/ListApi.md#get_event_teams_1) | **GET** /event/{event_key}/teams | 
*ListApi* | [**get_event_teams_keys_1**](docs/ListApi.md#get_event_teams_keys_1) | **GET** /event/{event_key}/teams/keys | 
*ListApi* | [**get_event_teams_simple_1**](docs/ListApi.md#get_event_teams_simple_1) | **GET** /event/{event_key}/teams/simple | 
*ListApi* | [**get_event_teams_statuses_1**](docs/ListApi.md#get_event_teams_statuses_1) | **GET** /event/{event_key}/teams/statuses | 
*ListApi* | [**get_events_by_year_0**](docs/ListApi.md#get_events_by_year_0) | **GET** /events/{year} | 
*ListApi* | [**get_events_by_year_keys_0**](docs/ListApi.md#get_events_by_year_keys_0) | **GET** /events/{year}/keys | 
*ListApi* | [**get_events_by_year_simple_0**](docs/ListApi.md#get_events_by_year_simple_0) | **GET** /events/{year}/simple | 
*ListApi* | [**get_insights_leaderboards_year_0**](docs/ListApi.md#get_insights_leaderboards_year_0) | **GET** /insights/leaderboards/{year} | 
*ListApi* | [**get_team_events_statuses_by_year**](docs/ListApi.md#get_team_events_statuses_by_year) | **GET** /team/{team_key}/events/{year}/statuses | 
*ListApi* | [**get_teams_0**](docs/ListApi.md#get_teams_0) | **GET** /teams/{page_num} | 
*ListApi* | [**get_teams_by_year_0**](docs/ListApi.md#get_teams_by_year_0) | **GET** /teams/{year}/{page_num} | 
*ListApi* | [**get_teams_by_year_keys_0**](docs/ListApi.md#get_teams_by_year_keys_0) | **GET** /teams/{year}/{page_num}/keys | 
*ListApi* | [**get_teams_by_year_simple_0**](docs/ListApi.md#get_teams_by_year_simple_0) | **GET** /teams/{year}/{page_num}/simple | 
*ListApi* | [**get_teams_keys_0**](docs/ListApi.md#get_teams_keys_0) | **GET** /teams/{page_num}/keys | 
*ListApi* | [**get_teams_simple_0**](docs/ListApi.md#get_teams_simple_0) | **GET** /teams/{page_num}/simple | 
*MatchApi* | [**get_event_match_timeseries_0**](docs/MatchApi.md#get_event_match_timeseries_0) | **GET** /event/{event_key}/matches/timeseries | 
*MatchApi* | [**get_event_matches_0**](docs/MatchApi.md#get_event_matches_0) | **GET** /event/{event_key}/matches | 
*MatchApi* | [**get_event_matches_keys_0**](docs/MatchApi.md#get_event_matches_keys_0) | **GET** /event/{event_key}/matches/keys | 
*MatchApi* | [**get_event_matches_simple_0**](docs/MatchApi.md#get_event_matches_simple_0) | **GET** /event/{event_key}/matches/simple | 
*MatchApi* | [**get_match**](docs/MatchApi.md#get_match) | **GET** /match/{match_key} | 
*MatchApi* | [**get_match_simple**](docs/MatchApi.md#get_match_simple) | **GET** /match/{match_key}/simple | 
*MatchApi* | [**get_match_timeseries**](docs/MatchApi.md#get_match_timeseries) | **GET** /match/{match_key}/timeseries | 
*MatchApi* | [**get_match_zebra**](docs/MatchApi.md#get_match_zebra) | **GET** /match/{match_key}/zebra_motionworks | 
*MatchApi* | [**get_team_event_matches_1**](docs/MatchApi.md#get_team_event_matches_1) | **GET** /team/{team_key}/event/{event_key}/matches | 
*MatchApi* | [**get_team_event_matches_keys_1**](docs/MatchApi.md#get_team_event_matches_keys_1) | **GET** /team/{team_key}/event/{event_key}/matches/keys | 
*MatchApi* | [**get_team_event_matches_simple_1**](docs/MatchApi.md#get_team_event_matches_simple_1) | **GET** /team/{team_key}/event/{event_key}/matches/simple | 
*MatchApi* | [**get_team_matches_by_year_0**](docs/MatchApi.md#get_team_matches_by_year_0) | **GET** /team/{team_key}/matches/{year} | 
*MatchApi* | [**get_team_matches_by_year_keys_0**](docs/MatchApi.md#get_team_matches_by_year_keys_0) | **GET** /team/{team_key}/matches/{year}/keys | 
*MatchApi* | [**get_team_matches_by_year_simple_0**](docs/MatchApi.md#get_team_matches_by_year_simple_0) | **GET** /team/{team_key}/matches/{year}/simple | 
*TbaApi* | [**get_status**](docs/TbaApi.md#get_status) | **GET** /status | 
*TeamApi* | [**get_district_rankings_0**](docs/TeamApi.md#get_district_rankings_0) | **GET** /district/{district_key}/rankings | 
*TeamApi* | [**get_district_teams_0**](docs/TeamApi.md#get_district_teams_0) | **GET** /district/{district_key}/teams | 
*TeamApi* | [**get_district_teams_keys_0**](docs/TeamApi.md#get_district_teams_keys_0) | **GET** /district/{district_key}/teams/keys | 
*TeamApi* | [**get_district_teams_simple_0**](docs/TeamApi.md#get_district_teams_simple_0) | **GET** /district/{district_key}/teams/simple | 
*TeamApi* | [**get_event_teams_0**](docs/TeamApi.md#get_event_teams_0) | **GET** /event/{event_key}/teams | 
*TeamApi* | [**get_event_teams_keys_0**](docs/TeamApi.md#get_event_teams_keys_0) | **GET** /event/{event_key}/teams/keys | 
*TeamApi* | [**get_event_teams_simple_0**](docs/TeamApi.md#get_event_teams_simple_0) | **GET** /event/{event_key}/teams/simple | 
*TeamApi* | [**get_event_teams_statuses_0**](docs/TeamApi.md#get_event_teams_statuses_0) | **GET** /event/{event_key}/teams/statuses | 
*TeamApi* | [**get_team**](docs/TeamApi.md#get_team) | **GET** /team/{team_key} | 
*TeamApi* | [**get_team_awards**](docs/TeamApi.md#get_team_awards) | **GET** /team/{team_key}/awards | 
*TeamApi* | [**get_team_awards_by_year**](docs/TeamApi.md#get_team_awards_by_year) | **GET** /team/{team_key}/awards/{year} | 
*TeamApi* | [**get_team_districts**](docs/TeamApi.md#get_team_districts) | **GET** /team/{team_key}/districts | 
*TeamApi* | [**get_team_event_awards**](docs/TeamApi.md#get_team_event_awards) | **GET** /team/{team_key}/event/{event_key}/awards | 
*TeamApi* | [**get_team_event_matches**](docs/TeamApi.md#get_team_event_matches) | **GET** /team/{team_key}/event/{event_key}/matches | 
*TeamApi* | [**get_team_event_matches_keys**](docs/TeamApi.md#get_team_event_matches_keys) | **GET** /team/{team_key}/event/{event_key}/matches/keys | 
*TeamApi* | [**get_team_event_matches_simple**](docs/TeamApi.md#get_team_event_matches_simple) | **GET** /team/{team_key}/event/{event_key}/matches/simple | 
*TeamApi* | [**get_team_event_status**](docs/TeamApi.md#get_team_event_status) | **GET** /team/{team_key}/event/{event_key}/status | 
*TeamApi* | [**get_team_events**](docs/TeamApi.md#get_team_events) | **GET** /team/{team_key}/events | 
*TeamApi* | [**get_team_events_by_year**](docs/TeamApi.md#get_team_events_by_year) | **GET** /team/{team_key}/events/{year} | 
*TeamApi* | [**get_team_events_by_year_keys**](docs/TeamApi.md#get_team_events_by_year_keys) | **GET** /team/{team_key}/events/{year}/keys | 
*TeamApi* | [**get_team_events_by_year_simple**](docs/TeamApi.md#get_team_events_by_year_simple) | **GET** /team/{team_key}/events/{year}/simple | 
*TeamApi* | [**get_team_events_keys**](docs/TeamApi.md#get_team_events_keys) | **GET** /team/{team_key}/events/keys | 
*TeamApi* | [**get_team_events_simple**](docs/TeamApi.md#get_team_events_simple) | **GET** /team/{team_key}/events/simple | 
*TeamApi* | [**get_team_events_statuses_by_year_0**](docs/TeamApi.md#get_team_events_statuses_by_year_0) | **GET** /team/{team_key}/events/{year}/statuses | 
*TeamApi* | [**get_team_history**](docs/TeamApi.md#get_team_history) | **GET** /team/{team_key}/history | 
*TeamApi* | [**get_team_matches_by_year**](docs/TeamApi.md#get_team_matches_by_year) | **GET** /team/{team_key}/matches/{year} | 
*TeamApi* | [**get_team_matches_by_year_keys**](docs/TeamApi.md#get_team_matches_by_year_keys) | **GET** /team/{team_key}/matches/{year}/keys | 
*TeamApi* | [**get_team_matches_by_year_simple**](docs/TeamApi.md#get_team_matches_by_year_simple) | **GET** /team/{team_key}/matches/{year}/simple | 
*TeamApi* | [**get_team_media_by_tag**](docs/TeamApi.md#get_team_media_by_tag) | **GET** /team/{team_key}/media/tag/{media_tag} | 
*TeamApi* | [**get_team_media_by_tag_year**](docs/TeamApi.md#get_team_media_by_tag_year) | **GET** /team/{team_key}/media/tag/{media_tag}/{year} | 
*TeamApi* | [**get_team_media_by_year**](docs/TeamApi.md#get_team_media_by_year) | **GET** /team/{team_key}/media/{year} | 
*TeamApi* | [**get_team_robots**](docs/TeamApi.md#get_team_robots) | **GET** /team/{team_key}/robots | 
*TeamApi* | [**get_team_simple**](docs/TeamApi.md#get_team_simple) | **GET** /team/{team_key}/simple | 
*TeamApi* | [**get_team_social_media**](docs/TeamApi.md#get_team_social_media) | **GET** /team/{team_key}/social_media | 
*TeamApi* | [**get_team_years_participated**](docs/TeamApi.md#get_team_years_participated) | **GET** /team/{team_key}/years_participated | 
*TeamApi* | [**get_teams**](docs/TeamApi.md#get_teams) | **GET** /teams/{page_num} | 
*TeamApi* | [**get_teams_by_year**](docs/TeamApi.md#get_teams_by_year) | **GET** /teams/{year}/{page_num} | 
*TeamApi* | [**get_teams_by_year_keys**](docs/TeamApi.md#get_teams_by_year_keys) | **GET** /teams/{year}/{page_num}/keys | 
*TeamApi* | [**get_teams_by_year_simple**](docs/TeamApi.md#get_teams_by_year_simple) | **GET** /teams/{year}/{page_num}/simple | 
*TeamApi* | [**get_teams_keys**](docs/TeamApi.md#get_teams_keys) | **GET** /teams/{page_num}/keys | 
*TeamApi* | [**get_teams_simple**](docs/TeamApi.md#get_teams_simple) | **GET** /teams/{page_num}/simple | 


## Documentation For Models

 - [ApiStatus](docs/ApiStatus.md)
 - [ApiStatusAppVersion](docs/ApiStatusAppVersion.md)
 - [Award](docs/Award.md)
 - [AwardRecipient](docs/AwardRecipient.md)
 - [DistrictList](docs/DistrictList.md)
 - [DistrictRanking](docs/DistrictRanking.md)
 - [DistrictRankingEventPointsInner](docs/DistrictRankingEventPointsInner.md)
 - [EliminationAlliance](docs/EliminationAlliance.md)
 - [EliminationAllianceBackup](docs/EliminationAllianceBackup.md)
 - [EliminationAllianceStatus](docs/EliminationAllianceStatus.md)
 - [Event](docs/Event.md)
 - [EventDistrictPoints](docs/EventDistrictPoints.md)
 - [EventDistrictPointsPointsValue](docs/EventDistrictPointsPointsValue.md)
 - [EventDistrictPointsTiebreakersValue](docs/EventDistrictPointsTiebreakersValue.md)
 - [EventInsights](docs/EventInsights.md)
 - [EventInsights2016](docs/EventInsights2016.md)
 - [EventInsights2017](docs/EventInsights2017.md)
 - [EventInsights2018](docs/EventInsights2018.md)
 - [EventOprs](docs/EventOprs.md)
 - [EventRanking](docs/EventRanking.md)
 - [EventRankingExtraStatsInfoInner](docs/EventRankingExtraStatsInfoInner.md)
 - [EventRankingRankingsInner](docs/EventRankingRankingsInner.md)
 - [EventRankingSortOrderInfoInner](docs/EventRankingSortOrderInfoInner.md)
 - [EventSimple](docs/EventSimple.md)
 - [GetStatus401Response](docs/GetStatus401Response.md)
 - [GetTeamEventsStatusesByYear200ResponseValue](docs/GetTeamEventsStatusesByYear200ResponseValue.md)
 - [History](docs/History.md)
 - [LeaderboardInsight](docs/LeaderboardInsight.md)
 - [LeaderboardInsightData](docs/LeaderboardInsightData.md)
 - [LeaderboardInsightDataRankingsInner](docs/LeaderboardInsightDataRankingsInner.md)
 - [Match](docs/Match.md)
 - [MatchAlliance](docs/MatchAlliance.md)
 - [MatchScoreBreakdown](docs/MatchScoreBreakdown.md)
 - [MatchScoreBreakdown2015](docs/MatchScoreBreakdown2015.md)
 - [MatchScoreBreakdown2015Alliance](docs/MatchScoreBreakdown2015Alliance.md)
 - [MatchScoreBreakdown2016](docs/MatchScoreBreakdown2016.md)
 - [MatchScoreBreakdown2016Alliance](docs/MatchScoreBreakdown2016Alliance.md)
 - [MatchScoreBreakdown2017](docs/MatchScoreBreakdown2017.md)
 - [MatchScoreBreakdown2017Alliance](docs/MatchScoreBreakdown2017Alliance.md)
 - [MatchScoreBreakdown2018](docs/MatchScoreBreakdown2018.md)
 - [MatchScoreBreakdown2018Alliance](docs/MatchScoreBreakdown2018Alliance.md)
 - [MatchScoreBreakdown2019](docs/MatchScoreBreakdown2019.md)
 - [MatchScoreBreakdown2019Alliance](docs/MatchScoreBreakdown2019Alliance.md)
 - [MatchScoreBreakdown2020](docs/MatchScoreBreakdown2020.md)
 - [MatchScoreBreakdown2020Alliance](docs/MatchScoreBreakdown2020Alliance.md)
 - [MatchScoreBreakdown2022](docs/MatchScoreBreakdown2022.md)
 - [MatchScoreBreakdown2022Alliance](docs/MatchScoreBreakdown2022Alliance.md)
 - [MatchScoreBreakdown2023](docs/MatchScoreBreakdown2023.md)
 - [MatchScoreBreakdown2023Alliance](docs/MatchScoreBreakdown2023Alliance.md)
 - [MatchScoreBreakdown2023AllianceAutoCommunity](docs/MatchScoreBreakdown2023AllianceAutoCommunity.md)
 - [MatchScoreBreakdown2023AllianceLinksInner](docs/MatchScoreBreakdown2023AllianceLinksInner.md)
 - [MatchScoreBreakdown2024](docs/MatchScoreBreakdown2024.md)
 - [MatchScoreBreakdown2024Alliance](docs/MatchScoreBreakdown2024Alliance.md)
 - [MatchSimple](docs/MatchSimple.md)
 - [MatchSimpleAlliances](docs/MatchSimpleAlliances.md)
 - [MatchTimeseries2018](docs/MatchTimeseries2018.md)
 - [MatchVideosInner](docs/MatchVideosInner.md)
 - [Media](docs/Media.md)
 - [Team](docs/Team.md)
 - [TeamEventStatus](docs/TeamEventStatus.md)
 - [TeamEventStatusAlliance](docs/TeamEventStatusAlliance.md)
 - [TeamEventStatusAllianceBackup](docs/TeamEventStatusAllianceBackup.md)
 - [TeamEventStatusPlayoff](docs/TeamEventStatusPlayoff.md)
 - [TeamEventStatusRank](docs/TeamEventStatusRank.md)
 - [TeamEventStatusRankRanking](docs/TeamEventStatusRankRanking.md)
 - [TeamEventStatusRankSortOrderInfoInner](docs/TeamEventStatusRankSortOrderInfoInner.md)
 - [TeamRobot](docs/TeamRobot.md)
 - [TeamSimple](docs/TeamSimple.md)
 - [Webcast](docs/Webcast.md)
 - [WltRecord](docs/WltRecord.md)
 - [Zebra](docs/Zebra.md)
 - [ZebraAlliances](docs/ZebraAlliances.md)
 - [ZebraTeam](docs/ZebraTeam.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



