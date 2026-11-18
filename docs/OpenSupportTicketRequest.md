# OpenSupportTicketRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | One line, written like an issue title. | 
**kind** | [**models::SupportReportKind**](SupportReportKind.md) | What kind of problem this is. | 
**category** | Option<[**models::SupportReportCategory**](SupportReportCategory.md)> | The part of DocSpring this is about. | [optional]
**description** | **String** | What was tried, what was expected and what happened. | 
**impact** | Option<[**models::SupportReportImpact**](SupportReportImpact.md)> | `blocked`: the task could not be done. `worked_around`: it was done another way. `friction`: it was only slower. | [optional]
**task** | Option<**String**> | What the user asked for, in a short phrase. | [optional]
**request_ids** | Option<**Vec<String>**> | The request IDs of the requests that went wrong: `error.request_id` in an error response, or any response's `X-Request-Id` header. | [optional]
**tool** | Option<**String**> | The API operation, MCP tool or CLI command involved. | [optional]
**error** | Option<**String**> | The exact error text. | [optional]
**suggestion** | Option<**String**> | What would have helped. | [optional]
**source** | Option<[**models::SupportReportSource**](SupportReportSource.md)> | `user`: the user asked for this to be sent. `agent` (the default): an AI agent noticed the problem itself. | [optional]
**session** | Option<**String**> | A label the sender chooses for its work, such as a session ID, a branch or a task name. When an account runs several agents at once, each can list only the tickets opened with its own label. | [optional]
**agent** | Option<[**models::SupportReportAgent**](SupportReportAgent.md)> | The AI agent sending this, as it describes itself. Stored as unverified. | [optional]

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


