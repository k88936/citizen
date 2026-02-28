# TestOccurrence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**status** | Option<**Status**> |  (enum: UNKNOWN, NORMAL, WARNING, FAILURE, ERROR) | [optional]
**ignored** | Option<**bool**> |  | [optional]
**duration** | Option<**i32**> |  | [optional]
**run_order** | Option<**String**> |  | [optional]
**new_failure** | Option<**bool**> |  | [optional]
**muted** | Option<**bool**> |  | [optional]
**currently_muted** | Option<**bool**> |  | [optional]
**currently_investigated** | Option<**bool**> |  | [optional]
**href** | Option<**String**> |  | [optional]
**ignore_details** | Option<**String**> |  | [optional]
**details** | Option<**String**> |  | [optional]
**test** | Option<[**models::Test**](Test.md)> |  | [optional]
**mute** | Option<[**models::Mute**](Mute.md)> |  | [optional]
**build** | Option<[**models::Build**](Build.md)> |  | [optional]
**first_failed** | Option<[**models::TestOccurrence**](TestOccurrence.md)> |  | [optional]
**next_fixed** | Option<[**models::TestOccurrence**](TestOccurrence.md)> |  | [optional]
**invocations** | Option<[**models::TestOccurrences**](TestOccurrences.md)> |  | [optional]
**metadata** | Option<[**models::TestRunMetadata**](TestRunMetadata.md)> |  | [optional]
**log_anchor** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


