# TemplateFormSection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The section's ID, unique among the template's sections. Any string you choose. | 
**title** | **String** | The section's heading on the web form. | 
**description** | Option<**String**> | Text shown under the section's heading. Null for none. | 
**field_ids** | **Vec<i32>** | The IDs of the section's fields (Field's `id`), in the order the web form shows them. A field can be in one section only. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


