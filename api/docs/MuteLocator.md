# MuteLocator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected_project** | Option<**String**> | Project affected by the mutes. | [optional]
**affected_project_and_parents** | Option<**String**> | Project with sub projects as well as parents. | [optional]
**count** | Option<**i64**> | For paginated calls, how many entities to return per page. | [optional]
**creation_date** | Option<**String**> | Mute creation time, yyyyMMddTHHmmss+ZZZZ. | [optional]
**id** | Option<**i64**> | Internal mute id. | [optional]
**problem** | Option<**String**> | Problem for which mute is assigned. | [optional]
**project** | Option<**String**> | Project in which mute is assigned. | [optional]
**reporter** | Option<**String**> | User who muted this test/problem. | [optional]
**resolution** | Option<**Resolution**> | Unmute condition. (enum: manually, whenfixed, attime) | [optional]
**start** | Option<**i64**> | For paginated calls, from which entity to start rendering the page. | [optional]
**test** | Option<**String**> | test for which mute is assigned | [optional]
**r#type** | Option<**Type**> | What is muted. (enum: test, problem) | [optional]
**unmute_date** | Option<**String**> | Automatic unmute time, yyyyMMddTHHmmss+ZZZZ. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


