# .AliasesApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**aliasesDelete**](AliasesApi.md#aliasesDelete) | **DELETE** /aliases | 
[**aliasesGet**](AliasesApi.md#aliasesGet) | **GET** /aliases | List aliases for a char_id
[**aliasesPost**](AliasesApi.md#aliasesPost) | **POST** /aliases | 


# **aliasesDelete**
> DeleteCount aliasesDelete()


### Example


```typescript
import { createConfiguration, AliasesApi } from '';
import type { AliasesApiAliasesDeleteRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AliasesApi(configuration);

const request: AliasesApiAliasesDeleteRequest = {
  
  name: "name_example",
};

const data = await apiInstance.aliasesDelete(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | [**string**] |  | defaults to undefined


### Return type

**DeleteCount**

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
**403** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)

# **aliasesGet**
> Array<Alias> aliasesGet()


### Example


```typescript
import { createConfiguration, AliasesApi } from '';
import type { AliasesApiAliasesGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AliasesApi(configuration);

const request: AliasesApiAliasesGetRequest = {
  
  charId: "char_id_example",
  
  name: "name_example",
  
  exact: "exact_example",
};

const data = await apiInstance.aliasesGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **charId** | [**string**] |  | (optional) defaults to undefined
 **name** | [**string**] |  | (optional) defaults to undefined
 **exact** | [**string**] |  | (optional) defaults to undefined


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

# **aliasesPost**
> Alias aliasesPost(aliasCreate)


### Example


```typescript
import { createConfiguration, AliasesApi } from '';
import type { AliasesApiAliasesPostRequest } from '';

const configuration = createConfiguration();
const apiInstance = new AliasesApi(configuration);

const request: AliasesApiAliasesPostRequest = {
  
  aliasCreate: {
    alias: "alias_example",
    charId: "charId_example",
    internal: true,
    cardType: "character",
    updatedAt: new Date('1970-01-01T00:00:00.00Z'),
  },
};

const data = await apiInstance.aliasesPost(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **aliasCreate** | **AliasCreate**|  |


### Return type

**Alias**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json; charset=utf-8
 - **Accept**: application/json; charset=utf-8, text/plain; charset=utf-8


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** |  |  -  |
**400** |  |  -  |

[[Back to top]](#) [[Back to API list]](README.md#documentation-for-api-endpoints) [[Back to Model list]](README.md#documentation-for-models) [[Back to README]](README.md)


