# VcsLabel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | Option<**String**> | The public VCS tag name | [optional]
**failure_reason** | Option<**String**> | The reason why the sources could not be labeled. Provides additional information why the `status` field returns FAILED. | [optional]
**status** | Option<**Status**> | The status of the labeling (tagging) operation. Possible values:   * *UNKNOWN* — the operation status is currently unknown. * *SUCCESSFUL_SET* — the sources were successfully tagged. * *IS_BEING_SET* — the labeling (tagging) process is currently ongoing. * *FAILED* — the sources could not be labeled. * *DISABLED_FOR_THE_ROOT* — the current root is excluded from the labeling process. * *LABELING_NOT_SUPPORTED* — VCS does not support labeling. (enum: UNKNOWN, SUCCESSFUL_SET, IS_BEING_SET, FAILED, DISABLED_FOR_THE_ROOT, LABELING_NOT_SUPPORTED) | [optional]
**build_id** | Option<**i64**> | The internal ID of the build during which the sources were tagged. | [optional]
**vcs_root_instance** | Option<[**models::VcsRootInstance**](VcsRootInstance.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


