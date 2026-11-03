# SubmissionValidation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**valid** | **bool** | True when creating the submission would succeed. | 
**template_id** | **String** | The template's ID. | 
**template_version** | Option<**String**> | The published version the request was checked against, such as `1.2.0`. Null for the draft. | 
**error** | Option<[**models::ErrorInfo**](ErrorInfo.md)> |  | [optional]
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


