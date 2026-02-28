# Changes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change** | Option<[**Vec<models::Change>**](Change.md)> | The list of Change objects owned by this collection. | [optional]
**href** | Option<**String**> | The relative link (without the server URL) used to retrieve this object. | [optional]
**next_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the next batch. | [optional]
**prev_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the previous batch. | [optional]
**count** | Option<**i32**> | The number of Change objects owned by this collection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


