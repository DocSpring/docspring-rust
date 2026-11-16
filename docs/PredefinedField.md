# PredefinedField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The field's name, the key of its value in submission data. A `/` nests the key in objects and arrays (`customer/email`, `signers/0/name`). | 
**display_name** | Option<**String**> | The name the template editor shows instead of `name`, where a `/` puts it in a folder, such as `Defendant/Name`. Null shows `name`. Two names can't share a display name. | [optional]
**title** | Option<**String**> | The Title setting a field gets when it's given this name. Null leaves the field's title as it is. | [optional]
**r#type** | [**models::FieldType**](FieldType.md) | The data type a field gets when it's given this name, as a field's `type`. | 
**option_list** | Option<**String**> | String fields only: the values allowed, separated by commas, such as `Yes,No`. Null allows any value. | [optional]
**option_labels** | Option<**std::collections::HashMap<String, String>**> | The labels forms show for `option_list`'s values, by value, as a field's `option_labels`, such as `{"married_filing_jointly": "Married filing jointly"}`. Each key must be one of `option_list`'s values. A value without a label shows the value, or its sentence case when the values are lowercase words joined by underscores. Null gives no value a label of its own. | [optional]
**required_in_template** | Option<**bool**> | Whether the template must have a field with this name. The embedded editor shows the names that are missing, and with `disableDoneButtonIfPredefinedFieldsMissing` keeps its Done button disabled until the template has them. Default false. | [optional]
**field_required** | Option<**bool**> | The Required setting a field gets when it's given this name. Default true. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


