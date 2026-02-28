# ChangeStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**running_successfully_builds** | Option<**i32**> | The number of builds that process this change and are currently running with the 'running and failing' status. | [optional]
**pending_build_types** | Option<**i32**> | The number of build configurations (build types) that have not yet processed this change and have it in their 'pending changes' list. | [optional]
**total_problems** | Option<**i32**> | The total number of build problems associated with this change, across all build configurations and their builds. | [optional]
**new_failed_tests** | Option<**i32**> | The total number of new failed tests associated with this change, across all build configurations and their builds. | [optional]
**other_failed_tests** | Option<**i32**> | The total number of failed tests associated with this change. This number does not inlcude the number of new test failures. | [optional]
**queued_builds_count** | Option<**i32**> | The number of currently queued builds whose list of processed changes includes this change. | [optional]
**critical_builds** | Option<[**models::Builds**](Builds.md)> |  | [optional]
**not_critical_builds** | Option<[**models::Builds**](Builds.md)> |  | [optional]
**new_tests_failed_builds** | Option<[**models::Builds**](Builds.md)> |  | [optional]
**compilation_error_builds** | Option<[**models::Builds**](Builds.md)> |  | [optional]
**failed_builds** | Option<**i32**> | Returns how many of `finishedBuilds` failed. | [optional]
**cancelled_builds** | Option<**i32**> | Returns how many of `finishedBuilds` were cancelled before they finished. | [optional]
**running_builds** | Option<**i32**> | The number of currently running builds processing this change, regardless of their current build status. | [optional]
**successful_builds** | Option<**i32**> | Returns how many of `finishedBuilds` finished successfully. | [optional]
**finished_builds** | Option<**i32**> | The number of finished builds that processed this change, regardless of their final status. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


