# .CharactersApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**charactersGet**](CharactersApi.md#charactersGet) | **GET** /characters | Find by alias or category
[**charactersIdAliasesGet**](CharactersApi.md#charactersIdAliasesGet) | **GET** /characters/{id}/aliases | 
[**charactersIdCardsGet**](CharactersApi.md#charactersIdCardsGet) | **GET** /characters/{id}/cards | 
[**charactersIdGet**](CharactersApi.md#charactersIdGet) | **GET** /characters/{id} | TODO: Option to refresh index
[**charactersIdPut**](CharactersApi.md#charactersIdPut) | **PUT** /characters/{id} | Create a character or update one if it already exists


# **charactersGet**
> Array<Character> charactersGet()


### Example


```typescript
import { createConfiguration, CharactersApi } from '';
import type { CharactersApiCharactersGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CharactersApi(configuration);

const request: CharactersApiCharactersGetRequest = {
  
  alias: "alias_example",
};

const data = await apiInstance.charactersGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **alias** | [**string**] |  | (optional) defaults to undefined


### Return type

**Array<Character>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |
**400** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **charactersIdAliasesGet**
> Array<Alias> charactersIdAliasesGet()


### Example


```typescript
import { createConfiguration, CharactersApi } from '';
import type { CharactersApiCharactersIdAliasesGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CharactersApi(configuration);

const request: CharactersApiCharactersIdAliasesGetRequest = {
  
  id: "id_example",
};

const data = await apiInstance.charactersIdAliasesGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**string**] |  | defaults to undefined


### Return type

**Array<Alias>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |
**400** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **charactersIdCardsGet**
> CardsAndMaterials charactersIdCardsGet()


### Example


```typescript
import { createConfiguration, CharactersApi } from '';
import type { CharactersApiCharactersIdCardsGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CharactersApi(configuration);

const request: CharactersApiCharactersIdCardsGetRequest = {
  
  id: "id_example",
    // Valid values: \"true\", \"false\". Default false. (optional)
  fetchFresh: "fetch_fresh_example",
};

const data = await apiInstance.charactersIdCardsGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**string**] |  | defaults to undefined
 **fetchFresh** | [**string**] | Valid values: \&quot;true\&quot;, \&quot;false\&quot;. Default false. | (optional) defaults to undefined


### Return type

**CardsAndMaterials**

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

# **charactersIdGet**
> Character charactersIdGet()


### Example


```typescript
import { createConfiguration, CharactersApi } from '';
import type { CharactersApiCharactersIdGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CharactersApi(configuration);

const request: CharactersApiCharactersIdGetRequest = {
  
  id: "id_example",
};

const data = await apiInstance.charactersIdGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**string**] |  | defaults to undefined


### Return type

**Character**

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

# **charactersIdPut**
> Character charactersIdPut(characterCreate)


### Example


```typescript
import { createConfiguration, CharactersApi } from '';
import type { CharactersApiCharactersIdPutRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CharactersApi(configuration);

const request: CharactersApiCharactersIdPutRequest = {
  
  id: "id_example",
  
  characterCreate: {
    name: "name_example",
    jpName: "jpName_example",
    linkName: "linkName_example",
    mainColor: "mainColor_example",
    sideColor: "sideColor_example",
    type1: "type1_example",
    type2: "type2_example",
    voiceTrans: "voiceTrans_example",
    updatedAt: new Date('1970-01-01T00:00:00.00Z'),
  },
};

const data = await apiInstance.charactersIdPut(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **characterCreate** | **CharacterCreate**|  |
 **id** | [**string**] |  | defaults to undefined


### Return type

**Character**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json; charset=utf-8
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  * LOCATION -  <br>  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)


