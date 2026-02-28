# ProgressInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**percentage_complete** | Option<**i32**> | The current completion rate (in percents). | [optional]
**elapsed_seconds** | Option<**i64**> | Time (in seconds) passed since the build actually started. Does not include time spent in queue. | [optional]
**estimated_total_seconds** | Option<**i64**> | The currently estimated build duration (in seconds). The sum of `elapsedSeconds`, `leftSeconds`, and post-build activity duration. | [optional]
**left_seconds** | Option<**i64**> | The estimated remaining build time (in seconds) | [optional]
**current_stage_text** | Option<**String**> | The description of the currently performed build stage. Combines the build status with the latest build log message. | [optional]
**outdated** | Option<**bool**> | Returns *true* for [history builds](https://www.jetbrains.com/help/teamcity/history-build.html). Builds are labeled as outdated (history) if there are finished non-personal builds that processed newer (or same) changes as in this build. | [optional]
**probably_hanging** | Option<**bool**> | Returns *true* if the server suspects this build is hanging. Builds are considered hung when their run duration exceeds the estimates and the build sends no new messages for some time. | [optional]
**last_activity_time** | Option<**String**> | The timestamp of the latest build log message. | [optional]
**outdated_reason_build** | Option<[**models::Build**](Build.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


