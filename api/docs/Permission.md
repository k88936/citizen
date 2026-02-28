# Permission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique permission ID. | [optional]
**name** | Option<**String**> | The read-only description of this permission | [optional]
**global** | Option<**bool**> | Returns **true** if the permission can be assigned only globally (for example, the `view_audit_log` permission); **false** if it can be limited to a specific TeamCity project (`cancel_build` or `view_project`).   This property is a characteristic of a permission itself and does not reflect how this permission is used in a specific role for a specific user. Users can be granted a non-global permission (either directly or by joining a user group) both for individual projects and globally (for all projects on a server). For example, when [assigning a role to an individual user](https://www.jetbrains.com/help/teamcity/rest/userapi.html#addRoleToUser), a request body can set the `scope` parameter to either `g` (global) or `p:{Project_ID}`. The actual permission scope for a specific user is stored in the [PermissionAssignment.isGlobalScope](https://www.jetbrains.com/help/teamcity/rest/permissionassignment.html) property. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


