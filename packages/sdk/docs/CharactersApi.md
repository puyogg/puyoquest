# \CharactersApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**characters_get**](CharactersApi.md#characters_get) | **GET** /characters | Find by alias or category
[**characters_id_aliases_get**](CharactersApi.md#characters_id_aliases_get) | **GET** /characters/{id}/aliases | 
[**characters_id_get**](CharactersApi.md#characters_id_get) | **GET** /characters/{id} | TODO: Option to refresh index
[**characters_id_put**](CharactersApi.md#characters_id_put) | **PUT** /characters/{id} | Create a character or update one if it already exists



## characters_get

> Vec<models::Character> characters_get(alias)
Find by alias or category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | Option<**String**> |  |  |

### Return type

[**Vec<models::Character>**](Character.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## characters_id_aliases_get

> Vec<models::Alias> characters_id_aliases_get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**Vec<models::Alias>**](Alias.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## characters_id_get

> models::Character characters_id_get(id)
TODO: Option to refresh index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::Character**](Character.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## characters_id_put

> models::Character characters_id_put(id, character_create)
Create a character or update one if it already exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**character_create** | [**CharacterCreate**](CharacterCreate.md) |  | [required] |

### Return type

[**models::Character**](Character.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

