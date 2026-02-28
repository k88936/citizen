# TriggeredBy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | Returns the following values depending on the authority that triggered this build:   * *user* if the build was triggered manually in TeamCity UI or via REST API. * *snapshotDependency* if the build is a part of a build chain and was requested by a downstream build configuration. * *vcs* if the build was initiated by the VCS Trigger that found new commits in the remote repository. * *schedule* if the build was initiated by the Schedule Trigger. * *finishBuild* if the build was initiated by the Finish Build Trigger. * *retry* if the build was initiated by the Retry Trigger that re-starts failed builds. | [optional]
**details** | Option<**String**> | Optional details regarding the triggering event. | [optional]
**date** | Option<**String**> | The timestamp of the build triggering. | [optional]
**display_text** | Option<**String**> | The summary displayed next to the 'Triggered' line in TeamCity UI. Enumerates authorities that caused this build to run and displays the build's parent configuration name and number. | [optional]
**raw_value** | Option<**String**> | The string representation of the `properties` array. | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]
**build** | Option<[**models::Build**](Build.md)> |  | [optional]
**build_type** | Option<[**models::BuildType**](BuildType.md)> |  | [optional]
**properties** | Option<[**models::Properties**](Properties.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


