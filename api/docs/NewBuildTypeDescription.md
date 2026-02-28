# NewBuildTypeDescription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**copy_all_associated_settings** | Option<**bool**> | Returns **true** if all settings of a cloned configuration or template should be copied; otherwise, **false**. | [optional]
**projects_ids_map** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**build_types_ids_map** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**vcs_roots_ids_map** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**name** | Option<**String**> | The public name of a build configuration or template. | [optional]
**id** | Option<**String**> | The ID of a build configuration or a template. | [optional]
**source_build_type_locator** | Option<**String**> | This property is deprecated, use `sourceBuildType` instead. | [optional]
**source_build_type** | Option<[**models::BuildType**](BuildType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


