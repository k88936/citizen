# Investigation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The investigation locator. | [optional]
**state** | Option<**State**> | The investigation state. Supported values:   * *TAKEN* — the investigation is assigned to a TeamCity user who works on resolving the problem. * *FIXED* — the investigation was marked as fixed. * *GIVEN_UP* — the investigation is no longer active, the 'no investigation' option was selected. * *NONE* — unknown investigation status. (enum: TAKEN, FIXED, GIVEN_UP, NONE) | [optional]
**href** | Option<**String**> | The short (without the TeamCity server address) link to this investigation. | [optional]
**assignee** | Option<[**models::User**](User.md)> |  | [optional]
**assignment** | Option<[**models::Comment**](Comment.md)> |  | [optional]
**scope** | Option<[**models::ProblemScope**](ProblemScope.md)> |  | [optional]
**target** | Option<[**models::ProblemTarget**](ProblemTarget.md)> |  | [optional]
**resolution** | Option<[**models::Resolution**](Resolution.md)> |  | [optional]
**responsible** | Option<[**models::User**](User.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


