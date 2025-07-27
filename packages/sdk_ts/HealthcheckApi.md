# .HealthcheckApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthcheckGet**](HealthcheckApi.md#healthcheckGet) | **GET** /healthcheck | 


# **healthcheckGet**
> string healthcheckGet()


### Example


```typescript
import { createConfiguration, HealthcheckApi } from '';

const configuration = createConfiguration();
const apiInstance = new HealthcheckApi(configuration);

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


