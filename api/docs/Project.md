# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The public project ID. If not specified, TeamCity generates one by removing all non-alphanumeric characters from the project name. | [optional]
**internal_id** | Option<**String**> | The internally used read-only project ID. | [optional]
**uuid** | Option<**String**> | The universally unique identifier of this project. | [optional]
**name** | Option<**String**> | The public project name. | [optional]
**parent_project_id** | Option<**String**> | The ID of a TeamCity project that owns this project. Returns '_Root' if this is project resides on the topmost level. | [optional]
**parent_project_internal_id** | Option<**String**> | The internal ID of a TeamCity project that owns this project. This property is deprecated, use `parent` and `parentProjectId` to identify parent projects and retrieve their properties. | [optional]
**parent_project_name** | Option<**String**> | The public name of a TeamCity project that owns this project. This property is deprecated, use `parent` and `parentProjectId` to identify parent projects and retrieve their properties. | [optional]
**archived** | Option<**bool**> | Returns **true** if the project is archived; otherwise, **false**. | [optional]
**r#virtual** | Option<**bool**> | Returns **true** if this project is dynamically created by TeamCity; **false** if this a regular project created by a user. | [optional]
**description** | Option<**String**> | The optional project description, or **null** if none was set. | [optional]
**href** | Option<**String**> | The shortest REST API link to this project (without the TeamCity server URL). | [optional]
**web_url** | Option<**String**> | The regular URL of the project. | [optional]
**links** | Option<[**models::Links**](Links.md)> |  | [optional]
**parent_project** | Option<[**models::Project**](Project.md)> |  | [optional]
**read_only_ui** | Option<[**models::StateField**](StateField.md)> |  | [optional]
**default_template** | Option<[**models::BuildType**](BuildType.md)> |  | [optional]
**build_types** | Option<[**models::BuildTypes**](BuildTypes.md)> |  | [optional]
**templates** | Option<[**models::BuildTypes**](BuildTypes.md)> |  | [optional]
**deployment_dashboards** | Option<[**models::DeploymentDashboards**](DeploymentDashboards.md)> |  | [optional]
**parameters** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**vcs_roots** | Option<[**models::VcsRoots**](VcsRoots.md)> |  | [optional]
**project_features** | Option<[**models::ProjectFeatures**](ProjectFeatures.md)> |  | [optional]
**projects** | Option<[**models::Projects**](Projects.md)> |  | [optional]
**cloud_profiles** | Option<[**models::CloudProfiles**](CloudProfiles.md)> |  | [optional]
**ancestor_projects** | Option<[**models::Projects**](Projects.md)> |  | [optional]
**locator** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


