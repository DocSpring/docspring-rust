# SupportMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The message's ID. | 
**author** | [**models::SupportMessageAuthor**](SupportMessageAuthor.md) | `reporter`: your side. `team`: a person at DocSpring. `system`: DocSpring itself. | 
**body** | **String** | The message, as plain text. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the message was sent. | 
**first_read_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When one of the team's messages was first returned by the API. It shows that the message was fetched, not that a person read it. Null for your own messages. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


