# CloudImages

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_image** | Option<[**Vec<models::CloudImage>**](CloudImage.md)> | The list of cloud images owned by this collection. | [optional]
**count** | Option<**i32**> | The number of cloud images in this collection. | [optional]
**next_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the next batch. | [optional]
**prev_href** | Option<**String**> | If the list of returned entities exceeds the request `count` value, TeamCity splits it into multiple batches. This property returns the endpoint that allows you to obtain the previous batch. | [optional]
**href** | Option<**String**> | The relative link (without the server URL) used to retrieve this object. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


