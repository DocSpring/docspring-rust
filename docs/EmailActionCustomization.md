# EmailActionCustomization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | **bool** | Whether the account email action sends for this template. It also needs to be on itself (`enabled`). | 
**customized** | [**Vec<models::EmailActionSetting>**](EmailActionSetting.md) | The settings this template changes. The others follow the account email action. | 
**consecutive_failures** | **i32** | How many of this template's emails in a row have failed because of a setting it changes. | 
**failing_since** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the current run of those failures started. | 
**last_failure_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When one of those emails last failed. | 
**last_failure_message** | Option<**String**> | Why it failed. | 
**disabled_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When DocSpring turned the email action off for this template, such as when an address the template added bounced. | 
**disabled_reason** | Option<**String**> | Why DocSpring turned it off for this template. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the template first changed the email action. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | When the template's changes last changed. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


