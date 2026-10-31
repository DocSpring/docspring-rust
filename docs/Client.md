# Api

All URIs are relative to *https://sync.api.docspring.com/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_template_fields**](Client.md#add_template_fields) | **POST** /templates/{template_id}/fields | Add fields to a template
[**copy_template**](Client.md#copy_template) | **POST** /templates/{template_id}/copy | Copy a template
[**create_combined_pdf**](Client.md#create_combined_pdf) | **POST** /combined_pdfs | Combine PDFs
[**create_custom_file_from_upload**](Client.md#create_custom_file_from_upload) | **POST** /custom_files | Create a custom file from an upload
[**create_folder**](Client.md#create_folder) | **POST** /folders | Create a folder
[**create_signature_request_event**](Client.md#create_signature_request_event) | **POST** /signature_requests/{signature_request_id}/events | Record a signature request event
[**create_signature_request_token**](Client.md#create_signature_request_token) | **POST** /signature_requests/{signature_request_id}/tokens | Create a signature request token
[**create_submission**](Client.md#create_submission) | **POST** /templates/{template_id}/submissions | Generate a PDF
[**create_submission_batch**](Client.md#create_submission_batch) | **POST** /submissions/batches | Generate PDFs in a batch
[**create_template**](Client.md#create_template) | **POST** /templates | Create a template
[**create_template_preview**](Client.md#create_template_preview) | **POST** /templates/{template_id}/previews | Preview a template with data
[**create_webhook**](Client.md#create_webhook) | **POST** /webhooks | Create a webhook
[**delete_custom_file**](Client.md#delete_custom_file) | **DELETE** /custom_files/{custom_file_id} | Delete a custom file
[**delete_folder**](Client.md#delete_folder) | **DELETE** /folders/{folder_id} | Delete a folder
[**delete_template**](Client.md#delete_template) | **DELETE** /templates/{template_id} | Delete a template
[**delete_webhook**](Client.md#delete_webhook) | **DELETE** /webhooks/{webhook_id} | Delete a webhook
[**expire_combined_pdf**](Client.md#expire_combined_pdf) | **DELETE** /combined_pdfs/{combined_pdf_id} | Expire a combined PDF
[**expire_submission**](Client.md#expire_submission) | **DELETE** /submissions/{submission_id} | Expire a submission
[**generate_submission_files**](Client.md#generate_submission_files) | **POST** /submissions/{submission_id}/generate_files | Generate more files
[**generate_submission_preview**](Client.md#generate_submission_preview) | **POST** /submissions/{submission_id}/generate_preview | Generate a preview PDF
[**get_combined_pdf**](Client.md#get_combined_pdf) | **GET** /combined_pdfs/{combined_pdf_id} | Get a combined PDF
[**get_custom_file**](Client.md#get_custom_file) | **GET** /custom_files/{custom_file_id} | Get a custom file
[**get_folder**](Client.md#get_folder) | **GET** /folders/{folder_id} | Get a folder
[**get_presign_url**](Client.md#get_presign_url) | **GET** /uploads/presign | Get a presigned URL to upload a file
[**get_signature_request**](Client.md#get_signature_request) | **GET** /signature_requests/{signature_request_id} | Get a signature request
[**get_submission**](Client.md#get_submission) | **GET** /submissions/{submission_id} | Get a submission
[**get_submission_batch**](Client.md#get_submission_batch) | **GET** /submissions/batches/{submission_batch_id} | Get a batch
[**get_template**](Client.md#get_template) | **GET** /templates/{template_id} | Get a template
[**get_template_diff**](Client.md#get_template_diff) | **GET** /templates/{template_id}/diff | Compare a version of a template with the one before it
[**get_template_model_code**](Client.md#get_template_model_code) | **GET** /templates/{template_id}/model_code | Generate typed code for a template's submission data
[**get_template_page_images**](Client.md#get_template_page_images) | **GET** /templates/{template_id}/page_images | Get images of a template's pages
[**get_template_preview**](Client.md#get_template_preview) | **GET** /templates/{template_id}/previews/{preview_id} | Get a template preview
[**get_template_schema**](Client.md#get_template_schema) | **GET** /templates/{template_id}/schema | Get the JSON Schema of a template's submission data
[**get_webhook**](Client.md#get_webhook) | **GET** /webhooks/{webhook_id} | Get a webhook
[**list_combined_pdfs**](Client.md#list_combined_pdfs) | **GET** /combined_pdfs | List combined PDFs
[**list_custom_files**](Client.md#list_custom_files) | **GET** /custom_files | List custom files
[**list_folders**](Client.md#list_folders) | **GET** /folders | List folders
[**list_submissions**](Client.md#list_submissions) | **GET** /submissions | List submissions
[**list_template_fields**](Client.md#list_template_fields) | **GET** /templates/{template_id}/fields | List a template's fields
[**list_template_submissions**](Client.md#list_template_submissions) | **GET** /templates/{template_id}/submissions | List a template's submissions
[**list_templates**](Client.md#list_templates) | **GET** /templates | List templates
[**list_webhooks**](Client.md#list_webhooks) | **GET** /webhooks | List webhooks
[**move_folder**](Client.md#move_folder) | **POST** /folders/{folder_id}/move | Move a folder
[**move_template**](Client.md#move_template) | **POST** /templates/{template_id}/move | Move a template to a folder
[**publish_template_version**](Client.md#publish_template_version) | **POST** /templates/{template_id}/publish_version | Publish a template version
[**remove_template_fields**](Client.md#remove_template_fields) | **POST** /templates/{template_id}/fields/remove | Remove fields from a template
[**rename_folder**](Client.md#rename_folder) | **POST** /folders/{folder_id}/rename | Rename a folder
[**reorder_template_fields**](Client.md#reorder_template_fields) | **POST** /templates/{template_id}/fields/reorder | Reorder a template's fields
[**restore_template_version**](Client.md#restore_template_version) | **POST** /templates/{template_id}/restore_version | Restore a template version
[**test_authentication**](Client.md#test_authentication) | **GET** /authentication | Test authentication
[**update_signature_request**](Client.md#update_signature_request) | **PUT** /signature_requests/{signature_request_id} | Update a signature request
[**update_template**](Client.md#update_template) | **PUT** /templates/{template_id} | Update a template
[**update_template_document**](Client.md#update_template_document) | **PUT** /templates/{template_id}/document | Replace a template's document
[**update_template_fields**](Client.md#update_template_fields) | **PUT** /templates/{template_id}/fields | Update a template's fields
[**update_webhook**](Client.md#update_webhook) | **PUT** /webhooks/{webhook_id} | Update a webhook
[**upload_template**](Client.md#upload_template) | **POST** /templates/upload | Create a template from an uploaded PDF or Word document
[**upload_template_document**](Client.md#upload_template_document) | **PUT** /templates/{template_id}/document/upload | Replace a template's document with an uploaded file
[**validate_submission**](Client.md#validate_submission) | **POST** /templates/{template_id}/submissions/validate | Check a submission without creating it



## add_template_fields

> async fn add_template_fields(&self, params: AddTemplateFieldsParams) -> Result<models::TemplateFields, Error<AddTemplateFieldsError>>
Add fields to a template

Adds fields to a template's draft and returns all of its fields. Publish a version for submissions of published versions to have them. 

### Parameters

Set them on `AddTemplateFieldsParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**AddTemplateFieldsRequest**](AddTemplateFieldsRequest.md) | The fields to add to the template's draft. | [required] |

### Return type

[**models::TemplateFields**](TemplateFields.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## copy_template

> async fn copy_template(&self, params: CopyTemplateParams) -> Result<models::Template, Error<CopyTemplateError>>
Copy a template

Copies a template (its draft, or a published version) into a new template.

### Parameters

Set them on `CopyTemplateParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | Option<[**CopyTemplateRequest**](CopyTemplateRequest.md)> | Settings for the copy. |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_combined_pdf

> async fn create_combined_pdf(&self, params: CreateCombinedPdfParams) -> Result<models::CombinedPdf, Error<CreateCombinedPdfError>>
Combine PDFs

Merges PDFs into one: submissions, other combined PDFs, templates, custom files (POST /custom_files) and PDFs from URLs, in the order of `source_pdfs`. The PDF is generated in the background: on sync.api hosts the request waits for it (unless you pass `wait=false`), otherwise poll GET /combined_pdfs/{combined_pdf_id} until `state` isn't `pending`.  A template source merges the template's processed PDF by default: the PDF submissions are filled in on, decrypted and with its form fields removed. Pass `processed: false` to merge the PDF as it was uploaded. 

### Parameters

Set them on `CreateCombinedPdfParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateCombinedPdfRequest**](CreateCombinedPdfRequest.md) | The PDFs to merge, and the combined PDF's settings. | [required] |
**wait** | Option<**bool**> | Wait until the PDF has been generated before responding. Default true on sync.api hosts. |  |
**idempotency_key** | Option<**String**> | A key of your own, such as a UUID, that makes it safe to send this request again: for 24 hours, DocSpring answers a request with the same key from the same API token with the first request's response (and an `Idempotent-Replayed: true` header) instead of creating another combined PDF. Send a new key with each new combined PDF, and the same key each time you send the same request again, such as after a timeout, a network error or a 5xx. The same key with a different request is a 422 `idempotency_key_reused`. A 409 `idempotency_key_in_use` means the first request is still being processed: send the request again in a few seconds. |  |

### Return type

[**models::CombinedPdf**](CombinedPdf.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_custom_file_from_upload

> async fn create_custom_file_from_upload(&self, params: CreateCustomFileFromUploadParams) -> Result<models::CustomFile, Error<CreateCustomFileFromUploadError>>
Create a custom file from an upload

Creates a custom file from a PDF you uploaded, so you can merge it with other PDFs. Upload the file with the presigned upload (GET /uploads/presign) first, then pass its `fields.key` as `cache_id`. 

### Parameters

Set them on `CreateCustomFileFromUploadParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateCustomFileFromUploadRequest**](CreateCustomFileFromUploadRequest.md) | The uploaded file to create the custom file from. | [required] |

### Return type

[**models::CustomFile**](CustomFile.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_folder

> async fn create_folder(&self, params: CreateFolderParams) -> Result<models::Folder, Error<CreateFolderError>>
Create a folder

Creates a folder for organizing templates. Pass `parent_folder_id` to create it inside another folder. Folder names are unique within their parent folder. 

### Parameters

Set them on `CreateFolderParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateFolderRequest**](CreateFolderRequest.md) | The folder to create. | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_signature_request_event

> async fn create_signature_request_event(&self, params: CreateSignatureRequestEventParams) -> Result<models::SignatureRequestEvent, Error<CreateSignatureRequestEventError>>
Record a signature request event

Records something that happened outside DocSpring in the signature request's audit trail: you sent the request to the person (`send_request`). The audit trail shows it as reported by your account. A finished (completed, declined or expired) signature request's audit trail takes no more events, and a test API token can't record one for a live signature request. 

### Parameters

Set them on `CreateSignatureRequestEventParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signature_request_id** | **String** | The signature request's ID. | [required] |
**body** | [**CreateSignatureRequestEventRequest**](CreateSignatureRequestEventRequest.md) | The event to record. | [required] |

### Return type

[**models::SignatureRequestEvent**](SignatureRequestEvent.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_signature_request_token

> async fn create_signature_request_token(&self, params: CreateSignatureRequestTokenParams) -> Result<models::SignatureRequestToken, Error<CreateSignatureRequestTokenError>>
Create a signature request token

Creates a token that opens the signature request without signing in, in a form embedded in your app or from a link you send. The response is the only time the token's secret is returned. The link works for 24 hours, or 7 days for an `email` token, and a signing page that opened in time keeps working for 7 more days. A test API token can't create a token for a live signature request. 

### Parameters

Set them on `CreateSignatureRequestTokenParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signature_request_id** | **String** | The signature request's ID. | [required] |
**body** | Option<[**CreateSignatureRequestTokenRequest**](CreateSignatureRequestTokenRequest.md)> | The kind of link to make. Send no body for an `api` token. |  |

### Return type

[**models::SignatureRequestToken**](SignatureRequestToken.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_submission

> async fn create_submission(&self, params: CreateSubmissionParams) -> Result<models::Submission, Error<CreateSubmissionError>>
Generate a PDF

Creates a submission: fills in a template with your data and generates a PDF. The PDF is generated in the background: on sync.api hosts the request waits for it (unless you pass `wait=false`), otherwise poll GET /submissions/{submission_id} until `state` isn't `pending`.  Data that doesn't match the template's fields returns a 422 with an `invalid_data` error. The submission is still saved, with its errors, so you can find it in DocSpring: the error's `submission_id` is its ID. 

### Parameters

Set them on `CreateSubmissionParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**CreateSubmissionRequest**](CreateSubmissionRequest.md) | The submission's data and settings. | [required] |
**version** | Option<**String**> | The template version to use: a published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `latest` (the draft when no version has been published). |  |
**wait** | Option<**bool**> | Wait until the files have been generated before responding. Default true on sync.api hosts. |  |
**idempotency_key** | Option<**String**> | A key of your own, such as a UUID, that makes it safe to send this request again: for 24 hours, DocSpring answers a request with the same key from the same API token with the first request's response (and an `Idempotent-Replayed: true` header) instead of creating another submission. Send a new key with each new submission, and the same key each time you send the same request again, such as after a timeout, a network error or a 5xx. The same key with a different request is a 422 `idempotency_key_reused`. A 409 `idempotency_key_in_use` means the first request is still being processed: send the request again in a few seconds. |  |

### Return type

[**models::Submission**](Submission.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_submission_batch

> async fn create_submission_batch(&self, params: CreateSubmissionBatchParams) -> Result<models::SubmissionBatch, Error<CreateSubmissionBatchError>>
Generate PDFs in a batch

Creates up to 50 submissions in one request, from one template or several. The PDFs are only generated when every submission is valid. On sync.api hosts the request waits until every PDF has been generated (unless you pass `wait=false`), otherwise poll GET /submissions/batches/{submission_batch_id} until `processed_at` is set.  When any submission is invalid, nothing is generated and the request returns a 422. Submissions whose data doesn't match their template's fields are still saved with their errors (an `invalid_data` error, with each one's `submission_id` in its details), so you can find them in DocSpring. The other submissions aren't saved. 

### Parameters

Set them on `CreateSubmissionBatchParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateSubmissionBatchRequest**](CreateSubmissionBatchRequest.md) | The submissions to create, and the settings they share. | [required] |
**wait** | Option<**bool**> | Wait until every file in the batch has been generated before responding. Default true on sync.api hosts. |  |
**idempotency_key** | Option<**String**> | A key of your own, such as a UUID, that makes it safe to send this request again: for 24 hours, DocSpring answers a request with the same key from the same API token with the first request's response (and an `Idempotent-Replayed: true` header) instead of creating another batch. Send a new key with each new batch, and the same key each time you send the same request again, such as after a timeout, a network error or a 5xx. The same key with a different request is a 422 `idempotency_key_reused`. A 409 `idempotency_key_in_use` means the first request is still being processed: send the request again in a few seconds. |  |

### Return type

[**models::SubmissionBatch**](SubmissionBatch.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_template

> async fn create_template(&self, params: CreateTemplateParams) -> Result<models::Template, Error<CreateTemplateError>>
Create a template

Creates an HTML template, or a PDF or Word template from a file you uploaded with the presigned upload (GET /uploads/presign): pass its `fields.key` as `cache_id`. You can send a link to the file (`document_url`) or the file in base64 (`document_base64`) instead, or send the file with the request with POST /templates/upload. A Word template is made from a Word document (.docx) or an OpenDocument text file (.odt), which DocSpring converts to a Word document. It needs its `output_format`: what each submission makes, a Word document (`docx`), an OpenDocument file (`odt`) or a PDF.  A PDF template is ready to generate PDFs once DocSpring has processed its PDF, a template created from a `document_url` once its document has been downloaded, and one created from an .odt once it has been converted (`ready` is true). On sync.api hosts the request waits for that unless you pass `wait=false`. 

### Parameters

Set them on `CreateTemplateParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateTemplateRequest**](CreateTemplateRequest.md) | The template to create. | [required] |
**wait** | Option<**bool**> | Wait until the PDF has been processed, a `document_url`'s PDF or Word document downloaded, and an OpenDocument file converted, before responding. Default true on sync.api hosts. |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_template_preview

> async fn create_template_preview(&self, params: CreateTemplatePreviewParams) -> Result<models::TemplatePreview, Error<CreateTemplatePreviewError>>
Preview a template with data

Makes PNG images of a template's pages filled in with the data you send (not a PDF), so you can check how an HTML, PDF or Word template prints with your data before you publish a version or send live data. A Word template's pages are its PDF's, whatever its `output_format`. It renders the template's draft, or the published `version` you ask for. A preview makes no submission and uses no credits, and its pages are watermarked like a test submission's PDF. It's `pending` while it renders: get it with GET /templates/{template_id}/previews/{preview_id} until it's `processed`, with an image of each page, or `error`. 

### Parameters

Set them on `CreateTemplatePreviewParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**CreateTemplatePreviewRequest**](CreateTemplatePreviewRequest.md) | The data to render the template with. | [required] |
**version** | Option<**String**> | A published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `draft`. |  |

### Return type

[**models::TemplatePreview**](TemplatePreview.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## create_webhook

> async fn create_webhook(&self, params: CreateWebhookParams) -> Result<models::Webhook, Error<CreateWebhookError>>
Create a webhook

Creates a webhook that sends a POST request to your endpoint whenever a subscribed event happens. Scope it to API tokens, templates or folders to receive only their events. 

### Parameters

Set them on `CreateWebhookParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateWebhookRequest**](CreateWebhookRequest.md) | The webhook to create. | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## delete_custom_file

> async fn delete_custom_file(&self, params: DeleteCustomFileParams) -> Result<models::CustomFile, Error<DeleteCustomFileError>>
Delete a custom file

Deletes a custom file and returns it. Use a live API token: test tokens can't delete custom files. 

### Parameters

Set them on `DeleteCustomFileParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_file_id** | **String** | The custom file's ID. | [required] |

### Return type

[**models::CustomFile**](CustomFile.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## delete_folder

> async fn delete_folder(&self, params: DeleteFolderParams) -> Result<models::Folder, Error<DeleteFolderError>>
Delete a folder

Deletes an empty folder and returns it. A folder that still has templates or folders in it can't be deleted. 

### Parameters

Set them on `DeleteFolderParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The folder's ID. | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## delete_template

> async fn delete_template(&self, params: DeleteTemplateParams) -> Result<models::Template, Error<DeleteTemplateError>>
Delete a template

Deletes a template's draft and all of its published versions, or one published version (`version`). Returns the template as it was before it was deleted. 

### Parameters

Set them on `DeleteTemplateParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**version** | Option<**String**> | Delete only this published version (`1.2.0`) or the draft (`draft`, when no version has been published). Without it, the draft and every published version are deleted. |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## delete_webhook

> async fn delete_webhook(&self, params: DeleteWebhookParams) -> Result<models::Webhook, Error<DeleteWebhookError>>
Delete a webhook

Deletes a webhook so it stops receiving events, and returns it.

### Parameters

Set them on `DeleteWebhookParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The webhook's ID. | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## expire_combined_pdf

> async fn expire_combined_pdf(&self, params: ExpireCombinedPdfParams) -> Result<models::CombinedPdf, Error<ExpireCombinedPdfError>>
Expire a combined PDF

Deletes a combined PDF's PDF and returns the expired combined PDF.

### Parameters

Set them on `ExpireCombinedPdfParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**combined_pdf_id** | **String** | The combined PDF's ID. | [required] |

### Return type

[**models::CombinedPdf**](CombinedPdf.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## expire_submission

> async fn expire_submission(&self, params: ExpireSubmissionParams) -> Result<models::Submission, Error<ExpireSubmissionError>>
Expire a submission

Deletes a submission's data and PDF now, and returns the expired submission.

### Parameters

Set them on `ExpireSubmissionParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** | The submission's ID. | [required] |

### Return type

[**models::Submission**](Submission.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## generate_submission_files

> async fn generate_submission_files(&self, params: GenerateSubmissionFilesParams) -> Result<models::Submission, Error<GenerateSubmissionFilesError>>
Generate more files

Adds formats to a processed submission's files, in the background, without using credits again: `pdf`, the PDF of a Word template's submission that made only its Word document (converted from that document), `docx`, the Word document of a Word template's submission that made only its PDF (made again from its data and the template version it used), `odt`, a Word template's submission's OpenDocument file (converted from its Word document, or from one made again when it didn't keep one), and `png`, a PNG image of each page of the PDF (up to 100 pages), which makes the PDF first when the submission doesn't have one. To make them with the submission, add them to `output_formats` when you create it.  Each new file is listed in `files` while it's made: on sync.api hosts the request waits for them (unless you pass `wait=false`), otherwise poll GET /submissions/{submission_id} until none of their `state` is `pending`. A file that couldn't be made has the state `error` and an `error_message`, and the submission stays processed. Asking for a format the submission already has (or is making) changes nothing. 

### Parameters

Set them on `GenerateSubmissionFilesParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** | The submission's ID. | [required] |
**body** | [**GenerateSubmissionFilesRequest**](GenerateSubmissionFilesRequest.md) | The formats to make. | [required] |
**wait** | Option<**bool**> | Wait until the files have been generated before responding. Default true on sync.api hosts. |  |

### Return type

[**models::Submission**](Submission.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## generate_submission_preview

> async fn generate_submission_preview(&self, params: GenerateSubmissionPreviewParams) -> Result<models::Submission, Error<GenerateSubmissionPreviewError>>
Generate a preview PDF

Generates a preview of a submission's PDF in the background, such as for a submission still waiting for its signature requests. It's in `files.preview` once `preview_generated_at` changes. 

### Parameters

Set them on `GenerateSubmissionPreviewParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** | The submission's ID. | [required] |

### Return type

[**models::Submission**](Submission.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_combined_pdf

> async fn get_combined_pdf(&self, params: GetCombinedPdfParams) -> Result<models::CombinedPdf, Error<GetCombinedPdfError>>
Get a combined PDF

Returns a combined PDF, with a temporary link to download it once it has been generated.

### Parameters

Set them on `GetCombinedPdfParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**combined_pdf_id** | **String** | The combined PDF's ID. | [required] |

### Return type

[**models::CombinedPdf**](CombinedPdf.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_custom_file

> async fn get_custom_file(&self, params: GetCustomFileParams) -> Result<models::CustomFile, Error<GetCustomFileError>>
Get a custom file

Returns a custom file, with a temporary URL to download it.

### Parameters

Set them on `GetCustomFileParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_file_id** | **String** | The custom file's ID. | [required] |

### Return type

[**models::CustomFile**](CustomFile.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_folder

> async fn get_folder(&self, params: GetFolderParams) -> Result<models::Folder, Error<GetFolderError>>
Get a folder

Returns a folder.

### Parameters

Set them on `GetFolderParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The folder's ID. | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_presign_url

> async fn get_presign_url(&self) -> Result<models::UploadPresign, Error<GetPresignUrlError>>
Get a presigned URL to upload a file

Returns a presigned S3 form for uploading a file directly to DocSpring, for files too large to send in a request body. POST the `fields` and the file (as `file`) to `url` as multipart/form-data, then pass `fields.key` as the `cache_id` when you create a custom file. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UploadPresign**](UploadPresign.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_signature_request

> async fn get_signature_request(&self, params: GetSignatureRequestParams) -> Result<models::SignatureRequest, Error<GetSignatureRequestError>>
Get a signature request

Returns a signature request, with the values the person has entered so far.

### Parameters

Set them on `GetSignatureRequestParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signature_request_id** | **String** | The signature request's ID. | [required] |

### Return type

[**models::SignatureRequest**](SignatureRequest.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_submission

> async fn get_submission(&self, params: GetSubmissionParams) -> Result<models::Submission, Error<GetSubmissionError>>
Get a submission

Returns a submission, with its data and links to download its PDF.

### Parameters

Set them on `GetSubmissionParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_id** | **String** | The submission's ID. | [required] |

### Return type

[**models::Submission**](Submission.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_submission_batch

> async fn get_submission_batch(&self, params: GetSubmissionBatchParams) -> Result<models::SubmissionBatch, Error<GetSubmissionBatchError>>
Get a batch

Returns a batch with its submissions. The batch has finished once `processed_at` is set: its `state` is `processed` when every PDF was generated, or `error` when any submission failed. 

### Parameters

Set them on `GetSubmissionBatchParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submission_batch_id** | **String** | The batch's ID. | [required] |
**include_data** | Option<**bool**> | Include each submission's `data` (null without it). |  |

### Return type

[**models::SubmissionBatch**](SubmissionBatch.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_template

> async fn get_template(&self, params: GetTemplateParams) -> Result<models::Template, Error<GetTemplateError>>
Get a template

Returns a template: its draft, or one of its published versions.

### Parameters

Set them on `GetTemplateParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**version** | Option<**String**> | A published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `draft`. |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_template_diff

> async fn get_template_diff(&self, params: GetTemplateDiffParams) -> Result<models::TemplateDiff, Error<GetTemplateDiffError>>
Compare a version of a template with the one before it

Returns what changed in the template's draft since its latest published version (or in a published version since the one before it): fields added, removed, changed and moved, the settings that changed, and whether the document was replaced. Check it before you publish a version. 

### Parameters

Set them on `GetTemplateDiffParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**version** | Option<**String**> | A published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `draft`. |  |

### Return type

[**models::TemplateDiff**](TemplateDiff.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_template_model_code

> async fn get_template_model_code(&self, params: GetTemplateModelCodeParams) -> Result<models::TemplateModelCode, Error<GetTemplateModelCodeError>>
Generate typed code for a template's submission data

Generates types for a template's submission data in TypeScript, JavaScript, Ruby, Python, PHP, Java, C#, Elixir, Go or Rust, so your app catches mistakes before it calls the API. `root_type_name` is the type of the whole `data` object, and `just_types=true` leaves out the code that converts JSON to and from the types. Java returns every class in `code`, each under a `// ClassName.java` comment. A template with no named fields returns 422. 

### Parameters

Set them on `GetTemplateModelCodeParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**lang** | [**ModelCodeLanguage**](ModelCodeLanguage.md) | The language to generate the code in. | [required] |
**version** | Option<**String**> | The template version: a published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `latest` (the draft when no version has been published), the version a submission uses by default. |  |
**just_types** | Option<**bool**> | Return only the types (interfaces, classes or structs), without the code that converts JSON to and from them. Default false. JavaScript has no types, PHP has no types-only code, and Rust's code is only types (serde converts the JSON), so theirs is the same either way. |  |

### Return type

[**models::TemplateModelCode**](TemplateModelCode.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_template_page_images

> async fn get_template_page_images(&self, params: GetTemplatePageImagesParams) -> Result<models::TemplatePageImages, Error<GetTemplatePageImagesError>>
Get images of a template's pages

Returns a JPEG image of each page of a PDF template's PDF, with temporary links to download them: the draft's PDF, or a published version's. Use them to see the pages that fields are placed on. 

### Parameters

Set them on `GetTemplatePageImagesParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**version** | Option<**String**> | A published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `draft`. |  |

### Return type

[**models::TemplatePageImages**](TemplatePageImages.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_template_preview

> async fn get_template_preview(&self, params: GetTemplatePreviewParams) -> Result<models::TemplatePreview, Error<GetTemplatePreviewError>>
Get a template preview

Returns a preview made with POST /templates/{template_id}/previews. Once it's `processed`, `pages` has a PNG image of each page with a temporary link to download it: a preview has no PDF. Previews are deleted an hour after they're created. 

### Parameters

Set them on `GetTemplatePreviewParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**preview_id** | **String** | The preview's ID. | [required] |

### Return type

[**models::TemplatePreview**](TemplatePreview.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_template_schema

> async fn get_template_schema(&self, params: GetTemplateSchemaParams) -> Result<std::collections::HashMap<String, serde_json::Value>, Error<GetTemplateSchemaError>>
Get the JSON Schema of a template's submission data

Returns the JSON Schema that a submission's `data` must match. Use it to check data before you create submissions, or to build a form for the template's fields. 

### Parameters

Set them on `GetTemplateSchemaParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**version** | Option<**String**> | The template version: a published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `latest` (the draft when no version has been published), the version a submission uses by default. |  |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## get_webhook

> async fn get_webhook(&self, params: GetWebhookParams) -> Result<models::Webhook, Error<GetWebhookError>>
Get a webhook

Returns a webhook.

### Parameters

Set them on `GetWebhookParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The webhook's ID. | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## list_combined_pdfs

> async fn list_combined_pdfs(&self, params: ListCombinedPdfsParams) -> Result<models::CombinedPdfList, Error<ListCombinedPdfsError>>
List combined PDFs

Returns the combined PDFs in your account, newest first, one page at a time.

### Parameters

Set them on `ListCombinedPdfsParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | How many items to return (1-100, default 50). |  |[default to 50]
**cursor** | Option<**String**> | The previous page's `next_cursor`, to get the next page. It's the ID of that page's last item, so get the next page before you delete or change the items on this one. |  |

### Return type

[**models::CombinedPdfList**](CombinedPdfList.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## list_custom_files

> async fn list_custom_files(&self, params: ListCustomFilesParams) -> Result<models::CustomFileList, Error<ListCustomFilesError>>
List custom files

Returns the custom files in your account, newest first, one page at a time.

### Parameters

Set them on `ListCustomFilesParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | How many items to return (1-100, default 50). |  |[default to 50]
**cursor** | Option<**String**> | The previous page's `next_cursor`, to get the next page. It's the ID of that page's last item, so get the next page before you delete or change the items on this one. |  |

### Return type

[**models::CustomFileList**](CustomFileList.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## list_folders

> async fn list_folders(&self, params: ListFoldersParams) -> Result<models::FolderList, Error<ListFoldersError>>
List folders

Returns the folders in your account, newest first, one page at a time. Pass `parent_folder_id` to list the folders in one folder. 

### Parameters

Set them on `ListFoldersParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_folder_id** | Option<**String**> | Only list the folders in this folder. Without it, the list has every folder, at any depth. |  |
**limit** | Option<**i32**> | How many items to return (1-100, default 50). |  |[default to 50]
**cursor** | Option<**String**> | The previous page's `next_cursor`, to get the next page. It's the ID of that page's last item, so get the next page before you delete or change the items on this one. |  |

### Return type

[**models::FolderList**](FolderList.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## list_submissions

> async fn list_submissions(&self, params: ListSubmissionsParams) -> Result<models::SubmissionList, Error<ListSubmissionsError>>
List submissions

Returns the submissions in your account, newest first, one page at a time. Each one is a summary: its `data` is null unless you pass `include_data=true`, and its `files` are empty unless you pass `include_files=true`. Get a submission by its ID for its signature requests, uploads and page images. 

### Parameters

Set them on `ListSubmissionsParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | Option<**String**> | Only list this template's submissions. |  |
**test** | Option<**bool**> | Only list test submissions (true) or live submissions (false). |  |
**created_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Only list submissions created after this time (RFC 3339, such as `2025-01-31T09:00:00Z`). |  |
**created_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Only list submissions created before this time. |  |
**include_data** | Option<**bool**> | Include each submission's `data` (null without it). |  |
**limit** | Option<**i32**> | How many items to return (1-100, default 50). |  |[default to 50]
**cursor** | Option<**String**> | The previous page's `next_cursor`, to get the next page. It's the ID of that page's last item, so get the next page before you delete or change the items on this one. |  |
**state** | Option<[**SubmissionState**](SubmissionState.md)> | Only list submissions in this state (one state per request). The ones that failed are `invalid_data`, `error`, `syntax_error`, `image_download_failed` and `image_processing_failed`, and one still `pending` an hour after it was created is stuck. |  |
**include_files** | Option<**bool**> | Include each submission's generated files and their download links (`files`, empty without it), without the page images' links. |  |

### Return type

[**models::SubmissionList**](SubmissionList.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## list_template_fields

> async fn list_template_fields(&self, params: ListTemplateFieldsParams) -> Result<models::TemplateFields, Error<ListTemplateFieldsError>>
List a template's fields

Returns a template's fields with all of their settings, page by page: the draft's fields, or a published version's. A PDF template's fields are placed on its pages: GET /templates/{template_id}/page_images has images of the pages. 

### Parameters

Set them on `ListTemplateFieldsParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**version** | Option<**String**> | A published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `draft`. |  |

### Return type

[**models::TemplateFields**](TemplateFields.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## list_template_submissions

> async fn list_template_submissions(&self, params: ListTemplateSubmissionsParams) -> Result<models::SubmissionList, Error<ListTemplateSubmissionsError>>
List a template's submissions

Returns a template's submissions (from every version of the template), newest first.

### Parameters

Set them on `ListTemplateSubmissionsParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**test** | Option<**bool**> | Only list test submissions (true) or live submissions (false). |  |
**created_after** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Only list submissions created after this time (RFC 3339, such as `2025-01-31T09:00:00Z`). |  |
**created_before** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Only list submissions created before this time. |  |
**include_data** | Option<**bool**> | Include each submission's `data` (null without it). |  |
**limit** | Option<**i32**> | How many items to return (1-100, default 50). |  |[default to 50]
**cursor** | Option<**String**> | The previous page's `next_cursor`, to get the next page. It's the ID of that page's last item, so get the next page before you delete or change the items on this one. |  |
**state** | Option<[**SubmissionState**](SubmissionState.md)> | Only list submissions in this state (one state per request). The ones that failed are `invalid_data`, `error`, `syntax_error`, `image_download_failed` and `image_processing_failed`, and one still `pending` an hour after it was created is stuck. |  |
**include_files** | Option<**bool**> | Include each submission's generated files and their download links (`files`, empty without it), without the page images' links. |  |

### Return type

[**models::SubmissionList**](SubmissionList.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## list_templates

> async fn list_templates(&self, params: ListTemplatesParams) -> Result<models::TemplateSummaryList, Error<ListTemplatesError>>
List templates

Returns the templates in your account (their drafts), newest first, one page at a time. Each one is a summary: get a template by its ID for its settings, its PDF in full, its HTML, defaults and published versions. 

### Parameters

Set them on `ListTemplatesParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | Only list templates whose name contains this text (ignoring case). |  |
**parent_folder_id** | Option<**String**> | Only list the templates in this folder (not in its subfolders). |  |
**limit** | Option<**i32**> | How many items to return (1-100, default 50). |  |[default to 50]
**cursor** | Option<**String**> | The previous page's `next_cursor`, to get the next page. It's the ID of that page's last item, so get the next page before you delete or change the items on this one. |  |

### Return type

[**models::TemplateSummaryList**](TemplateSummaryList.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## list_webhooks

> async fn list_webhooks(&self, params: ListWebhooksParams) -> Result<models::WebhookList, Error<ListWebhooksError>>
List webhooks

Returns the webhooks in your account, newest first, one page at a time. Filter them by event type, mode, or the API token, template or folder they're scoped to. 

### Parameters

Set them on `ListWebhooksParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_type** | Option<[**WebhookEventType**](WebhookEventType.md)> | Only list webhooks that subscribe to this event. |  |
**mode** | Option<[**WebhookMode**](WebhookMode.md)> | Only list webhooks set to this mode. |  |
**template_id** | Option<**String**> | Only list webhooks scoped to this template. |  |
**folder_id** | Option<**String**> | Only list webhooks scoped to this folder. |  |
**api_token_id** | Option<**String**> | Only list webhooks scoped to this API token. |  |
**limit** | Option<**i32**> | How many items to return (1-100, default 50). |  |[default to 50]
**cursor** | Option<**String**> | The previous page's `next_cursor`, to get the next page. It's the ID of that page's last item, so get the next page before you delete or change the items on this one. |  |

### Return type

[**models::WebhookList**](WebhookList.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## move_folder

> async fn move_folder(&self, params: MoveFolderParams) -> Result<models::Folder, Error<MoveFolderError>>
Move a folder

Moves a folder into another folder, or to the root with `parent_folder_id: null`. Its templates and folders move with it. A folder can't move into itself or one of its own folders. 

### Parameters

Set them on `MoveFolderParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The folder's ID. | [required] |
**body** | [**MoveFolderRequest**](MoveFolderRequest.md) | Where to move the folder. | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## move_template

> async fn move_template(&self, params: MoveTemplateParams) -> Result<models::Template, Error<MoveTemplateError>>
Move a template to a folder

Moves a template and all of its published versions to a folder, or to the root (`null`).

### Parameters

Set them on `MoveTemplateParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**MoveTemplateRequest**](MoveTemplateRequest.md) | Where to move the template. | [required] |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## publish_template_version

> async fn publish_template_version(&self, params: PublishTemplateVersionParams) -> Result<models::TemplateVersion, Error<PublishTemplateVersionError>>
Publish a template version

Publishes the template's draft as a new version. Its content (the PDF or HTML, fields, name and description) never changes, while settings such as `webhook_url` and `public_submissions` keep following the draft. `version_type` says which part of the latest version number goes up. While the template's PDF is downloading or being processed, or DocSpring is setting up its fields, it returns a 409 that says so. 

### Parameters

Set them on `PublishTemplateVersionParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**PublishTemplateVersionRequest**](PublishTemplateVersionRequest.md) | The version to publish. | [required] |

### Return type

[**models::TemplateVersion**](TemplateVersion.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## remove_template_fields

> async fn remove_template_fields(&self, params: RemoveTemplateFieldsParams) -> Result<models::TemplateFields, Error<RemoveTemplateFieldsError>>
Remove fields from a template

Removes fields from a template's draft and returns the fields that are left. Conditional logic that used only the removed fields is removed too. 

### Parameters

Set them on `RemoveTemplateFieldsParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**RemoveTemplateFieldsRequest**](RemoveTemplateFieldsRequest.md) | The fields to remove from the template's draft. | [required] |

### Return type

[**models::TemplateFields**](TemplateFields.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## rename_folder

> async fn rename_folder(&self, params: RenameFolderParams) -> Result<models::Folder, Error<RenameFolderError>>
Rename a folder

Renames a folder. The name must be unique within its parent folder. 

### Parameters

Set them on `RenameFolderParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | **String** | The folder's ID. | [required] |
**body** | [**RenameFolderRequest**](RenameFolderRequest.md) | The folder's new name. | [required] |

### Return type

[**models::Folder**](Folder.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## reorder_template_fields

> async fn reorder_template_fields(&self, params: ReorderTemplateFieldsParams) -> Result<models::TemplateFields, Error<ReorderTemplateFieldsError>>
Reorder a template's fields

Changes the order of the fields in a template's draft and returns them in their new order. Send every field's ID: each field stays on its page, and a page's fields are printed in this order, so a later field prints over an earlier one where they overlap. Their values are in this order in the template's data schema and forms. 

### Parameters

Set them on `ReorderTemplateFieldsParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**ReorderTemplateFieldsRequest**](ReorderTemplateFieldsRequest.md) | The new order of the fields in the template's draft. | [required] |

### Return type

[**models::TemplateFields**](TemplateFields.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## restore_template_version

> async fn restore_template_version(&self, params: RestoreTemplateVersionParams) -> Result<models::Template, Error<RestoreTemplateVersionError>>
Restore a template version

Replaces the template's draft with a copy of one of its published versions. The draft keeps its settings that apply to every version, such as `webhook_url`.

### Parameters

Set them on `RestoreTemplateVersionParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**RestoreTemplateVersionRequest**](RestoreTemplateVersionRequest.md) | The version to restore. | [required] |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## test_authentication

> async fn test_authentication(&self) -> Result<models::AuthenticationCheck, Error<TestAuthenticationError>>
Test authentication

Checks whether your API token is valid by making an authenticated request. This endpoint is useful for verifying credentials during setup or troubleshooting issues. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthenticationCheck**](AuthenticationCheck.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## update_signature_request

> async fn update_signature_request(&self, params: UpdateSignatureRequestParams) -> Result<models::SignatureRequest, Error<UpdateSignatureRequestError>>
Update a signature request

Changes a pending signature request's metadata, or how your app authenticated the person (for the audit trail). Once the person has opened it, only its metadata can be changed, and a completed or declined signature request can't be changed. 

### Parameters

Set them on `UpdateSignatureRequestParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signature_request_id** | **String** | The signature request's ID. | [required] |
**body** | [**UpdateSignatureRequestRequest**](UpdateSignatureRequestRequest.md) | The settings to change. Properties left out keep their values. | [required] |

### Return type

[**models::SignatureRequest**](SignatureRequest.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## update_template

> async fn update_template(&self, params: UpdateTemplateParams) -> Result<models::Template, Error<UpdateTemplateError>>
Update a template

Changes a template's draft. Properties left out keep their values. `defaults` and `html_engine_options` replace the whole object, and `field_groups` changes only the groups it lists. Settings such as `webhook_url`, `public_submissions` and `expire_after` apply to every published version too, and setting `parent_folder_id` moves the template and all of its published versions. 

### Parameters

Set them on `UpdateTemplateParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**UpdateTemplateRequest**](UpdateTemplateRequest.md) | The settings to change. Properties left out keep their values. | [required] |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## update_template_document

> async fn update_template_document(&self, params: UpdateTemplateDocumentParams) -> Result<models::Template, Error<UpdateTemplateDocumentError>>
Replace a template's document

Replaces a PDF template's PDF, or a Word template's Word document (from a .docx or an .odt), with one you uploaded with the presigned upload (GET /uploads/presign) (`cache_id`), a link to download it from (`document_url`), or the file in base64 (`document_base64`). A PDF template's fields stay where they are, unless `reimport_fields` replaces them with the new PDF's fillable fields, and a Word template gets a field for each new tag. To send the file as `multipart/form-data` instead, use PUT /templates/{template_id}/document/upload. 

### Parameters

Set them on `UpdateTemplateDocumentParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**UpdateTemplateDocumentRequest**](UpdateTemplateDocumentRequest.md) | The new PDF, or a Word template's new Word document, as one of `cache_id`, `document_url` and `document_base64`. | [required] |
**wait** | Option<**bool**> | Wait until the PDF has been processed, a `document_url`'s PDF or Word document downloaded, and an OpenDocument file converted, before responding. Default true on sync.api hosts. |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## update_template_fields

> async fn update_template_fields(&self, params: UpdateTemplateFieldsParams) -> Result<models::TemplateFields, Error<UpdateTemplateFieldsError>>
Update a template's fields

Changes fields of a template's draft and returns all of its fields. Settings left out keep their values. A shared setting (`type`, `required`, `default`, ...) changes for every field with the same name. 

### Parameters

Set them on `UpdateTemplateFieldsParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**UpdateTemplateFieldsRequest**](UpdateTemplateFieldsRequest.md) | The fields to change in the template's draft. | [required] |

### Return type

[**models::TemplateFields**](TemplateFields.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## update_webhook

> async fn update_webhook(&self, params: UpdateWebhookParams) -> Result<models::Webhook, Error<UpdateWebhookError>>
Update a webhook

Updates a webhook. Only the properties you send change.

### Parameters

Set them on `UpdateWebhookParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The webhook's ID. | [required] |
**body** | [**UpdateWebhookRequest**](UpdateWebhookRequest.md) | The settings to change. Properties left out keep their values. | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## upload_template

> async fn upload_template(&self, params: UploadTemplateParams) -> Result<models::Template, Error<UploadTemplateError>>
Create a template from an uploaded PDF or Word document

Creates a PDF template from a PDF, or a Word template from a Word document (.docx) or an OpenDocument text file (.odt), sent with the request (`multipart/form-data`). A Word template needs its `output_format`. For large files, upload the file with the presigned upload (GET /uploads/presign) and create the template with POST /templates. 

### Parameters

Set them on `UploadTemplateParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document** | **std::path::PathBuf** | A PDF (for a PDF template) or a Word document (.docx or .odt, for a Word template). DocSpring converts an OpenDocument text file (.odt) to a Word document after responding: the template's `document.state` is `converting` until then (sync.api hosts wait for it). | [required] |
**wait** | Option<**bool**> | Wait until the PDF has been processed, a `document_url`'s PDF or Word document downloaded, and an OpenDocument file converted, before responding. Default true on sync.api hosts. |  |
**name** | Option<**String**> | The template's name. |  |
**description** | Option<**String**> | The template's description. |  |
**parent_folder_id** | Option<**String**> | The folder to put the template in. Left out, it's created at the root. |  |
**ai_field_naming_language** | Option<[**AiFieldNamingLanguage**](AiFieldNamingLanguage.md)> | The language AI field setup names fields in: `auto` uses the form's language when it's written in the Latin alphabet without accents, and English otherwise; `english` always uses English; `form_language` uses the form's language and script. Default: your account's template settings. |  |
**detect_fields** | Option<**bool**> | Set up the PDF's fields as DocSpring's "Automatically detect fields" upload option does: import or detect the fields, name them with AI, and set up conditional logic. `field_source`, `field_naming` and `field_logic` choose how. Default false. `document.page_processing_statuses` and `document.field_logic_status` show how it's going, and `document.state` becomes `processed` once every page's fields are set up. |  |
**field_source** | Option<[**FieldSetupSource**](FieldSetupSource.md)> | With `detect_fields`, where the fields come from. `auto` (the default) imports the form fields of a fillable PDF and detects the fields of a PDF without them; `pdf_detection` detects the fields from the page layout, even on a fillable PDF; `pdf_form_fields` only imports the PDF's form fields. |  |
**field_naming** | Option<[**FieldSetupNaming**](FieldSetupNaming.md)> | With `detect_fields`, how the fields are named. `ai` (the default) names them and sets their types and settings with AI; `preserve` keeps the names of the PDF's form fields, and AI only names the fields it detected. `preserve` needs `field_source` `auto` or `pdf_form_fields`. |  |
**field_logic** | Option<[**FieldSetupLogic**](FieldSetupLogic.md)> | With `detect_fields`, whether AI reads the whole form once the fields are named and sets up conditional logic and required fields (`ai`) or not (`none`). Default: your account's template settings. |  |
**output_format** | Option<[**OutputFormat**](OutputFormat.md)> | What each submission of a Word template makes unless its request sets `output_formats`: `docx` (a Word document), `odt` (an OpenDocument file, converted from the Word document) or `pdf`. Required when `document` is a Word document. |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## upload_template_document

> async fn upload_template_document(&self, params: UploadTemplateDocumentParams) -> Result<models::Template, Error<UploadTemplateDocumentError>>
Replace a template's document with an uploaded file

Replaces a PDF template's PDF, or a Word template's Word document, with one sent with the request (`multipart/form-data`). A PDF template's fields stay where they are, unless `reimport_fields` replaces them with the new PDF's fillable fields, and a Word template gets a field for each new tag. 

### Parameters

Set them on `UploadTemplateDocumentParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**document** | **std::path::PathBuf** | The new PDF, or the new Word document (.docx or .odt) of a Word template. | [required] |
**wait** | Option<**bool**> | Wait until the PDF has been processed, a `document_url`'s PDF or Word document downloaded, and an OpenDocument file converted, before responding. Default true on sync.api hosts. |  |
**reimport_fields** | Option<**bool**> | Replace all of the template's fields with the fillable fields in the new PDF, once DocSpring has processed it (`document.field_reimport_state`). As on a first upload, the default typeface, font size, bold and color are taken from the PDF's fields; the template's other settings are kept. If the PDF has no fillable fields, the template keeps its fields. A locked template can't re-import its fields. Only for PDF templates: a Word template adds a field for each new tag. |  |

### Return type

[**models::Template**](Template.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)


## validate_submission

> async fn validate_submission(&self, params: ValidateSubmissionParams) -> Result<models::SubmissionValidation, Error<ValidateSubmissionError>>
Check a submission without creating it

Checks a submission as creating it would (its data against the template's fields, its settings and signature requests) without saving it, generating anything or using credits. It doesn't check your account's credits. The response says whether the submission is valid, and if not, has the error that creating it would return. 

### Parameters

Set them on `ValidateSubmissionParams::builder()`.


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | The template's ID. | [required] |
**body** | [**ValidateSubmissionRequest**](ValidateSubmissionRequest.md) | The submission's data and settings. | [required] |
**version** | Option<**String**> | The template version to use: a published version (`1.2.0`, or `1` for the latest 1.x.x), `latest` or `draft`. Default `latest` (the draft when no version has been published). |  |

### Return type

[**models::SubmissionValidation**](SubmissionValidation.md)

### Authorization

[api_token_basic](../README.md#install-and-configure)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#methods) [[Back to Model list]](../README.md#methods) [[Back to README]](../README.md)

