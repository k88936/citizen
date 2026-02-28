# Revisions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**i32**> | The number of Revision objects in this collection. | [optional]
**revision** | Option<[**Vec<models::Revision>**](Revision.md)> | The list of Revision objects owned by this collection. | [optional]
**fail_on_missing_revisions** | Option<**bool**> | **true** if the build should fail when child Revision objects are missing valid `version` values; **false** if in this case a build should process latest changes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


