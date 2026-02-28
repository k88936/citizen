# CompositeApprovalRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**required_approvals_count** | Option<**i32**> | The number of TeamCity user approvals required to start this build. These can be any users that belong to either `users` or `groups` lists. | [optional]
**groups** | Option<[**Vec<models::Group>**](Group.md)> | The list of groups whose users can approve the corresponding build. | [optional]
**users** | Option<[**Vec<models::User>**](User.md)> | The list of users who can approve the corresponding build. | [optional]
**currently_approved_by** | Option<[**models::Users**](Users.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


