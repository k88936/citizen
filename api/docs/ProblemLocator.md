# ProblemLocator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected_project** | Option<**String**> | Project (direct or indirect parent) locator. | [optional]
**build** | Option<**String**> | Build locator. | [optional]
**count** | Option<**i32**> | For paginated calls, how many entities to return per page. | [optional]
**currently_failing** | Option<**bool**> | Is currently failing. | [optional]
**currently_investigated** | Option<**bool**> | Is currently investigated. | [optional]
**currently_muted** | Option<**bool**> | Is currently muted. | [optional]
**id** | Option<**String**> | Entity ID. | [optional]
**identity** | Option<**String**> |  | [optional]
**item** | Option<**String**> | Supply multiple locators and return a union of the results. | [optional]
**lookup_limit** | Option<**i32**> | Limit processing to the latest `<lookupLimit>` entities. | [optional]
**start** | Option<**i32**> | For paginated calls, from which entity to start rendering the page. | [optional]
**r#type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


