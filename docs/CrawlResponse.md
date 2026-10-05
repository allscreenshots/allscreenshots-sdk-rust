# CrawlResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**depth** | **i32** |  | 
**error_code** | Option<**String**> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**id** | **String** |  | 
**limit** | **i32** |  | 
**outputs** | [**HashSet<models::OutputSpec>**](OutputSpec.md) |  | 
**progress** | [**models::CrawlProgressResponse**](CrawlProgressResponse.md) |  | 
**started_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**status** | **Status** |  (enum: QUEUED, RUNNING, COMPLETED, PARTIALLY_COMPLETED, FAILED, CANCELLED) | 
**url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


