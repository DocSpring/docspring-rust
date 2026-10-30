# CreateTemplatePreviewRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | **std::collections::HashMap<String, serde_json::Value>** | The values of the template's fields, by field name, as you'd send them to create a submission. They're rendered as they are: they aren't checked against the template's fields. A value that a field can't print, such as an image URL that doesn't download, makes the preview's `state` `error`, with an `error_message`. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


