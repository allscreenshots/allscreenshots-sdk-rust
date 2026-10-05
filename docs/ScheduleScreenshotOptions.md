# ScheduleScreenshotOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | Option<[**Vec<models::PageAction>**](PageAction.md)> |  | [optional]
**block_ads** | Option<**bool**> |  | [optional][default to true]
**block_cookie_banners** | Option<**bool**> |  | [optional][default to true]
**block_level** | Option<**String**> |  | [optional][default to none]
**block_popups** | Option<**bool**> |  | [optional][default to true]
**custom_css** | Option<**String**> |  | [optional]
**dark_mode** | Option<**bool**> |  | [optional][default to false]
**delay** | Option<**i32**> |  | [optional][default to 0]
**device** | Option<**String**> |  | [optional]
**format** | Option<**String**> |  | [optional][default to png]
**freeze_fixed** | Option<**bool**> |  | [optional][default to true]
**full_page** | Option<**bool**> |  | [optional][default to false]
**full_page_mode** | Option<**String**> |  | [optional][default to stitch]
**hide_selectors** | Option<**Vec<String>**> |  | [optional]
**max_height** | Option<**i32**> |  | [optional]
**max_sections** | Option<**i32**> |  | [optional][default to 50]
**outputs** | Option<[**Vec<models::OutputSpec>**](OutputSpec.md)> |  | [optional]
**quality** | Option<**i32**> |  | [optional][default to 80]
**scroll_interval** | Option<**i32**> |  | [optional][default to 150]
**selector** | Option<**String**> |  | [optional]
**stealth_mode** | Option<**bool**> |  | [optional][default to false]
**timeout** | Option<**i32**> |  | [optional][default to 30000]
**viewport** | Option<[**models::ViewportConfig**](ViewportConfig.md)> |  | [optional]
**wait_for** | Option<**String**> |  | [optional]
**wait_until** | Option<**String**> |  | [optional][default to domcontentloaded]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


