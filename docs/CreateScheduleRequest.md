# CreateScheduleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alert_on_failure** | Option<**bool**> |  | [optional][default to false]
**auto_pause_after_failures** | Option<**i32**> |  | [optional]
**destinations** | Option<[**Vec<models::DeliveryDestination>**](DeliveryDestination.md)> |  | [optional]
**diff_threshold** | Option<**f64**> |  | [optional]
**ends_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**name** | **String** |  | 
**only_on_change** | Option<**bool**> |  | [optional][default to false]
**options** | Option<[**models::ScheduleScreenshotOptions**](ScheduleScreenshotOptions.md)> |  | [optional]
**retention_days** | Option<**i32**> |  | [optional][default to 30]
**schedule** | **String** |  | 
**starts_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**timezone** | Option<**String**> |  | [optional][default to UTC]
**url** | **String** |  | 
**webhook_secret** | Option<**String**> |  | [optional]
**webhook_url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


