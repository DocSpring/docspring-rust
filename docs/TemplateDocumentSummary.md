# TemplateDocumentSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filename** | Option<**String**> | The PDF's file name. Null until a `document_url` has been downloaded. | 
**state** | [**models::TemplateDocumentState**](TemplateDocumentState.md) | `pending` while DocSpring processes the PDF, then `processed` (`ready` says when its pages are done too). `password_required`, `malformed_pdf` and `unknown_error` mean the PDF couldn't be processed, and `error_message` says why: upload it again or upload another one. `downloading` while DocSpring downloads a `document_url`, then as for an upload. `download_failed` means it couldn't be downloaded, and `error_message` says why. A template keeps its PDF while a new one is downloaded, and when it couldn't be. | 
**error_message** | Option<**String**> | Why the PDF couldn't be processed or downloaded, and how to fix it. Null unless `state` is an error. | 
**page_count** | **i32** | 0 until the PDF has been processed. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


