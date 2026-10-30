# TemplateDiff

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | 
**from_version** | Option<**String**> | The earlier version, such as `1.1.0`. Null when no version was published before `to_version`: it's compared with a new template, so all of its fields are added. | 
**to_version** | Option<**String**> | The later version, such as `1.2.0`. Null for the draft. | 
**summary** | [**models::TemplateDiffSummary**](TemplateDiffSummary.md) | How many changes of each kind there are. | 
**fields** | [**models::TemplateFieldChanges**](TemplateFieldChanges.md) | The fields that changed. | 
**changed_settings** | **Vec<String>** | The names of the template's settings that changed, as they're named on the Template (`name`, `html`, `defaults`, `merge_audit_trail_pdf`, ...). Values aren't included. Settings that apply to every version, such as `webhook_url` and `encrypt_pdfs_password`, are the same on every version, so they're named only when there's no earlier version to compare with; a secret is named without its value. | 
**document_changed** | **bool** | The template's PDF was replaced (or added, when there's no `from_version`). | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


