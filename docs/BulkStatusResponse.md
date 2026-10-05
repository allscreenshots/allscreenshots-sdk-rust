# BulkStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**completed_jobs** | **i32** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**failed_jobs** | **i32** |  | 
**id** | **String** |  | 
**jobs** | [**Vec<models::BulkJobDetailInfo>**](BulkJobDetailInfo.md) |  | 
**progress** | **i32** |  | 
**status** | **String** |  | 
**total_jobs** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


