# PermissionRestriction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_global_scope** | Option<**bool**> | Returns **false** if the permission was granted to an access token for a specific project only. Returns **true** for global permissions (for example, the `change_own_profile` permission) and project-specific permissions that were assigned globally (for example, the `authorize_agent` permission is granted on the Root project level). | [optional]
**project** | Option<[**models::Project**](Project.md)> |  | [optional]
**permission** | Option<[**models::Permission**](Permission.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


