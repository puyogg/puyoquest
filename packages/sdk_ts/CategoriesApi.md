# .CategoriesApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**categoriesAllGet**](CategoriesApi.md#categoriesAllGet) | **GET** /categories/all | 
[**categoriesGet**](CategoriesApi.md#categoriesGet) | **GET** /categories | 


# **categoriesAllGet**
> Array<string> categoriesAllGet()


### Example


```typescript
import { createConfiguration, CategoriesApi } from '';

const configuration = createConfiguration();
const apiInstance = new CategoriesApi(configuration);

const request = {};

const data = await apiInstance.categoriesAllGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**Array<string>**

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

# **categoriesGet**
> Array<string> categoriesGet()


### Example


```typescript
import { createConfiguration, CategoriesApi } from '';
import type { CategoriesApiCategoriesGetRequest } from '';

const configuration = createConfiguration();
const apiInstance = new CategoriesApi(configuration);

const request: CategoriesApiCategoriesGetRequest = {
  
  name: "name_example",
  
  limit: 10,
  
  exact: false,
};

const data = await apiInstance.categoriesGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | [**string**] |  | defaults to undefined
 **limit** | [**number**] |  | (optional) defaults to 10
 **exact** | [**boolean**] |  | (optional) defaults to false


### Return type

**Array<string>**

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


