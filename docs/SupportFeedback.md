# SupportFeedback

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The feedback's ID. | 
**received_at** | **chrono::DateTime<chrono::FixedOffset>** | When DocSpring received it. | 
**follow_up** | **String** | What happens next, in a sentence for an AI agent to act on. Nobody replies to feedback. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


