# ScheduleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alert_on_failure** | **bool** |  | 
**auto_pause_after_failures** | Option<**i32**> |  | [optional]
**consecutive_failures** | **i32** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**destinations** | Option<[**Vec<models::DeliveryDestination>**](DeliveryDestination.md)> |  | [optional]
**diff_threshold** | Option<**f64**> |  | [optional]
**ends_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**execution_count** | **i32** |  | 
**failure_count** | **i32** |  | 
**id** | **String** |  | 
**last_executed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**name** | **String** |  | 
**next_execution_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**only_on_change** | **bool** |  | 
**options** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**retention_days** | **i32** |  | 
**schedule** | **String** |  | 
**schedule_description** | **String** |  | 
**starts_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**status** | **String** |  | 
**success_count** | **i32** |  | 
**timezone** | **String** |  | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**url** | **String** |  | 
**webhook_url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


