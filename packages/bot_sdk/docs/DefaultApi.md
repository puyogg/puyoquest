# \DefaultApi

All URIs are relative to *http://localhost:3001*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthcheck_get**](DefaultApi.md#healthcheck_get) | **GET** /healthcheck | 
[**leaderboards_game_type_delete**](DefaultApi.md#leaderboards_game_type_delete) | **DELETE** /leaderboards/{game_type} | Delete all the results for a game_type across all servers. I don't want to hold Discord user data for very long.
[**leaderboards_server_id_game_type_player_count_get**](DefaultApi.md#leaderboards_server_id_game_type_player_count_get) | **GET** /leaderboards/{server_id}/{game_type}/player_count | Get the player count for a game and server
[**leaderboards_server_id_game_type_top_get**](DefaultApi.md#leaderboards_server_id_game_type_top_get) | **GET** /leaderboards/{server_id}/{game_type}/top | Fetch the top 10 players for a game type on a server
[**leaderboards_server_id_game_type_user_id_increment_post**](DefaultApi.md#leaderboards_server_id_game_type_user_id_increment_post) | **POST** /leaderboards/{server_id}/{game_type}/{user_id}/increment | Upsert a user ranking and increment their score
[**leaderboards_server_id_game_type_window_get**](DefaultApi.md#leaderboards_server_id_game_type_window_get) | **GET** /leaderboards/{server_id}/{game_type}/window | Fetch a user's ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.



## healthcheck_get

> String healthcheck_get()


### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboards_game_type_delete

> String leaderboards_game_type_delete(game_type)
Delete all the results for a game_type across all servers. I don't want to hold Discord user data for very long.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_type** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboards_server_id_game_type_player_count_get

> i64 leaderboards_server_id_game_type_player_count_get(server_id, game_type)
Get the player count for a game and server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**game_type** | **String** |  | [required] |

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboards_server_id_game_type_top_get

> Vec<models::UserRanking> leaderboards_server_id_game_type_top_get(server_id, game_type)
Fetch the top 10 players for a game type on a server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**game_type** | **String** |  | [required] |

### Return type

[**Vec<models::UserRanking>**](UserRanking.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboards_server_id_game_type_user_id_increment_post

> models::UserRanking leaderboards_server_id_game_type_user_id_increment_post(server_id, game_type, user_id)
Upsert a user ranking and increment their score

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**game_type** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::UserRanking**](UserRanking.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboards_server_id_game_type_window_get

> Vec<models::UserRanking> leaderboards_server_id_game_type_window_get(server_id, game_type, user_id)
Fetch a user's ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**game_type** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**Vec<models::UserRanking>**](UserRanking.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

