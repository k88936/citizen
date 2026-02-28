# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> | The mandatory username. | [optional]
**name** | Option<**String**> | The public user name, or **null** if not specified. | [optional]
**id** | Option<**i64**> | The unique user account ID. | [optional]
**email** | Option<**String**> |  | [optional]
**last_login** | Option<**String**> | The date and time when this user last logged in TeamCity. | [optional]
**password** | Option<**String**> | The user password. For security reasons, TeamCity does not return the value of this field via `GET` requests. You can only use `PUT` requests to set new password values. | [optional]
**has_password** | Option<**bool**> | Returns **true** if the user has a regular password; otherwise, **false**. Users with no password cannot log in using the username/password credentials pair. | [optional]
**realm** | Option<**String**> | This member supports the internal infrastructure and is not intended to be used directly from your code. | [optional]
**href** | Option<**String**> | The relative (without the TeamCity server URL) link to this user. | [optional]
**properties** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**roles** | Option<[**models::RoleAssignments**](RoleAssignments.md)> |  | [optional]
**groups** | Option<[**models::Groups**](Groups.md)> |  | [optional]
**locator** | Option<**String**> | The locator required to access this user. Only used for `POST` requests. | [optional]
**avatars** | Option<[**models::UserAvatars**](UserAvatars.md)> |  | [optional]
**enabled2_fa** | Option<**bool**> | Returns **true** if the user configured the [two-factor user authentication](https://www.jetbrains.com/help/teamcity/managing-two-factor-authentication.html); otherwise, **false**. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


