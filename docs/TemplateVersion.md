# TemplateVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **String** | The version number, such as `1.2.0`. | 
**description** | Option<**String**> | What changed in the version, as written when it was published. | 
**published_at** | **chrono::DateTime<chrono::FixedOffset>** | When the version was published. | 
**published_by** | Option<[**models::TemplateActor**](TemplateActor.md)> | Who published the version. Null when that wasn't recorded. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


