# PermissionAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_global_scope** | Option<**bool**> | Returns **false** if the permission was granted to a user for a specific project only. Returns **true** for global permissions (for example, the `change_own_profile` permission) and project-specific permissions that were assigned globally (for example, the `view_file_content` permission included in a role that was [granted to a group](https://www.jetbrains.com/help/teamcity/rest/groupapi.html#addRoleToGroup) with `'scope': 'g'` in the request body). | [optional]
**permission** | Option<[**models::Permission**](Permission.md)> |  | [optional]
**project** | Option<[**models::Project**](Project.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


