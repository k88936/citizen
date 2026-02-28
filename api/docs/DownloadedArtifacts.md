# DownloadedArtifacts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**unfiltered_count** | Option<**i32**> | The total number of artifacts downloaded by this build. | [optional]
**download_info** | Option<[**Vec<models::DownloadInfo>**](DownloadInfo.md)> | Contains detailed information about downloaded files: the upstream build that provided artifacts, the number of artifacts delivered by this upstream build, and the individual file data. | [optional]
**count** | Option<**i32**> | The number of artifacts downloaded by this build, matching the number of DownloadInfo records. Can be lower than `unfilteredCount` if request locator filters out certain builds. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


