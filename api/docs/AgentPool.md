# AgentPool

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique ID of this pool. The Default pool has the ID value of *0*. | [optional]
**name** | Option<**String**> | The public name of the pool. | [optional]
**href** | Option<**String**> | Returns the shortened (without the server URL) link to the current pool. | [optional]
**max_agents** | Option<**i32**> | The pool capacity. Returns *null* for pools with the unlimited number of agent slots. | [optional]
**owner_project** | Option<[**models::Project**](Project.md)> |  | [optional]
**projects** | Option<[**models::Projects**](Projects.md)> |  | [optional]
**agents** | Option<[**models::Agents**](Agents.md)> |  | [optional]
**locator** | Option<**String**> |  | [optional]
**agent_types** | Option<[**models::AgentTypes**](AgentTypes.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


