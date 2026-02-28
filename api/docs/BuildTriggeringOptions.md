# BuildTriggeringOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clean_sources** | Option<**bool**> | Returns *true* if the agent should clear its build checkout directory and pull all sources anew; otherwise, *false*. | [optional]
**clean_sources_in_all_dependencies** | Option<**bool**> | Returns *true* if the current 'cleanSources' behavior also be applied to all snapshot dependency builds, otherwise, *false*. | [optional]
**rebuild_all_dependencies** | Option<**bool**> | Use *true* to prevent the build from reusing any of its snapshot dependency builds and re-run all dependency configurations; *false* to let TeamCity decide which snapshot builds should be re-run and which have [suitable builds]()https://www.jetbrains.com/help/teamcity/snapshot-dependencies.html#Suitable+Builds. | [optional]
**rebuild_failed_or_incomplete_dependencies** | Option<**bool**> | *true* to re-run all failed, failed to start, and canceled snapshot dependency builds; *false* to let TeamCity decide which snapshot builds should be re-run and which have [suitable builds]()https://www.jetbrains.com/help/teamcity/snapshot-dependencies.html#Suitable+Builds. | [optional]
**queue_at_top** | Option<**bool**> | *true* to place the build at the top spot of the build queue, *false* to let TeamCity automatically process this build. | [optional]
**freeze_settings** | Option<**bool**> | Use *true* to collect all code commits and versioned settings right away; *false* to do this when build is assigned to the agent and is ready to start. | [optional]
**tag_dependencies** | Option<**bool**> | Use *true* to label all dependency builds with tags specified in the `Build.Tags` field; *false* to label the current build only. | [optional]
**rebuild_dependencies** | Option<[**models::BuildTypes**](BuildTypes.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


