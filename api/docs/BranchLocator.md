# BranchLocator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dollar_single_value** | Option<**String**> | Branch name. | [optional]
**build** | Option<**String**> | Build locator. | [optional]
**build_type** | Option<**String**> | Build type locator. | [optional]
**changes_from_dependencies** | Option<**ChangesFromDependencies**> | Include branches from dependencies. (enum: true, false, any) | [optional]
**count** | Option<**i64**> | For paginated calls, how many entities to return per page. | [optional]
**default** | Option<**Default**> | Is default branch. (enum: true, false, any) | [optional]
**name** | Option<**String**> | Branch name. | [optional]
**policy** | Option<**Policy**> | Branch retrieval policy. (enum: vcs_branches, active_vcs_branches, history_branches, active_history_branches, active_history_and_active_vcs_branches, all_branches) | [optional]
**start** | Option<**i64**> | For paginated calls, from which entity to start rendering the page. | [optional]
**unspecified** | Option<**Unspecified**> | Branch is unspecified if for some reason TeamCity failed to find appropriate branch name for the build. (enum: true, false, any) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


