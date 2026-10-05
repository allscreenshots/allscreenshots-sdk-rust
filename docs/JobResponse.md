# JobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**error_code** | Option<**String**> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**id** | **String** |  | 
**metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**outputs** | Option<[**std::collections::HashMap<String, models::OutputInfo>**](OutputInfo.md)> |  | [optional]
**result_url** | Option<**String**> |  | [optional]
**started_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**status** | **Status** |  (enum: QUEUED, PROCESSING, COMPLETED, FAILED, CANCELLED) | 
**storage_url** | Option<**String**> |  | [optional]
**url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


