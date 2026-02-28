# HealthCategories

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | Option<**i32**> | The current number of HealthCategory objects in this collection. | [optional]
**health_category** | Option<[**Vec<models::HealthCategory>**](HealthCategory.md)> | The list of HealthCategory objects in this collection. | [optional]
**href** | Option<**String**> | The relative link (without the server URL) used to retrieve this object. | [optional]
**next_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the next batch. | [optional]
**prev_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the previous batch. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


