# BuildTriggerCustomization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enforce_clean_checkout** | Option<**bool**> | Returns **true** if the agent should clear the build checkout directory before checking out sources from the VCS; otherwise, **false**. | [optional]
**enforce_clean_checkout_for_dependencies** | Option<**bool**> | Returns **true** if the `enforceCleanCheckout` behavior should all apply to all snapshot dependencies of this build; otherwise, **false**. | [optional]
**parameters** | Option<[**models::Properties**](Properties.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


