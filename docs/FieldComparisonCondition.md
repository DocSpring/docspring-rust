# FieldComparisonCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | The name of the field whose value is compared. | 
**operator** | [**models::FieldComparisonOperator**](FieldComparisonOperator.md) | How the field's value is compared with `value`. `in`: one of a list. `range`: from `value` to `range_max`. | 
**value** | Option<**serde_json::Value**> | The number the field's value is compared with, or a number as a string (as the template editor saves it). The field must be a number field. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


