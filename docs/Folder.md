# Folder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The folder's ID. | 
**name** | **String** | The folder's name. It can't contain `/`, and is unique within its parent folder. | 
**path** | **String** | The path of the folder this folder is in, such as `/Contracts`. `/` for a folder at the root. | 
**parent_folder_id** | Option<**String**> | The ID of the folder this folder is in. Null for a folder at the root. | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** | When the folder was created. | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** | When the folder was last updated. | 

[[Back to Model list]](../README.md#methods) [[Back to API list]](../README.md#methods) [[Back to README]](../README.md)


