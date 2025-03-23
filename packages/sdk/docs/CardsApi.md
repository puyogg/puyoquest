# \CardsApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cards_card_id_full_art_get**](CardsApi.md#cards_card_id_full_art_get) | **GET** /cards/{card_id}/full-art | Get card full art (all orientations)
[**cards_card_id_lore_get**](CardsApi.md#cards_card_id_lore_get) | **GET** /cards/{card_id}/lore | Get card lore
[**cards_category_search_get**](CardsApi.md#cards_category_search_get) | **GET** /cards/category-search | 
[**cards_get**](CardsApi.md#cards_get) | **GET** /cards | Find by name and rarity
[**cards_id_get**](CardsApi.md#cards_id_get) | **GET** /cards/{id} | Find by card_id
[**cards_post**](CardsApi.md#cards_post) | **POST** /cards | Upsert card data (admins only)
[**cards_random_card_get**](CardsApi.md#cards_random_card_get) | **GET** /cards/random-card | List random cards
[**cards_random_lore_get**](CardsApi.md#cards_random_lore_get) | **GET** /cards/random-lore | 



## cards_card_id_full_art_get

> models::CardFullArtUrls cards_card_id_full_art_get(card_id)
Get card full art (all orientations)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **String** |  | [required] |

### Return type

[**models::CardFullArtUrls**](CardFullArtUrls.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cards_card_id_lore_get

> models::Lore cards_card_id_lore_get(card_id)
Get card lore

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **String** |  | [required] |

### Return type

[**models::Lore**](Lore.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cards_category_search_get

> Vec<models::Card> cards_category_search_get(categories)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**categories** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**Vec<models::Card>**](Card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cards_get

> models::Card cards_get(name, rarity)
Find by name and rarity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |
**rarity** | Option<**String**> |  |  |

### Return type

[**models::Card**](Card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cards_id_get

> models::Card cards_id_get(id)
Find by card_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Card**](Card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cards_post

> models::Card cards_post(card_create)
Upsert card data (admins only)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_create** | [**CardCreate**](CardCreate.md) |  | [required] |

### Return type

[**models::Card**](Card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cards_random_card_get

> Vec<models::Card> cards_random_card_get(count, exclude)
List random cards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | **i32** |  | [required] |
**exclude** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<models::Card>**](Card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cards_random_lore_get

> models::Lore cards_random_lore_get()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Lore**](Lore.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

