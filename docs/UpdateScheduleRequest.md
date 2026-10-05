# UpdateScheduleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alert_on_failure** | Option<**bool**> |  | [optional]
**auto_pause_after_failures** | Option<**i32**> |  | [optional]
**destinations** | Option<[**Vec<models::DeliveryDestination>**](DeliveryDestination.md)> |  | [optional]
**diff_threshold** | Option<**f64**> |  | [optional]
**ends_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**only_on_change** | Option<**bool**> |  | [optional]
**options** | Option<[**models::ScheduleScreenshotOptions**](ScheduleScreenshotOptions.md)> |  | [optional]
**retention_days** | Option<**i32**> |  | [optional]
**schedule** | Option<**String**> |  | [optional]
**starts_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**timezone** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**webhook_secret** | Option<**String**> |  | [optional]
**webhook_url** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


