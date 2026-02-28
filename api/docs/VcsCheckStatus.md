# VcsCheckStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**String**> | The status of the changes check.  * **finished** — the check for changes is complete, no new check is scheduled. * **scheduled** — the new check for changes is scheduled. * **started** — a check for changes is currently in progress. | [optional]
**requestor_type** | Option<**String**> | The authority that issued a check for changes request.   * **schedule** — a check for changes is initiated by the TeamCity server, as a part of default repository polling mechanism. * **build** — a default check for changes that preceds any new build starting. * **user** — a TeamCity user clicked the configuration's 'Check for pending changes' action in TeamCity UI. * **commit_hook** — a check for changes was triggered by a [post-commit webhook](https://www.jetbrains.com/help/teamcity/configuring-vcs-post-commit-hooks-for-teamcity.html). | [optional]
**timestamp** | Option<**String**> | The time of the latest check entry. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


