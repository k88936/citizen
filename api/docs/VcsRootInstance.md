# VcsRootInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The internal read-only ID of this root instance. | [optional]
**vcs_root_id** | Option<**String**> | The ID of a parent VcsRoot. | [optional]
**vcs_root_internal_id** | Option<**String**> | The internal ID of a parent VcsRoot | [optional]
**name** | Option<**String**> | The public name of a parent VcsRoot. | [optional]
**vcs_name** | Option<**String**> | The type of a VCS provider to which the parent VcsRoot connects. | [optional]
**modification_check_interval** | Option<**i32**> | The actual interval (in seconds) for polling VCS for new changes. | [optional]
**commit_hook_mode** | Option<**bool**> | Returns **true** if the configuration uses post-commit hooks to get new repository changes' notifications; **false** if pending changes are retrieved via periodic repository polling. | [optional]
**last_version** | Option<**String**> | The latest revision of a remote repository branch targeted by this root instance. | [optional]
**last_version_internal** | Option<**String**> | This is the internal property and is not intented to be used. | [optional]
**href** | Option<**String**> | The short (without TeamCity server address) link to this root instance. | [optional]
**vcs_root** | Option<[**models::VcsRoot**](VcsRoot.md)> |  | [optional]
**status** | Option<[**models::VcsStatus**](VcsStatus.md)> |  | [optional]
**repository_state** | Option<[**models::RepositoryState**](RepositoryState.md)> |  | [optional]
**properties** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**repository_id_strings** | Option<[**models::Items**](Items.md)> |  | [optional]
**project_locator** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


