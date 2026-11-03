# TemplateFieldDependency

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | [**models::TemplateFieldDependencyAction**](TemplateFieldDependencyAction.md) | What the rule does to the fields it applies to when its conditions are met. | 
**r#match** | Option<[**models::TemplateFieldDependencyMatch**](TemplateFieldDependencyMatch.md)> | Whether all of the conditions (the default) or any of them must be met. | [optional]
**conditions** | [**Vec<models::TemplateFieldDependencyCondition>**](TemplateFieldDependencyCondition.md) | The conditions on other fields' values. | 
**targets** | **Vec<String>** | The names of the fields the rule shows or makes required. | 
**group** | Option<**String**> | Marks a rule made for a field group in the template editor. Its targets outside the group are removed when the template is saved, and the rule is removed once the group has no fields. | [optional]
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


