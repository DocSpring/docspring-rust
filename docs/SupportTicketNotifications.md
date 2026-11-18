# SupportTicketNotifications

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | The masked address of the user the reply is emailed to: the user the API token or connected app belongs to. Null when that user was removed. | 
**watch** | [**models::SupportTicketWatch**](SupportTicketWatch.md) |  | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


