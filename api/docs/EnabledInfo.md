# EnabledInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | Option<**bool**> | Returns *true* if the current object state is enabled (for example, pinned for build `PinInfo` or authorized for agent `AuthorizedInfo`; otherwise, *false*. | [optional]
**comment** | Option<[**models::Comment**](Comment.md)> |  | [optional]
**status_switch_time** | Option<**String**> | Returns date and time when the agent state should be automatically flipped. Specify this value to temporarily disable or enable agents, or leave it empty to permanently change the agent state (until it is updated manually). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


