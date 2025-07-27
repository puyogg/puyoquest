# .CardsApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cardsCardIdFullArtGet**](CardsApi.md#cardsCardIdFullArtGet) | **GET** /cards/{card_id}/full-art | Get card full art (all orientations)
[**cardsCardIdLoreGet**](CardsApi.md#cardsCardIdLoreGet) | **GET** /cards/{card_id}/lore | Get card lore
[**cardsCategorySearchGet**](CardsApi.md#cardsCategorySearchGet) | **GET** /cards/category-search | 
[**cardsGet**](CardsApi.md#cardsGet) | **GET** /cards | Find by name and rarity
[**cardsIdGet**](CardsApi.md#cardsIdGet) | **GET** /cards/{id} | Find by card_id
[**cardsPost**](CardsApi.md#cardsPost) | **POST** /cards | Upsert card data (admins only)
[**cardsRandomCardGet**](CardsApi.md#cardsRandomCardGet) | **GET** /cards/random-card | List random cards
[**cardsRandomLoreGet**](CardsApi.md#cardsRandomLoreGet) | **GET** /cards/random-lore | 


# **cardsCardIdFullArtGet**
> CardFullArtUrls cardsCardIdFullArtGet()


### Example


```typescript
import { createConfiguration, CardsApi } from '';
import type { CardsApiCardsCardIdFullArtGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CardsApi(configuration);

const request: CardsApiCardsCardIdFullArtGetRequest = {
  
  cardId: "card_id_example",
};

const data = await apiInstance.cardsCardIdFullArtGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cardId** | [**string**] |  | defaults to undefined


### Return type

**CardFullArtUrls**

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

# **cardsCardIdLoreGet**
> Lore cardsCardIdLoreGet()


### Example


```typescript
import { createConfiguration, CardsApi } from '';
import type { CardsApiCardsCardIdLoreGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CardsApi(configuration);

const request: CardsApiCardsCardIdLoreGetRequest = {
  
  cardId: "card_id_example",
};

const data = await apiInstance.cardsCardIdLoreGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cardId** | [**string**] |  | defaults to undefined


### Return type

**Lore**

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

# **cardsCategorySearchGet**
> Array<Card> cardsCategorySearchGet()


### Example


```typescript
import { createConfiguration, CardsApi } from '';
import type { CardsApiCardsCategorySearchGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CardsApi(configuration);

const request: CardsApiCardsCategorySearchGetRequest = {
  
  categories: [
    "categories_example",
  ],
};

const data = await apiInstance.cardsCategorySearchGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **categories** | **Array&lt;string&gt;** |  | defaults to undefined


### Return type

**Array<Card>**

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

# **cardsGet**
> Card cardsGet()


### Example


```typescript
import { createConfiguration, CardsApi } from '';
import type { CardsApiCardsGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CardsApi(configuration);

const request: CardsApiCardsGetRequest = {
  
  name: "name_example",
  
  rarity: "rarity_example",
};

const data = await apiInstance.cardsGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | [**string**] |  | (optional) defaults to undefined
 **rarity** | [**string**] |  | (optional) defaults to undefined


### Return type

**Card**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |
**400** |  |  -  |
**404** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **cardsIdGet**
> Card cardsIdGet()


### Example


```typescript
import { createConfiguration, CardsApi } from '';
import type { CardsApiCardsIdGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CardsApi(configuration);

const request: CardsApiCardsIdGetRequest = {
  
  id: "id_example",
};

const data = await apiInstance.cardsIdGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | [**string**] |  | defaults to undefined


### Return type

**Card**

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

# **cardsPost**
> Card cardsPost(cardCreate)


### Example


```typescript
import { createConfiguration, CardsApi } from '';
import type { CardsApiCardsPostRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CardsApi(configuration);

const request: CardsApiCardsPostRequest = {
  
  cardCreate: {
    cardId: "cardId_example",
    charId: "charId_example",
    rarity: "rarity_example",
    rarityModifier: "rarityModifier_example",
    name: "name_example",
    nameNormalized: "nameNormalized_example",
    jpName: "jpName_example",
    jpNameNormalized: "jpNameNormalized_example",
    linkName: "linkName_example",
    linkNameNormalized: "linkNameNormalized_example",
    cardType: "character",
    mainColor: "mainColor_example",
    sideColor: "sideColor_example",
    updatedAt: new Date('1970-01-01T00:00:00.00Z'),
  },
};

const data = await apiInstance.cardsPost(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cardCreate** | **CardCreate**|  |


### Return type

**Card**

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

# **cardsRandomCardGet**
> Array<Card> cardsRandomCardGet()


### Example


```typescript
import { createConfiguration, CardsApi } from '';
import type { CardsApiCardsRandomCardGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CardsApi(configuration);

const request: CardsApiCardsRandomCardGetRequest = {
  
  count: 1.0,
  
  exclude: [
    "exclude_example",
  ],
};

const data = await apiInstance.cardsRandomCardGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | [**number**] |  | defaults to undefined
 **exclude** | **Array&lt;string&gt;** |  | (optional) defaults to undefined


### Return type

**Array<Card>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  * CACHE-CONTROL -  <br>  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **cardsRandomLoreGet**
> Lore cardsRandomLoreGet()


### Example


```typescript
import { createConfiguration, CardsApi } from '';

const configuration = createConfiguration();
const apiInstance = new CardsApi(configuration);

const request = {};

const data = await apiInstance.cardsRandomLoreGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**Lore**

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


