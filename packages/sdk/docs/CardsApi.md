# \CardsApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cards_card_id_lore_get**](CardsApi.md#cards_card_id_lore_get) | **GET** /cards/{card_id}/lore | Get card lore
[**cards_get**](CardsApi.md#cards_get) | **GET** /cards | Find by name and rarity
[**cards_id_get**](CardsApi.md#cards_id_get) | **GET** /cards/{id} | Find by card_id
[**cards_post**](CardsApi.md#cards_post) | **POST** /cards | Upsert card data (admins only)



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

