# TemplateSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The template's ID. | 
**name** | Option<**String**> | The template's name. | 
**description** | Option<**String**> | Shown under the template's name at the top of its hosted forms, and used as their meta description. Not shown on embedded forms or signature requests. | 
**template_type** | [**models::TemplateType**](TemplateType.md) | `pdf`: fields placed on an uploaded PDF. `html`: a PDF rendered from HTML and SCSS. It can't be changed after the template is created. | 
**ready** | **bool** | Whether the template can generate PDFs. HTML templates are always ready. A PDF template is ready once DocSpring has stored its PDF, made an image of each page and finished looking for fields on each page, which can be after `document.state` becomes `processed`. A PDF template with no PDF (`document` is null) isn't ready. | 
**document** | Option<[**models::TemplateDocumentSummary**](TemplateDocumentSummary.md)> | The PDF of a PDF template: its file name, page count and processing state. Null for HTML templates, and for a PDF template that has no PDF yet. | 
**parent_folder_id** | Option<**String**> | The ID of the folder the template is in. Null for a template at the root. Setting it moves the template and all of its published versions. | 
**path** | **String** | The path of the folder the template is in, such as `/Contracts`. `/` for a template at the root. | 
**latest_version** | Option<**String**> | The template's latest published version. Null if no version has been published. | 
**version_published_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When this version was published. Null for the draft. | 
**locked** | **bool** | A locked template can't be changed or deleted until it's unlocked (`locked` can always be changed). | 
**last_changed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the draft was last changed. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the template was created. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | When the template was last updated. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


