# Cleanup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | **true** if the periodic clean-up is enabled; otherwise, **false**. | [optional]
**max_cleanup_duration** | Option<**i32**> | The maximum clean-up duration (in munutes). If a clean-up exceeds this limit, TeamCity will automatically abort it. | [optional]
**daily** | Option<[**models::Daily**](Daily.md)> |  | [optional]
**cron** | Option<[**models::Cron**](Cron.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


