# VcsRoot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique ID of the root. Typically consists of trunkated parent project ID and root name. | [optional]
**internal_id** | Option<**String**> | The internal read-only ID of the root. | [optional]
**uuid** | Option<**String**> | The internal read-only UUID of the root. | [optional]
**name** | Option<**String**> | The public root name displayed in TeamCity UI. | [optional]
**vcs_name** | Option<**String**> | The type of a VCS provider to which this root connects. | [optional]
**modification_check_interval** | Option<**i32**> | The interval (in seconds) for polling VCS for new changes. Equals **null** if this root uses default server polling interval. | [optional]
**href** | Option<**String**> | The short (without the TeamCity server address) link to this VCS root. | [optional]
**project** | Option<[**models::Project**](Project.md)> |  | [optional]
**properties** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**vcs_root_instances** | Option<[**models::VcsRootInstances**](VcsRootInstances.md)> |  | [optional]
**repository_id_strings** | Option<[**models::Items**](Items.md)> |  | [optional]
**locator** | Option<**String**> | The part of the `href` property value that specifies the object locator. Ommitted from regular responses. | [optional]
**project_locator** | Option<**String**> | This property is deprecated. | [optional]
**connection_id** | Option<**String**> | Optional ID of a project connection to use for authentication when creating a new VCS root. Used only for POST; not returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


