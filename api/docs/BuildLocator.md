# BuildLocator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected_project** | Option<**String**> | Project (direct or indirect parent) locator. | [optional]
**agent** | Option<**String**> | Agent locator. | [optional]
**agent_type_id** | Option<**i32**> | typeId of agent used to execute build. | [optional]
**any** | Option<**bool**> | State can be any. | [optional]
**artifact_dependency** | Option<**String**> |  | [optional]
**branch** | Option<**String**> | Branch locator. | [optional]
**build_type** | Option<**String**> | Build type locator. | [optional]
**canceled** | Option<**bool**> | Is canceled. | [optional]
**compatible_agent** | Option<**String**> | Agent locator. | [optional]
**composite** | Option<**bool**> | Is composite. | [optional]
**count** | Option<**i32**> | For paginated calls, how many entities to return per page. | [optional]
**default_filter** | Option<**bool**> | If true, applies default filter which returns only \"normal\" builds (finished builds which are not canceled, not failed-to-start, not personal, and on default branch (in branched build configurations)). | [optional]
**failed_to_start** | Option<**bool**> | Is failed to start. | [optional]
**finish_date** | Option<**String**> | Requires either date or build dimension. | [optional]
**finished** | Option<**bool**> | Is finished. | [optional]
**hanging** | Option<**bool**> | Is hanging. | [optional]
**history** | Option<**bool**> | Is history build. | [optional]
**id** | Option<**String**> | Entity ID. | [optional]
**item** | Option<**String**> | Supply multiple locators and return a union of the results. | [optional]
**lookup_limit** | Option<**i32**> | Limit processing to the latest `<lookupLimit>` entities. | [optional]
**number** | Option<**String**> | Build number string. | [optional]
**personal** | Option<**bool**> | Is a personal build. | [optional]
**pinned** | Option<**bool**> | Is pinned. | [optional]
**project** | Option<**String**> | Project (direct parent) locator. | [optional]
**property** | Option<**Property**> |  (enum: exists, not-exists, equals, does-not-equal, starts-with, contains, does-not-contain, ends-with, any, matches, does-not-match, more-than, no-more-than, less-than, no-less-than, ver-more-than, ver-no-more-than, ver-less-than, ver-no-less-than) | [optional]
**queued** | Option<**bool**> | Is queued. | [optional]
**queued_date** | Option<**String**> | Requires either date or build dimension. | [optional]
**revision** | Option<**String**> | Build revision. | [optional]
**running** | Option<**bool**> | Is running. | [optional]
**snapshot_dependency** | Option<**String**> |  | [optional]
**start** | Option<**i32**> | For paginated calls, from which entity to start rendering the page. | [optional]
**start_date** | Option<**String**> | Requires either date or build dimension. | [optional]
**state** | Option<**State**> |  (enum: queued, running, finished, any) | [optional]
**status** | Option<**String**> | Status text. | [optional]
**tag** | Option<**String**> | Tag locator. | [optional]
**task_id** | Option<**i32**> | ID of a build or build promotion. | [optional]
**user** | Option<**String**> | For personal builds checks the owner of the build, triggerring user in other cases. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


