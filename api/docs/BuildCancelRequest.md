# BuildCancelRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | A note that explains why this build was canceled. This note will be written to the canceled build's `build.canceledInfo.text` property. | [optional]
**readd_into_queue** | Option<**bool**> | If *true*, a new identical build will be queued as soon as the current build is canceled. If *false*, a build will simply be canceled. Similarly to TeamCity UI, the option to re-queue a build is only available if this build is already running. Builds that are still queued can be only aborted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


