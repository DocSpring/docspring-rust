# CombinedPdf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The combined PDF's ID. | 
**state** | [**models::CombinedPdfState**](CombinedPdfState.md) | `pending` until the PDF has been generated, then `processed`. `insufficient_credits`: waiting for credits; `error_message` says whether the PDF is generated once you add them. `account_suspended`: waiting for the account to be reactivated, for up to 30 days. `error`: the PDFs couldn't be merged (see `error_message`). Waiting ends in `error` if it's discarded or takes over 30 days. | 
**source_pdfs** | [**Vec<models::CombinedPdfSource>**](CombinedPdfSource.md) | The PDFs to merge, in order. | 
**metadata** | **std::collections::HashMap<String, serde_json::Value>** | Your own data about the combined PDF, such as an ID from your app. Its `pdf_filename` key names the PDF's file (`files.pdf.filename`, and its download): a `.pdf` ending is removed, each run of characters other than letters, digits, `-`, `_` and `.` becomes one `_`, and the first 128 characters are kept, then `.pdf` is added. Without it, the file is named after the combined PDF's ID (`com_1234567890abcdef01.pdf`). | 
**has_password** | **bool** | Whether the PDF needs a password to open (`password`). | 
**delete_custom_files** | **bool** | Delete the custom files in `source_pdfs` once the combined PDF has been generated (they're kept if it fails). Other combined PDFs can't use them after that. A test API token can't delete custom files. | 
**error_message** | Option<**String**> | Why the PDFs couldn't be merged, when `state` is `error`. | 
**files** | [**models::CombinedPdfFiles**](CombinedPdfFiles.md) | The generated files, by format. Empty until the PDF has been generated, and once it has expired. | 
**actions** | [**Vec<models::SubmissionAction>**](SubmissionAction.md) | The uploads of the PDF to your storage (account integrations). | 
**expired** | **bool** | An expired combined PDF's PDF, metadata, password and source URLs have been deleted. | 
**expires_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the combined PDF expires (`expires_in` after it's generated). Null if it's kept until you delete it. | 
**processed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When the PDF was generated. Null until then. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the combined PDF was created. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | When the combined PDF was last updated. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


