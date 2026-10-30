# SignatureRequestEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The event's ID. | 
**submission_id** | **String** | The ID of the signature request's submission. | 
**signature_request_id** | **String** | The ID of the signature request. | 
**event_type** | [**models::SignatureRequestEventType**](SignatureRequestEventType.md) | What happened. | 
**message_type** | Option<[**models::SignatureRequestMessageType**](SignatureRequestMessageType.md)> | How you sent the request. Required for `send_request`. | 
**message_recipient** | Option<**String**> | Where you sent the request, such as an email address or a phone number. Required for `send_request`. | 
**occurred_at** | **chrono::DateTime<chrono::FixedOffset>** | When it happened. Default now. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the event was recorded. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


