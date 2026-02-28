# HealthItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**identity** | Option<**String**> | The unique health item ID. | [optional]
**severity** | Option<**Severity**> | The health item severity. Supported values:   * *INFO* for suggestions and minor issues.  * *WARN* for issues that have no immediate negative impact.  * *ERROR* for misconfigurations and issues that prevent normal server and/or project operations. (enum: INFO, WARN, ERROR) | [optional]
**health_category** | Option<[**models::HealthCategory**](HealthCategory.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


