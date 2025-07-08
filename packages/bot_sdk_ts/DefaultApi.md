# .DefaultApi

All URIs are relative to *http://localhost:3001*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthcheckGet**](DefaultApi.md#healthcheckGet) | **GET** /healthcheck | 
[**kagaDelete**](DefaultApi.md#kagaDelete) | **DELETE** /kaga | Delete a kaga image url
[**kagaGet**](DefaultApi.md#kagaGet) | **GET** /kaga | Get a kaga image url
[**kagaPut**](DefaultApi.md#kagaPut) | **PUT** /kaga | Set a kaga image url
[**leaderboardChannelServerIdGameTypeDelete**](DefaultApi.md#leaderboardChannelServerIdGameTypeDelete) | **DELETE** /leaderboard-channel/{server_id}/{game_type} | 
[**leaderboardChannelServerIdGameTypeGet**](DefaultApi.md#leaderboardChannelServerIdGameTypeGet) | **GET** /leaderboard-channel/{server_id}/{game_type} | 
[**leaderboardChannelServerIdGameTypePost**](DefaultApi.md#leaderboardChannelServerIdGameTypePost) | **POST** /leaderboard-channel/{server_id}/{game_type} | 
[**leaderboardsGameTypeDelete**](DefaultApi.md#leaderboardsGameTypeDelete) | **DELETE** /leaderboards/{game_type} | Delete all the results for a game_type across all servers. I don\&#39;t want to hold Discord user data for very long.
[**leaderboardsServerIdGameTypePlayerCountGet**](DefaultApi.md#leaderboardsServerIdGameTypePlayerCountGet) | **GET** /leaderboards/{server_id}/{game_type}/player_count | Get the player count for a game and server
[**leaderboardsServerIdGameTypeTopGet**](DefaultApi.md#leaderboardsServerIdGameTypeTopGet) | **GET** /leaderboards/{server_id}/{game_type}/top | Fetch the top 10 players for a game type on a server
[**leaderboardsServerIdGameTypeUserIdIncrementPost**](DefaultApi.md#leaderboardsServerIdGameTypeUserIdIncrementPost) | **POST** /leaderboards/{server_id}/{game_type}/{user_id}/increment | Upsert a user ranking and increment their score
[**leaderboardsServerIdGameTypeWindowGet**](DefaultApi.md#leaderboardsServerIdGameTypeWindowGet) | **GET** /leaderboards/{server_id}/{game_type}/window | Fetch a user\&#39;s ranking and the 9 players surrounding them. This tries to show 10 users unless the leaderboard+game currently has less than 10 players.
[**serverSettingsExistsPost**](DefaultApi.md#serverSettingsExistsPost) | **POST** /server-settings/exists | Check if server settings exists for the provided server ids
[**serverSettingsPost**](DefaultApi.md#serverSettingsPost) | **POST** /server-settings | 
[**serverSettingsServerIdDelete**](DefaultApi.md#serverSettingsServerIdDelete) | **DELETE** /server-settings/{server_id} | 
[**serverSettingsServerIdGet**](DefaultApi.md#serverSettingsServerIdGet) | **GET** /server-settings/{server_id} | 


# **healthcheckGet**
> string healthcheckGet()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request = {};

const data = await apiInstance.healthcheckGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **kagaDelete**
> void kagaDelete()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiKagaDeleteRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiKagaDeleteRequest = {
  
  id: "id_example",
};

const data = await apiInstance.kagaDelete(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**string**] |  | defaults to undefined


### Return type

**void**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **kagaGet**
> KagaData kagaGet()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiKagaGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiKagaGetRequest = {
  
  id: "id_example",
};

const data = await apiInstance.kagaGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**string**] |  | defaults to undefined


### Return type

**KagaData**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |
**404** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **kagaPut**
> KagaData kagaPut(kagaData)


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiKagaPutRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiKagaPutRequest = {
  
  kagaData: {
    kagaId: "kagaId_example",
    url: "url_example",
  },
};

const data = await apiInstance.kagaPut(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **kagaData** | **KagaData**|  |


### Return type

**KagaData**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json; charset=utf-8
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **leaderboardChannelServerIdGameTypeDelete**
> string leaderboardChannelServerIdGameTypeDelete()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiLeaderboardChannelServerIdGameTypeDeleteRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiLeaderboardChannelServerIdGameTypeDeleteRequest = {
  
  serverId: "server_id_example",
  
  gameType: "game_type_example",
};

const data = await apiInstance.leaderboardChannelServerIdGameTypeDelete(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverId** | [**string**] |  | defaults to undefined
 **gameType** | [**string**] |  | defaults to undefined


### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **leaderboardChannelServerIdGameTypeGet**
> LeaderboardChannel leaderboardChannelServerIdGameTypeGet()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiLeaderboardChannelServerIdGameTypeGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiLeaderboardChannelServerIdGameTypeGetRequest = {
  
  serverId: "server_id_example",
  
  gameType: "game_type_example",
};

const data = await apiInstance.leaderboardChannelServerIdGameTypeGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverId** | [**string**] |  | defaults to undefined
 **gameType** | [**string**] |  | defaults to undefined


### Return type

**LeaderboardChannel**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |
**404** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **leaderboardChannelServerIdGameTypePost**
> LeaderboardChannel leaderboardChannelServerIdGameTypePost(leaderboardChannelCreate)


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiLeaderboardChannelServerIdGameTypePostRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiLeaderboardChannelServerIdGameTypePostRequest = {
  
  serverId: "server_id_example",
  
  gameType: "game_type_example",
  
  leaderboardChannelCreate: {
    channelId: "channelId_example",
  },
};

const data = await apiInstance.leaderboardChannelServerIdGameTypePost(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **leaderboardChannelCreate** | **LeaderboardChannelCreate**|  |
 **serverId** | [**string**] |  | defaults to undefined
 **gameType** | [**string**] |  | defaults to undefined


### Return type

**LeaderboardChannel**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json; charset=utf-8
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **leaderboardsGameTypeDelete**
> string leaderboardsGameTypeDelete()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiLeaderboardsGameTypeDeleteRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiLeaderboardsGameTypeDeleteRequest = {
  
  gameType: "game_type_example",
};

const data = await apiInstance.leaderboardsGameTypeDelete(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **gameType** | [**string**] |  | defaults to undefined


### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **leaderboardsServerIdGameTypePlayerCountGet**
> number leaderboardsServerIdGameTypePlayerCountGet()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiLeaderboardsServerIdGameTypePlayerCountGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiLeaderboardsServerIdGameTypePlayerCountGetRequest = {
  
  serverId: "server_id_example",
  
  gameType: "game_type_example",
};

const data = await apiInstance.leaderboardsServerIdGameTypePlayerCountGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverId** | [**string**] |  | defaults to undefined
 **gameType** | [**string**] |  | defaults to undefined


### Return type

**number**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **leaderboardsServerIdGameTypeTopGet**
> Array<UserRanking> leaderboardsServerIdGameTypeTopGet()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiLeaderboardsServerIdGameTypeTopGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiLeaderboardsServerIdGameTypeTopGetRequest = {
  
  serverId: "server_id_example",
  
  gameType: "game_type_example",
};

const data = await apiInstance.leaderboardsServerIdGameTypeTopGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverId** | [**string**] |  | defaults to undefined
 **gameType** | [**string**] |  | defaults to undefined


### Return type

**Array<UserRanking>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **leaderboardsServerIdGameTypeUserIdIncrementPost**
> UserRanking leaderboardsServerIdGameTypeUserIdIncrementPost()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiLeaderboardsServerIdGameTypeUserIdIncrementPostRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiLeaderboardsServerIdGameTypeUserIdIncrementPostRequest = {
  
  serverId: "server_id_example",
  
  gameType: "game_type_example",
  
  userId: "user_id_example",
};

const data = await apiInstance.leaderboardsServerIdGameTypeUserIdIncrementPost(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverId** | [**string**] |  | defaults to undefined
 **gameType** | [**string**] |  | defaults to undefined
 **userId** | [**string**] |  | defaults to undefined


### Return type

**UserRanking**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **leaderboardsServerIdGameTypeWindowGet**
> Array<UserRanking> leaderboardsServerIdGameTypeWindowGet()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiLeaderboardsServerIdGameTypeWindowGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiLeaderboardsServerIdGameTypeWindowGetRequest = {
  
  serverId: "server_id_example",
  
  gameType: "game_type_example",
  
  userId: "user_id_example",
};

const data = await apiInstance.leaderboardsServerIdGameTypeWindowGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverId** | [**string**] |  | defaults to undefined
 **gameType** | [**string**] |  | defaults to undefined
 **userId** | [**string**] |  | defaults to undefined


### Return type

**Array<UserRanking>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **serverSettingsExistsPost**
> { [key: string]: boolean; } serverSettingsExistsPost(requestBody)


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiServerSettingsExistsPostRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiServerSettingsExistsPostRequest = {
  
  requestBody: [
    "requestBody_example",
  ],
};

const data = await apiInstance.serverSettingsExistsPost(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **requestBody** | **Array<string>**|  |


### Return type

**{ [key: string]: boolean; }**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json; charset=utf-8
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **serverSettingsPost**
> ServerSettings serverSettingsPost(serverSettings)


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiServerSettingsPostRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiServerSettingsPostRequest = {
  
  serverSettings: {
    serverId: "serverId_example",
  },
};

const data = await apiInstance.serverSettingsPost(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverSettings** | **ServerSettings**|  |


### Return type

**ServerSettings**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json; charset=utf-8
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **serverSettingsServerIdDelete**
> string serverSettingsServerIdDelete()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiServerSettingsServerIdDeleteRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiServerSettingsServerIdDeleteRequest = {
  
  serverId: "server_id_example",
};

const data = await apiInstance.serverSettingsServerIdDelete(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverId** | [**string**] |  | defaults to undefined


### Return type

**string**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **serverSettingsServerIdGet**
> ServerSettings serverSettingsServerIdGet()


### Example


```typescript
import { createConfiguration, DefaultApi } from '';
import type { DefaultApiServerSettingsServerIdGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new DefaultApi(configuration);

const request: DefaultApiServerSettingsServerIdGetRequest = {
  
  serverId: "server_id_example",
};

const data = await apiInstance.serverSettingsServerIdGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **serverId** | [**string**] |  | defaults to undefined


### Return type

**ServerSettings**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |
**404** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)


