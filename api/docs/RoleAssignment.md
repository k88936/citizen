# RoleAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role_id** | Option<**String**> | The ID of a role assigned to the current user or user group. | [optional]
**scope** | Option<**String**> | The project scope defining where the assigned role applies. Returns `g` (for `global`) for roles that have no project scope by desgin (for example, the `SYSTEM_ADMIN` role), and 'p:{ProjectID}' for project-specific roles. | [optional]
**href** | Option<**String**> | The relative (without the TeamCity server URL) link to this role assignment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


