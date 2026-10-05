# \UsageApi

All URIs are relative to *https://api.allscreenshots.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_activity**](UsageApi.md#get_activity) | **GET** /v1/usage/activity | 
[**get_quota**](UsageApi.md#get_quota) | **GET** /v1/usage/quota | 
[**get_usage**](UsageApi.md#get_usage) | **GET** /v1/usage | 
[**get_usage_by_key**](UsageApi.md#get_usage_by_key) | **GET** /v1/usage/keys | 



## get_activity

> Vec<models::DailyCaptureCountResponse> get_activity(days, api_key_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> |  |  |[default to 120]
**api_key_ids** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<models::DailyCaptureCountResponse>**](DailyCaptureCountResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quota

> models::QuotaStatusResponse get_quota()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::QuotaStatusResponse**](QuotaStatusResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage

> models::UsageResponse get_usage()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UsageResponse**](UsageResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_by_key

> models::ApiKeyUsageResponse get_usage_by_key()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiKeyUsageResponse**](ApiKeyUsageResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

