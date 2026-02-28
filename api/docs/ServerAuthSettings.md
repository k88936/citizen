# ServerAuthSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_guest** | Option<**bool**> | **true** if users can log into TeamCity as [guests](https://www.jetbrains.com/help/teamcity/enabling-guest-login.html); **false** if only registered users can access TeamCity. | [optional]
**guest_username** | Option<**String**> | The default username for all users logged in as guests. | [optional]
**welcome_text** | Option<**String**> | The custom text displayed on the TeamCity log in page. | [optional]
**collapse_login_form** | Option<**bool**> | **true** if the username/password fields on the login page is collapsed by default, promoting authentication via third-party services. **false** if the username/password fields are initially visible. | [optional]
**per_project_permissions** | Option<**bool**> | **true** if the [per-project authorization mode](https://www.jetbrains.com/help/teamcity/managing-roles-and-permissions.html#Per-Project+Authorization+Mode) is enabled; otherwise, **false**. | [optional]
**email_verification** | Option<**bool**> | **true** if users must enter their emails on registration; **false** if they can skip this setting. | [optional]
**build_authentication_mode** | Option<**String**> | **strict** if builds can access only their own projects, projects that own VCS roots, and artifact dependencies projects. **lax** if builds have elevated permissions to access any project on the server (the default behavior). | [optional]
**modules** | Option<[**models::AuthModules**](AuthModules.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


