# TemplateModelCode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lang** | [**models::ModelCodeLanguage**](ModelCodeLanguage.md) | The language of `code`. | 
**root_type_name** | **String** | The name of the type for the whole submission data object. | 
**code** | **String** | The source code of the types, starting with `root_type_name`. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


