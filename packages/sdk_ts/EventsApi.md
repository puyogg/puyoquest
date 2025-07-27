# .EventsApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**eventsGet**](EventsApi.md#eventsGet) | **GET** /events | 


# **eventsGet**
> PpqEventSchedule eventsGet()


### Example


```typescript
import { createConfiguration, EventsApi } from '';

const configuration = createConfiguration();
const apiInstance = new EventsApi(configuration);

const request = {};

const data = await apiInstance.eventsGet(request);
console.log('API called successfully. Returned data:', data);
```


### Parameters
This endpoint does not need any parameter.


### Return type

**PpqEventSchedule**

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


