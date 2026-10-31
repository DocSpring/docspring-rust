# SubmissionBatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The batch's ID. | 
**state** | [**models::SubmissionBatchState**](SubmissionBatchState.md) | `pending` while the files are being generated, then `processed`. `error` as soon as a submission fails, while the others can still be pending: the batch has finished once `processed_at` is set. | 
**metadata** | **std::collections::HashMap<String, serde_json::Value>** | Your own data about the batch, such as an ID from your app. | 
**total_count** | **i32** | The number of submissions in the request. | 
**pending_count** | **i32** | The submissions whose files haven't been generated yet. | 
**error_count** | **i32** | The submissions that were invalid or whose files couldn't be generated. | 
**completion_percentage** | **i32** | The percentage of the batch's submissions that have finished, from 0 to 100. | 
**submissions** | [**Vec<models::Submission>**](Submission.md) | The batch's submissions, in the order of the request. Their `data` is null unless you pass `include_data=true`. A batch that failed validation only has the submissions that were recorded as `invalid_data`. | 
**processed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | When every submission had finished. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the batch was created. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | When the batch was last updated. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


