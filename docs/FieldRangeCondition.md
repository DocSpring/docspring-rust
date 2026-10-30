# FieldRangeCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field** | **String** | The name of the field whose value is compared. | 
**operator** | **String** | Must be `range`. | [default to range]
**value** | Option<**serde_json::Value**> | The lower end of the range, a number (or a number as a string, as the template editor saves it). Null or left out for no lower end. A range needs `value`, `range_max` or both. | [optional]
**range_max** | Option<**serde_json::Value**> | The upper end of the range, a number (or a number as a string, as the template editor saves it). Null or left out for no upper end. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


