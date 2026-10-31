# TemplateSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The template's ID. | 
**name** | Option<**String**> | The template's name. | 
**description** | Option<**String**> | Shown under the template's name at the top of its hosted forms, and used as their meta description. Not shown on embedded forms or signature requests. | 
**template_type** | [**models::TemplateType**](TemplateType.md) | `pdf`: fields placed on an uploaded PDF. `html`: a PDF rendered from HTML and SCSS. `docx`: a Word document with tags where the data goes, which makes Word documents or PDFs. It can't be changed after the template is created. | 
**output_format** | [**models::OutputFormat**](OutputFormat.md) | What each submission makes unless its request sets `output_formats`: `docx` (a Word document), `odt` (an OpenDocument file, converted from the Word document) or `pdf`. A Word template's is chosen when it's created; PDF and HTML templates always make PDFs. A Word template that encrypts its PDFs makes PDFs, because Word documents and OpenDocument files can't be password-protected. | 
**ready** | **bool** | Whether the template can make documents. HTML templates are always ready. A PDF template is ready once DocSpring has stored its PDF, made an image of each page and finished looking for fields on each page, which can be after `document.state` becomes `processed`. A PDF template with no PDF (`document` is null) isn't ready. A Word template is ready once it has a Word document (a `document_url`'s has been downloaded). | 
**document** | Option<[**models::TemplateDocumentSummary**](TemplateDocumentSummary.md)> | The PDF of a PDF template, or the Word document of a Word template: its file name, page count and processing state. Null for HTML templates, and for a PDF template that has no PDF yet. A template created from a `document_url`, or a Word template created from an OpenDocument file (.odt), has one, with no file, while it's downloaded or converted. | 
**parent_folder_id** | Option<**String**> | The ID of the folder the template is in. Null for a template at the root. Setting it moves the template and all of its published versions. | 
**path** | **String** | The path of the folder the template is in, such as `/Contracts`. `/` for a template at the root. | 
**latest_version** | Option<**String**> | The template's latest published version. Null if no version has been published. | 
**version_published_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When this version was published. Null for the draft. | 
**locked** | **bool** | A locked template can't be changed or deleted until it's unlocked (`locked` can always be changed). | 
**last_changed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the draft was last changed. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the template was created. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | When the template was last updated. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


