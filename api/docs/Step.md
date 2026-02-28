# Step

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The internal read-only ID of the object. This property is inherited from the base PropEntity class. | [optional]
**name** | Option<**String**> | The public object name. Some objects, like build steps, support public names, while others, like build triggers, do not. This property is inherited from the base PropEntity class. | [optional]
**r#type** | Option<**String**> | The object type. This property is inherited from the base PropEntity class. | [optional]
**disabled** | Option<**bool**> | Returns **rue** if the object is disabled and inactive; otherwise, **false**. This property is inherited from the base PropEntity class. | [optional]
**inherited** | Option<**bool**> | Returns **rue** if the object is inherited from another object; otherwise, **false**. This property is inherited from the base PropEntity class. | [optional]
**href** | Option<**String**> | Returns the shortened (without the server URL) link to the current object. This property is inherited from the base PropEntity class. | [optional]
**properties** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**short_description** | Option<**String**> | The short description of this build step, displayed in the list of configuration steps in TeamCity UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


