# TemplatePageImages

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | 
**version** | Option<**String**> | The published version the images are of, such as `1.2.0`. Null for the draft. | 
**page_count** | **i32** | How many pages the PDF has. | 
**pages** | [**Vec<models::PageImage>**](PageImage.md) | An image of each page, in order: `pages[0]` is page 1. Their `download_url`s expire, so get them again when you need them. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


