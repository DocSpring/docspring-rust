# PageImage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | **i32** | The page number, from 1. | 
**width** | **i32** | The image's width in pixels. | 
**height** | **i32** | The image's height in pixels. | 
**download_url** | Option<**String**> | A temporary link to download the image. A submission's is null when your account turns off download links for submissions. | 
**permanent_url** | Option<**String**> | A link to download a submission's image that doesn't expire: it redirects to a new temporary link. It needs your API token or a signed-in session, except for a submission of a template that makes its submissions public. Null for a template's page images. | 
**filename** | **String** | The image's file name, such as `sub_123_page_1.png`. | 
**content_type** | **String** | The image's media type, `image/png` for a submission's pages or `image/jpeg` for a template's. | 
**size** | **i32** | The image's size in bytes. | 
**sha256** | Option<**String**> | The SHA-256 hash of the image. Null for a template's page images. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


