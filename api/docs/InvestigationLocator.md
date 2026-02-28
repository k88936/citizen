# InvestigationLocator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected_project** | Option<**String**> | Project (direct or indirect parent) locator. | [optional]
**affected_project_and_parents** | Option<**String**> | Project with sub projects as well as parents. | [optional]
**assignee** | Option<**String**> | User locator. | [optional]
**assignment_project** | Option<**String**> | Project (direct parent) locator. | [optional]
**build_type** | Option<**String**> | Build type locator. | [optional]
**count** | Option<**i64**> | For paginated calls, how many entities to return per page. | [optional]
**problem** | Option<**String**> | Problem locator. | [optional]
**reporter** | Option<**String**> | User locator. | [optional]
**resolution** | Option<**Resolution**> |  (enum: manually, when_fixed) | [optional]
**since_date** | Option<**String**> | yyyyMMddTHHmmss+ZZZZ | [optional]
**start** | Option<**i64**> | For paginated calls, from which entity to start rendering the page. | [optional]
**state** | Option<**State**> |  (enum: taken, fixed, given_up, none) | [optional]
**test** | Option<**String**> | Test locator. | [optional]
**r#type** | Option<**Type**> |  (enum: anyProblem, test, problem, unknown) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


