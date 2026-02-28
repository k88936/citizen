# ApprovalInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_be_approved_by_current_user** | Option<**bool**> | Returns *true* if the currently authorized user is among TeamCity users who can approve this build; otherwise, *false*. | [optional]
**timeout_timestamp** | Option<**String**> | The timestamp when the build is (or was) canceled due to reviewers failing to approve it on time. This value is calculated at the moment a build is queued. | [optional]
**configuration_valid** | Option<**bool**> | Returns *true* if the configuration of users and user groups that should approve this build is valid; otherwise, *false*. | [optional]
**user_approvals** | Option<[**models::UserApprovals**](UserApprovals.md)> |  | [optional]
**group_approvals** | Option<[**models::GroupApprovals**](GroupApprovals.md)> |  | [optional]
**composite_approvals** | Option<[**models::CompositeApprovals**](CompositeApprovals.md)> |  | [optional]
**approval_reasons** | Option<[**Vec<models::ApprovableBuild>**](ApprovableBuild.md)> | Returns the reasons why TeamCity is waiting for build approval. | [optional]
**build_chain_builds** | Option<[**Vec<models::BuildsWithReason>**](BuildsWithReason.md)> | The list of other builds (both upstream and downstream) that belong to the same build chain and require an approval. | [optional]
**status** | Option<**Status**> | The current build status. Supported values:  * *waitingForApproval* — the build is waiting in a build queue; * *approved* — the build received a required number of approvals from reviewers; * *timedOut* — the build was automatically canceled as the reviewer(s) failed to approve it within the given time; * *canceled* — reviewer(s) explicitly canceled the build. (enum: waitingForApproval, approved, timedOut, canceled) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


