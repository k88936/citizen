# CloudInstance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The internally used read-only ID of this instance. | [optional]
**name** | Option<**String**> | The name of this instance. | [optional]
**state** | Option<**String**> | A String value that specifies the instance current status. Can return the following values: `scheduled to start`, `scheduled to stop`, `starting`, `running`, `restarting`, `stopping`, `stopped`, `unknown`, and `error`. | [optional]
**start_date** | Option<**String**> | The time of this instance's initial start. | [optional]
**network_address** | Option<**String**> | The actual instance address. | [optional]
**href** | Option<**String**> | The relative (without the TeamCity server URL) link to this instance. | [optional]
**image** | Option<[**models::CloudImage**](CloudImage.md)> |  | [optional]
**agent** | Option<[**models::Agent**](Agent.md)> |  | [optional]
**error_message** | Option<**String**> | This property is deprecated, use `error` instead. | [optional]
**error** | Option<[**models::CloudError**](CloudError.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


