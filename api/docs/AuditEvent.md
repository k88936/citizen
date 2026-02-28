# AuditEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The automatically generated unique ID of the event. | [optional]
**timestamp** | Option<**String**> | The time of the logged event. | [optional]
**comment** | Option<**String**> | The event summary that briefly explains its essence. Certain events (such as starting a build) have no comment attached. | [optional]
**action** | Option<[**models::AuditAction**](AuditAction.md)> |  | [optional]
**related_entities** | Option<[**models::RelatedEntities**](RelatedEntities.md)> |  | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


