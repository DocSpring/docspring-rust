# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The webhook's ID. | 
**name** | Option<**String**> | A name for the webhook, shown in the dashboard. | 
**url** | **String** | The endpoint that receives webhook POST requests. | 
**enabled** | **bool** | Whether DocSpring sends events to the webhook. Webhooks are disabled after repeated failures. | 
**event_types** | [**Vec<models::WebhookEventType>**](WebhookEventType.md) | The events the webhook subscribes to. A webhook made in the dashboard can also subscribe to API token, account integration and user events, which are listed with their own names. | 
**mode** | Option<[**models::WebhookMode**](WebhookMode.md)> | Only send events for test or live submissions. Null sends both. | 
**version** | **i32** | The format of the requests DocSpring sends. Webhooks created with API v2 use 4: each event's `data` is the resource as API v2 returns it. Webhooks made in the dashboard or with API v1 use 3, which has API v1's resources and names (`combined_submission`, `submission_data_request`). Versions 1 and 2 are legacy formats for submission events. | 
**api_token_ids** | **Vec<String>** | Only send events from these API tokens. Combined with the other scope lists using OR. Applies to submission, signature request, combined PDF, submission batch and custom file events. | 
**template_ids** | **Vec<String>** | Only send events for these templates. Combined with the other scope lists using OR. Applies to submission, signature request and template events. | 
**folder_ids** | **Vec<String>** | Only send events for templates in these folders, including their subfolders. Combined with the other scope lists using OR. Applies to submission, signature request, template and folder events. | 
**include_submission_data** | **bool** | Include the submission's field data in the payload. When false, the submission's data and truncated_text, and each signature request's data, ip_address and user_agent, are null. | 
**strip_fields** | **Vec<String>** | Dot-separated paths to remove from the event's data object. Form fields live under `data` (`data.ssn` removes the ssn field). A bare name removes a top-level key (`metadata` removes the whole metadata object). | 
**headers** | **std::collections::HashMap<String, String>** | Headers to send with each request. | 
**consecutive_failures** | **i32** | How many deliveries in a row have failed. | 
**last_failure_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When a delivery last failed. | 
**disabled_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the webhook was disabled. Null while it is enabled. | 
**disabled_reason** | Option<**String**> | Why the webhook was disabled, such as after repeated failures. | 
**secret** | Option<**String**> | The key that signs each request (the X-DocSpring-Signature header), so you can check that a request came from DocSpring. Null for some webhooks created before signing existed. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the webhook was created. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | When the webhook was last updated. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


