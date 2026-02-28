# CloudImage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of this cloud image. Typically consists of the truncated image name added to the parent profile ID. | [optional]
**name** | Option<**String**> | The full image name. Includes the public cloud image name and ID. | [optional]
**href** | Option<**String**> | The shortened (without the server URL) link to this cloud image. | [optional]
**profile** | Option<[**models::CloudProfile**](CloudProfile.md)> |  | [optional]
**instances** | Option<[**models::CloudInstances**](CloudInstances.md)> |  | [optional]
**error_message** | Option<**String**> | This property is deprecated, use `error` instead. | [optional]
**error** | Option<[**models::CloudError**](CloudError.md)> |  | [optional]
**agent_type_id** | Option<**i32**> | The value of the `typeId` property of all agents spawned from this image. Allows you to identify sibling cloud agents. | [optional]
**agent_pool_id** | Option<**i32**> | The ID of the agent pool to which agents spawned from this image belong. | [optional]
**operating_system_name** | Option<**String**> | The full name of the OS installed on virtual machines spawned from this image. | [optional]
**web_url** | Option<**String**> | Returns the full web link to access this cloud image. | [optional]
**locator** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


