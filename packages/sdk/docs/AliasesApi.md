# \AliasesApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aliases_get**](AliasesApi.md#aliases_get) | **GET** /aliases | List aliases for a char_id
[**aliases_name_get**](AliasesApi.md#aliases_name_get) | **GET** /aliases/{name} | Find an alias by name
[**aliases_name_put**](AliasesApi.md#aliases_name_put) | **PUT** /aliases/{name} | 



## aliases_get

> Vec<models::Alias> aliases_get(char_id)
List aliases for a char_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**char_id** | Option<**String**> |  |  |

### Return type

[**Vec<models::Alias>**](Alias.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## aliases_name_get

> models::Alias aliases_name_get(name)
Find an alias by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::Alias**](Alias.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## aliases_name_put

> models::Alias aliases_name_put(name, alias_create)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**alias_create** | [**AliasCreate**](AliasCreate.md) |  | [required] |

### Return type

[**models::Alias**](Alias.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json; charset=utf-8
- **Accept**: application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

