# CrawlRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_ads** | Option<**bool**> |  | [optional][default to true]
**block_cookie_banners** | Option<**bool**> |  | [optional][default to true]
**block_popups** | Option<**bool**> |  | [optional][default to true]
**crawl_delay_ms** | Option<**i32**> |  | [optional][default to 500]
**dark_mode** | Option<**bool**> |  | [optional][default to false]
**depth** | Option<**i32**> |  | [optional][default to 2]
**exclude_patterns** | Option<**Vec<String>**> |  | [optional]
**format** | Option<**String**> |  | [optional][default to png]
**full_page** | Option<**bool**> |  | [optional][default to true]
**include_patterns** | Option<**Vec<String>**> |  | [optional]
**include_subdomains** | Option<**bool**> |  | [optional][default to false]
**limit** | Option<**i32**> |  | [optional][default to 25]
**outputs** | Option<[**HashSet<models::OutputSpec>**](OutputSpec.md)> |  | [optional]
**quality** | Option<**i32**> |  | [optional][default to 80]
**render_delay** | Option<**i32**> |  | [optional][default to 0]
**timeout** | Option<**i32**> |  | [optional][default to 30000]
**url** | **String** |  | 
**viewport** | Option<[**models::ViewportConfig**](ViewportConfig.md)> |  | [optional]
**wait_until** | Option<**String**> |  | [optional][default to domcontentloaded]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


