# SupportTicketWatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The ticket's `watch_url`. | 
**min_interval_seconds** | **i32** | Poll no more often than this. | 
**how** | **String** | What to do with the URL, in a sentence for an AI agent to act on. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


