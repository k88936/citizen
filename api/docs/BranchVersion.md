# BranchVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<**String**> | The full revision SHA. Points to the latest commit in this branch. | [optional]
**unspecified** | Option<**bool**> | Returns **true** for unspecified branches; otherwise, **false**. A branch is labeled as unspecified when TeamCity fails to find a branch for this build. Such branches have a predefined '<unspecified>' name. | [optional]
**group_flag** | Option<**bool**> | Returns **true** if the branch belongs to the automatically created 'My Branches' group; otherwise, **false**. Branch grouping allows TeamCity to automatically categorize branches based on current TeamCity user commits. | [optional]
**builds** | Option<[**models::Builds**](Builds.md)> |  | [optional]
**last_activity** | Option<**String**> | Returns the date and time of the last branch activity (the last TeamCity build or the latest VCS commit). | [optional]
**name** | Option<**String**> | A public branch name displayed in TeamCity UI. To get a full VCS name instead (for example, 'refs/heads/main'), read the *vcsBranchName* property of a nested Revision entity. | [optional]
**default** | Option<**bool**> | Returns **true** if this branch is the default branch of this build configuration; otherwise, **false**. When obtaining builds of the specific BuildType, add the *branch(default:any)* locator to get builds from all existing branches. | [optional]
**active** | Option<**bool**> | Returns **true** for active branches and **false** for inactive ones. A branch is considered inactive if it had neither new TeamCity builds in the last 24 hours, nor recent VCS repository changes in the last 7 days. Learn more: [Active branches](https://www.jetbrains.com/help/teamcity/working-with-feature-branches.html#Active+Branches). | [optional]
**internal_name** | Option<**String**> | An internal branch name. Returns `<default>` for default branches, and real branch names for other (non-default) branches. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


