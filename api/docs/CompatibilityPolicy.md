# CompatibilityPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**policy** | Option<**String**> | Returns whether the agent can run only builds of specific build configurations. Available values:  * *any* — agents with this policy can run builds of any build configurations. * *selected* — agents with this policy can only run builds of configurations stored in the `buildTypes` collection. | [optional]
**build_types** | Option<[**models::BuildTypes**](BuildTypes.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


