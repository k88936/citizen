# FileChange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**before_revision** | Option<**String**> | The revision a modified file had before the change. | [optional]
**after_revision** | Option<**String**> | The revision a modified file obtained after the change. | [optional]
**change_type** | Option<**String**> | Specifies the type of a file edit: `added`, `removed`, or `edited`. | [optional]
**change_type_comment** | Option<**String**> |  | [optional]
**file** | Option<**String**> | The name of a modified file, including the relative repository path. | [optional]
**relative_file** | Option<**String**> | The relative name of a modified file. | [optional]
**directory** | Option<**bool**> | Returns **true** if a modified entity is a directory instead of a file; otherwise, **true**. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


