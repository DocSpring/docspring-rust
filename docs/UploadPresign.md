# UploadPresign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The S3 URL to upload the file to. | 
**method** | [**models::UploadMethod**](UploadMethod.md) | The HTTP method for the upload. | 
**fields** | **std::collections::HashMap<String, String>** | Form fields to send with the file. Send them all, as they are. `key` is the uploaded file's ID: pass it as `cache_id`. | 
**headers** | **std::collections::HashMap<String, String>** | Headers to send with the upload. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


