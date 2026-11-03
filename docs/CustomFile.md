# CustomFile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The custom file's ID. | 
**download_url** | Option<**String**> | A temporary link to download the file. | 
**size** | Option<**i32**> | The file's size in bytes. | 
**content_type** | Option<**String**> | The file's media type, such as `application/pdf`. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the custom file was created. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | When the custom file was last updated. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


