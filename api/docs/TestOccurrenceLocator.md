# TestOccurrenceLocator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected_project** | Option<**String**> | Project (direct or indirect parent) locator. | [optional]
**branch** | Option<**String**> |  | [optional]
**build** | Option<**String**> | Build locator. | [optional]
**build_type** | Option<**String**> | Build type locator. | [optional]
**count** | Option<**i32**> | For paginated calls, how many entities to return per page. | [optional]
**currently_failing** | Option<**bool**> | Is currently failing. | [optional]
**currently_investigated** | Option<**bool**> | Is currently investigated. | [optional]
**currently_muted** | Option<**String**> |  | [optional]
**id** | Option<**String**> | Entity ID. | [optional]
**ignored** | Option<**bool**> | Is ignored. | [optional]
**include_personal** | Option<**bool**> |  | [optional]
**item** | Option<**String**> | Supply multiple locators and return a union of the results. | [optional]
**lookup_limit** | Option<**i32**> | Limit processing to the latest `<lookupLimit>` entities. | [optional]
**muted** | Option<**bool**> | Is muted. | [optional]
**name** | Option<**String**> |  | [optional]
**new_failure** | Option<**String**> |  | [optional]
**start** | Option<**i32**> | For paginated calls, from which entity to start rendering the page. | [optional]
**status** | Option<**Status**> |  (enum: unknown, normal, warning, failure, error, success) | [optional]
**test** | Option<**String**> | Test locator. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


