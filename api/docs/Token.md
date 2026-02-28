# Token

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The public access token name. | [optional]
**creation_time** | Option<**String**> | The timestamp of a moment when this access token was created. | [optional]
**value** | Option<**String**> | The token value that should be used instead of a password when logging in TeamCity, or sent in the 'Authorization: Bearer' header when using REST API. For security, TeamCity displays a token value only once — at creation — and never returns it for existing tokens. | [optional]
**expiration_time** | Option<**String**> | The timestamp of token expiration, or **null** if this is a permanent token that never expires. | [optional]
**permission_restrictions** | Option<[**models::PermissionRestrictions**](PermissionRestrictions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


