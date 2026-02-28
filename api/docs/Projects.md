# Projects

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**i32**> | Returns the number of Project objects in this collection. | [optional]
**href** | Option<**String**> | The relative link (without the server URL) to the current collection. | [optional]
**next_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the next batch. | [optional]
**prev_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the next batch. | [optional]
**project** | Option<[**Vec<models::Project>**](Project.md)> | The list of Project entities owned by this collection. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


