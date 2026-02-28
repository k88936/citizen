# Builds

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**build** | Option<[**Vec<models::Build>**](Build.md)> | The list of builds owned by this collection that satisfy the given locator. | [optional]
**href** | Option<**String**> | The relative link (without the server URL) used to retrieve this object. | [optional]
**next_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the next batch. | [optional]
**prev_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the previous batch. | [optional]
**count** | Option<**i32**> | The current number of Build objects in this list. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


