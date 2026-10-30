# TemplateFieldDependencyCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | The name of the field whose value is compared. | 
**operator** | [**models::TemplateFieldDependencyOperator**](TemplateFieldDependencyOperator.md) | How the field's value is compared with `value`. `in`: one of a list. `range`: from `value` to `range_max`. | 
**value** | Option<**serde_json::Value**> | The value to compare the field's value with. | [optional]
**range_max** | Option<**serde_json::Value**> | The upper end of a `range`, a number (or a number as a string, as the template editor saves it). | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


