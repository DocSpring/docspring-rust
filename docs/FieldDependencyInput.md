# FieldDependencyInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | [**models::TemplateFieldDependencyAction**](TemplateFieldDependencyAction.md) | What the rule does to the fields it applies to when its conditions are met. | 
**r#match** | Option<[**models::TemplateFieldDependencyMatch**](TemplateFieldDependencyMatch.md)> | Whether all of the conditions (the default) or any of them must be met. | [optional]
**conditions** | [**Vec<models::TemplateFieldDependencyConditionInput>**](TemplateFieldDependencyConditionInput.md) | The conditions on other fields' values. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


