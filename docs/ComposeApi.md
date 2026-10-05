# \ComposeApi

All URIs are relative to *https://api.allscreenshots.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**compose**](ComposeApi.md#compose) | **POST** /v1/screenshots/compose | 
[**get_compose_job_status**](ComposeApi.md#get_compose_job_status) | **GET** /v1/screenshots/compose/jobs/{jobId} | 
[**get_layout_preview**](ComposeApi.md#get_layout_preview) | **GET** /v1/screenshots/compose/preview | 
[**list_compose_jobs**](ComposeApi.md#list_compose_jobs) | **GET** /v1/screenshots/compose/jobs | 



## compose

> std::path::PathBuf compose(compose_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compose_request** | [**ComposeRequest**](ComposeRequest.md) |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compose_job_status

> models::ComposeJobStatusResponse get_compose_job_status(job_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::ComposeJobStatusResponse**](ComposeJobStatusResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_layout_preview

> models::LayoutPreviewResponse get_layout_preview(layout, image_count, canvas_width, canvas_height, aspect_ratios)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**layout** | **String** |  | [required] |
**image_count** | **i32** |  | [required] |
**canvas_width** | Option<**i32**> |  |  |[default to 1200]
**canvas_height** | Option<**i32**> |  |  |[default to 800]
**aspect_ratios** | Option<[**Vec<f64>**](F64.md)> |  |  |

### Return type

[**models::LayoutPreviewResponse**](LayoutPreviewResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_compose_jobs

> Vec<models::ComposeJobSummaryResponse> list_compose_jobs()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ComposeJobSummaryResponse>**](ComposeJobSummaryResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

