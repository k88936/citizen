# CloudProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The internally used cloud profile ID. | [optional]
**name** | Option<**String**> | The public cloud profile name displayed in TeamCity UI. | [optional]
**cloud_provider_id** | Option<**String**> | A shortened name of the cloud provider. | [optional]
**href** | Option<**String**> | The shortened (without the TeamCity server URL) link to this profile. | [optional]
**project** | Option<[**models::Project**](Project.md)> |  | [optional]
**images** | Option<[**models::CloudImages**](CloudImages.md)> |  | [optional]
**error** | Option<[**models::CloudError**](CloudError.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


