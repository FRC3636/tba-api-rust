# \ListApi

All URIs are relative to *https://www.thebluealliance.com/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_district_events_1**](ListApi.md#get_district_events_1) | **GET** /district/{district_key}/events | 
[**get_district_events_keys_1**](ListApi.md#get_district_events_keys_1) | **GET** /district/{district_key}/events/keys | 
[**get_district_events_simple_1**](ListApi.md#get_district_events_simple_1) | **GET** /district/{district_key}/events/simple | 
[**get_district_rankings_1**](ListApi.md#get_district_rankings_1) | **GET** /district/{district_key}/rankings | 
[**get_district_teams_1**](ListApi.md#get_district_teams_1) | **GET** /district/{district_key}/teams | 
[**get_district_teams_keys_1**](ListApi.md#get_district_teams_keys_1) | **GET** /district/{district_key}/teams/keys | 
[**get_district_teams_simple_1**](ListApi.md#get_district_teams_simple_1) | **GET** /district/{district_key}/teams/simple | 
[**get_event_teams_1**](ListApi.md#get_event_teams_1) | **GET** /event/{event_key}/teams | 
[**get_event_teams_keys_1**](ListApi.md#get_event_teams_keys_1) | **GET** /event/{event_key}/teams/keys | 
[**get_event_teams_simple_1**](ListApi.md#get_event_teams_simple_1) | **GET** /event/{event_key}/teams/simple | 
[**get_event_teams_statuses_1**](ListApi.md#get_event_teams_statuses_1) | **GET** /event/{event_key}/teams/statuses | 
[**get_events_by_year_0**](ListApi.md#get_events_by_year_0) | **GET** /events/{year} | 
[**get_events_by_year_keys_0**](ListApi.md#get_events_by_year_keys_0) | **GET** /events/{year}/keys | 
[**get_events_by_year_simple_0**](ListApi.md#get_events_by_year_simple_0) | **GET** /events/{year}/simple | 
[**get_insights_leaderboards_year_0**](ListApi.md#get_insights_leaderboards_year_0) | **GET** /insights/leaderboards/{year} | 
[**get_team_events_statuses_by_year**](ListApi.md#get_team_events_statuses_by_year) | **GET** /team/{team_key}/events/{year}/statuses | 
[**get_teams_0**](ListApi.md#get_teams_0) | **GET** /teams/{page_num} | 
[**get_teams_by_year_0**](ListApi.md#get_teams_by_year_0) | **GET** /teams/{year}/{page_num} | 
[**get_teams_by_year_keys_0**](ListApi.md#get_teams_by_year_keys_0) | **GET** /teams/{year}/{page_num}/keys | 
[**get_teams_by_year_simple_0**](ListApi.md#get_teams_by_year_simple_0) | **GET** /teams/{year}/{page_num}/simple | 
[**get_teams_keys_0**](ListApi.md#get_teams_keys_0) | **GET** /teams/{page_num}/keys | 
[**get_teams_simple_0**](ListApi.md#get_teams_simple_0) | **GET** /teams/{page_num}/simple | 



## get_district_events_1

> Vec<models::Event> get_district_events_1(district_key, if_none_match)


Gets a list of events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_events_keys_1

> Vec<String> get_district_events_keys_1(district_key, if_none_match)


Gets a list of event keys for events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_events_simple_1

> Vec<models::EventSimple> get_district_events_simple_1(district_key, if_none_match)


Gets a short-form list of events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::EventSimple>**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_rankings_1

> Vec<models::DistrictRanking> get_district_rankings_1(district_key, if_none_match)


Gets a list of team district rankings for the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::DistrictRanking>**](District_Ranking.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams_1

> Vec<models::Team> get_district_teams_1(district_key, if_none_match)


Gets a list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams_keys_1

> Vec<String> get_district_teams_keys_1(district_key, if_none_match)


Gets a list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_district_teams_simple_1

> Vec<models::TeamSimple> get_district_teams_simple_1(district_key, if_none_match)


Gets a short-form list of `Team` objects that competed in events in the given district.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**district_key** | **String** | TBA District Key, eg `2016fim` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_1

> Vec<models::Team> get_event_teams_1(event_key, if_none_match)


Gets a list of `Team` objects that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_keys_1

> Vec<String> get_event_teams_keys_1(event_key, if_none_match)


Gets a list of `Team` keys that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_simple_1

> Vec<models::TeamSimple> get_event_teams_simple_1(event_key, if_none_match)


Gets a short-form list of `Team` objects that competed in the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_teams_statuses_1

> std::collections::HashMap<String, models::GetTeamEventsStatusesByYear200ResponseValue> get_event_teams_statuses_1(event_key, if_none_match)


Gets a key-value list of the event statuses for teams competing at the given event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_key** | **String** | TBA Event Key, eg `2016nytr` | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**std::collections::HashMap<String, models::GetTeamEventsStatusesByYear200ResponseValue>**](getTeamEventsStatusesByYear_200_response_value.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year_0

> Vec<models::Event> get_events_by_year_0(year, if_none_match)


Gets a list of events in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Event>**](Event.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year_keys_0

> Vec<String> get_events_by_year_keys_0(year, if_none_match)


Gets a list of event keys in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events_by_year_simple_0

> Vec<models::EventSimple> get_events_by_year_simple_0(year, if_none_match)


Gets a short-form list of events in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::EventSimple>**](Event_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_insights_leaderboards_year_0

> Vec<models::LeaderboardInsight> get_insights_leaderboards_year_0(year, if_none_match)


Gets a list of `LeaderboardInsight` objects from a specific year. Use year=0 for overall.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::LeaderboardInsight>**](LeaderboardInsight.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team_events_statuses_by_year

> std::collections::HashMap<String, models::GetTeamEventsStatusesByYear200ResponseValue> get_team_events_statuses_by_year(team_key, year, if_none_match)


Gets a key-value list of the event statuses for events this team has competed at in the given year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_key** | **String** | TBA Team Key, eg `frc254` | [required] |
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**std::collections::HashMap<String, models::GetTeamEventsStatusesByYear200ResponseValue>**](getTeamEventsStatusesByYear_200_response_value.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_0

> Vec<models::Team> get_teams_0(page_num, if_none_match)


Gets a list of `Team` objects, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_by_year_0

> Vec<models::Team> get_teams_by_year_0(year, page_num, if_none_match)


Gets a list of `Team` objects that competed in the given year, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::Team>**](Team.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_by_year_keys_0

> Vec<String> get_teams_by_year_keys_0(year, page_num, if_none_match)


Gets a list Team Keys that competed in the given year, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_by_year_simple_0

> Vec<models::TeamSimple> get_teams_by_year_simple_0(year, page_num, if_none_match)


Gets a list of short form `Team_Simple` objects that competed in the given year, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Competition Year (or Season). Must be 4 digits. | [required] |
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_keys_0

> Vec<String> get_teams_keys_0(page_num, if_none_match)


Gets a list of Team keys, paginated in groups of 500. (Note, each page will not have 500 teams, but will include the teams within that range of 500.)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_teams_simple_0

> Vec<models::TeamSimple> get_teams_simple_0(page_num, if_none_match)


Gets a list of short form `Team_Simple` objects, paginated in groups of 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_num** | **i32** | Page number of results to return, zero-indexed | [required] |
**if_none_match** | Option<**String**> | Value of the `ETag` header in the most recently cached response by the client. |  |

### Return type

[**Vec<models::TeamSimple>**](Team_Simple.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

