# \ScheduleApi

All URIs are relative to *https://api.allscreenshots.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_schedule**](ScheduleApi.md#create_schedule) | **POST** /v1/schedules | 
[**delete_schedule**](ScheduleApi.md#delete_schedule) | **DELETE** /v1/schedules/{id} | 
[**get_execution_history**](ScheduleApi.md#get_execution_history) | **GET** /v1/schedules/{id}/history | 
[**get_schedule**](ScheduleApi.md#get_schedule) | **GET** /v1/schedules/{id} | 
[**list_schedules**](ScheduleApi.md#list_schedules) | **GET** /v1/schedules | 
[**pause_schedule**](ScheduleApi.md#pause_schedule) | **POST** /v1/schedules/{id}/pause | 
[**resume_schedule**](ScheduleApi.md#resume_schedule) | **POST** /v1/schedules/{id}/resume | 
[**trigger_schedule**](ScheduleApi.md#trigger_schedule) | **POST** /v1/schedules/{id}/trigger | 
[**update_schedule**](ScheduleApi.md#update_schedule) | **PUT** /v1/schedules/{id} | 



## create_schedule

> models::ScheduleResponse create_schedule(create_schedule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_schedule_request** | [**CreateScheduleRequest**](CreateScheduleRequest.md) |  | [required] |

### Return type

[**models::ScheduleResponse**](ScheduleResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_schedule

> delete_schedule(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_execution_history

> models::ScheduleHistoryResponse get_execution_history(id, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**models::ScheduleHistoryResponse**](ScheduleHistoryResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedule

> models::ScheduleResponse get_schedule(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ScheduleResponse**](ScheduleResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_schedules

> models::ScheduleListResponse list_schedules()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ScheduleListResponse**](ScheduleListResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_schedule

> models::ScheduleResponse pause_schedule(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ScheduleResponse**](ScheduleResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_schedule

> models::ScheduleResponse resume_schedule(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ScheduleResponse**](ScheduleResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_schedule

> models::ScheduleResponse trigger_schedule(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ScheduleResponse**](ScheduleResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_schedule

> models::ScheduleResponse update_schedule(id, update_schedule_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_schedule_request** | [**UpdateScheduleRequest**](UpdateScheduleRequest.md) |  | [required] |

### Return type

[**models::ScheduleResponse**](ScheduleResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

