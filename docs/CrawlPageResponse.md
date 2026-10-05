# CrawlPageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_url** | Option<**String**> |  | [optional]
**completed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**content_type** | Option<**String**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**depth** | **i32** |  | 
**description** | Option<**String**> |  | [optional]
**error_code** | Option<**String**> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**http_status** | Option<**i32**> |  | [optional]
**id** | **String** |  | 
**language** | Option<**String**> |  | [optional]
**outputs** | [**Vec<models::OutputSpec>**](OutputSpec.md) |  | 
**parent_id** | Option<**String**> |  | [optional]
**render_time_ms** | Option<**i64**> |  | [optional]
**status** | **Status** |  (enum: QUEUED, RUNNING, COMPLETED, FAILED, SKIPPED, CANCELLED) | 
**title** | Option<**String**> |  | [optional]
**url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


