# SignatureRequestToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The token's ID. | 
**signature_request_id** | **String** | The ID of the signature request the token opens. | 
**r#type** | [**models::SignatureRequestTokenType**](SignatureRequestTokenType.md) | `api`: the link opens the signature request for 24 hours. `email`: 7 days. | 
**secret** | **String** | The token's secret. It's only returned when the token is created. | 
**signature_request_url** | **String** | A link that opens the signature request with this token. | 
**expires_at** | **chrono::DateTime<chrono::FixedOffset>** | When the link stops opening the signature request. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the token was created. | 
**additional_properties** | **serde_json::Map<String, serde_json::Value>** | Properties the schema doesn't list (it allows additional properties), in the order they came (with serde_json's `preserve_order` feature) or sorted by name. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


