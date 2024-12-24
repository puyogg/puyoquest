# \AliasesApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aliases_delete**](AliasesApi.md#aliases_delete) | **DELETE** /aliases | 
[**aliases_get**](AliasesApi.md#aliases_get) | **GET** /aliases | List aliases for a char_id
[**aliases_post**](AliasesApi.md#aliases_post) | **POST** /aliases | 



## aliases_delete

> models::DeleteCount aliases_delete(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::DeleteCount**](DeleteCount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## aliases_get

> Vec<models::Alias> aliases_get(char_id, name, exact)
List aliases for a char_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**char_id** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |
**exact** | Option<**String**> |  |  |

### Return type

[**Vec<models::Alias>**](Alias.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## aliases_post

> models::Alias aliases_post(alias_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_create** | [**AliasCreate**](AliasCreate.md) |  | [required] |

### Return type

[**models::Alias**](Alias.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

