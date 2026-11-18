# SupportHelp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The help request's ID. | 
**state** | [**models::SupportHelpState**](SupportHelpState.md) | `pending` until there's an answer, then `completed`. `failed`: the question couldn't be answered just now. | 
**answered** | Option<**bool**> | Whether the documentation had an answer. Null while `state` is `pending`. | 
**answer** | Option<**String**> | The answer, as plain text. When `answered` is false, what the documentation doesn't cover. Null while `state` is `pending`. | 
**sources** | [**Vec<models::SupportHelpSource>**](SupportHelpSource.md) | The documentation pages the answer came from. | 
**next** | **String** | What to do when the answer doesn't solve the problem, in a sentence for an AI agent to act on. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


