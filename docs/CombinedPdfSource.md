# CombinedPdfSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::CombinedPdfSourceType**](CombinedPdfSourceType.md) | What the PDF is: a submission's PDF, another combined PDF, a template's PDF, a custom file (POST /custom_files) or a PDF to download from a URL. | 
**id** | Option<**String**> | The ID of the submission, combined PDF, template or custom file. | [optional]
**url** | Option<**String**> | For a `url` source, the URL to download the PDF from. Removed once the combined PDF has expired. | [optional]
**template_version** | Option<**String**> | For a `template` source, the version to use: a published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` (the newest published version, or the draft when none has been published) or `draft`. Default `latest`. | [optional]
**processed** | Option<**bool**> | For a `template` source: true (the default) merges the template's processed PDF, which is the PDF submissions are filled in on (decrypted, with its form fields removed). false merges the PDF as it was uploaded. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


