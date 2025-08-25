# \DefaultApi

All URIs are relative to *http://localhost:3001*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthcheck_get**](DefaultApi.md#healthcheck_get) | **GET** /healthcheck | 
[**kaga_delete**](DefaultApi.md#kaga_delete) | **DELETE** /kaga | Delete a kaga image url
[**kaga_get**](DefaultApi.md#kaga_get) | **GET** /kaga | Get a kaga image url
[**kaga_put**](DefaultApi.md#kaga_put) | **PUT** /kaga | Set a kaga image url
[**kaga_random_get**](DefaultApi.md#kaga_random_get) | **GET** /kaga/random | Get a random kaga image
[**leaderboard_channel_server_id_game_type_delete**](DefaultApi.md#leaderboard_channel_server_id_game_type_delete) | **DELETE** /leaderboard-channel/{server_id}/{game_type} | 
[**leaderboard_channel_server_id_game_type_get**](DefaultApi.md#leaderboard_channel_server_id_game_type_get) | **GET** /leaderboard-channel/{server_id}/{game_type} | 
[**leaderboard_channel_server_id_game_type_post**](DefaultApi.md#leaderboard_channel_server_id_game_type_post) | **POST** /leaderboard-channel/{server_id}/{game_type} | 
[**leaderboards_game_type_delete**](DefaultApi.md#leaderboards_game_type_delete) | **DELETE** /leaderboards/{game_type} | Delete all the results for a game_type across all servers. I don't want to hold Discord user data for very long.
[**leaderboards_server_id_game_type_player_count_get**](DefaultApi.md#leaderboards_server_id_game_type_player_count_get) | **GET** /leaderboards/{server_id}/{game_type}/player_count | Get the player count for a game and server
[**leaderboards_server_id_game_type_top_get**](DefaultApi.md#leaderboards_server_id_game_type_top_get) | **GET** /leaderboards/{server_id}/{game_type}/top | Fetch the top 10 players for a game type on a server
[**leaderboards_server_id_game_type_user_id_increment_post**](DefaultApi.md#leaderboards_server_id_game_type_user_id_increment_post) | **POST** /leaderboards/{server_id}/{game_type}/{user_id}/increment | Upsert a user ranking and increment their score
[**leaderboards_server_id_game_type_window_get**](DefaultApi.md#leaderboards_server_id_game_type_window_get) | **GET** /leaderboards/{server_id}/{game_type}/window | Fetch a user's ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.
[**server_settings_exists_post**](DefaultApi.md#server_settings_exists_post) | **POST** /server-settings/exists | Check if server settings exists for the provided server ids
[**server_settings_post**](DefaultApi.md#server_settings_post) | **POST** /server-settings | 
[**server_settings_server_id_delete**](DefaultApi.md#server_settings_server_id_delete) | **DELETE** /server-settings/{server_id} | 
[**server_settings_server_id_get**](DefaultApi.md#server_settings_server_id_get) | **GET** /server-settings/{server_id} | 



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


## kaga_delete

> kaga_delete(id)
Delete a kaga image url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kaga_get

> models::KagaData kaga_get(id)
Get a kaga image url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::KagaData**](KagaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kaga_put

> models::KagaData kaga_put(kaga_data)
Set a kaga image url

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kaga_data** | [**KagaData**](KagaData.md) |  | [required] |

### Return type

[**models::KagaData**](KagaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kaga_random_get

> models::KagaData kaga_random_get()
Get a random kaga image

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::KagaData**](KagaData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboard_channel_server_id_game_type_delete

> String leaderboard_channel_server_id_game_type_delete(server_id, game_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**game_type** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboard_channel_server_id_game_type_get

> models::LeaderboardChannel leaderboard_channel_server_id_game_type_get(server_id, game_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**game_type** | **String** |  | [required] |

### Return type

[**models::LeaderboardChannel**](LeaderboardChannel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leaderboard_channel_server_id_game_type_post

> models::LeaderboardChannel leaderboard_channel_server_id_game_type_post(server_id, game_type, leaderboard_channel_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |
**game_type** | **String** |  | [required] |
**leaderboard_channel_create** | [**LeaderboardChannelCreate**](LeaderboardChannelCreate.md) |  | [required] |

### Return type

[**models::LeaderboardChannel**](LeaderboardChannel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

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


## server_settings_exists_post

> std::collections::HashMap<String, bool> server_settings_exists_post(request_body)
Check if server settings exists for the provided server ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) |  | [required] |

### Return type

**std::collections::HashMap<String, bool>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_settings_post

> models::ServerSettings server_settings_post(server_settings)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_settings** | [**ServerSettings**](ServerSettings.md) |  | [required] |

### Return type

[**models::ServerSettings**](ServerSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_settings_server_id_delete

> String server_settings_server_id_delete(server_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_settings_server_id_get

> models::ServerSettings server_settings_server_id_get(server_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** |  | [required] |

### Return type

[**models::ServerSettings**](ServerSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

