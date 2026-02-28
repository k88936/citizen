# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | Option<**String**> | The group key that serves as a unique group identifier. In TeamCity REST API, group keys are used as the default group dimension, meaning you can omit the `key:` in path locators (for example, `/app/rest/userGroups/ALL_USERS_GROUP`). | [optional]
**name** | Option<**String**> | The public group name displayed in TeamCity UI. | [optional]
**href** | Option<**String**> | The relative (without the TeamCity server URL) link to this group. | [optional]
**description** | Option<**String**> | The public user group description. | [optional]
**parent_groups** | Option<[**models::Groups**](Groups.md)> |  | [optional]
**child_groups** | Option<[**models::Groups**](Groups.md)> |  | [optional]
**users** | Option<[**models::Users**](Users.md)> |  | [optional]
**roles** | Option<[**models::RoleAssignments**](RoleAssignments.md)> |  | [optional]
**properties** | Option<[**models::Properties**](Properties.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


