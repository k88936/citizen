# BuildType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The build configuration ID. Typically consists of a trunkated project and configuration names in the ProjectName_ConfigurationName format. | [optional]
**internal_id** | Option<**String**> | The automatically generated build configuration ID. This is the internal property and is not intented to be used. | [optional]
**name** | Option<**String**> | The public build configuration name displayed in TeamCity UI. | [optional]
**template_flag** | Option<**bool**> | Returns *true* if the current BuildType entity is a build configuration template; *false* if this is a regular build configuration. Inspect the `templates` field to view all templates attached to the target configuration. | [optional]
**r#type** | Option<**Type**> | The build configuration type:  * *regular* — a regular build configuration. * *composite* — a composite configuration that aggregates results of multiple upstream build chain configurations. * *deployment* — a deployment build configuration.  See [Changing build configuration type](https://www.jetbrains.com/help/teamcity/changing-build-configuration-type.html) for more information. (enum: regular, composite, deployment) | [optional]
**paused** | Option<**bool**> | Returns *true* if a configuration is paused and cannot start new builds automatically upon build trigger requests; otherwise, *false*. To pause or unpause a configuration, use the configuration's Actions menu in TeamCity UI or send a `PUT` request with the required Boolean value to the `/app/rest/buildTypes/_build_type_locator_/paused` endpoint. | [optional]
**uuid** | Option<**String**> | An internally used universally unique identifier. | [optional]
**description** | Option<**String**> | The custom build configuration description, or *null* if not set. | [optional]
**project_name** | Option<**String**> | The public name of a parent TeamCity project. | [optional]
**project_id** | Option<**String**> | The ID of a parent TeamCity project that owns this build configuration. | [optional]
**project_internal_id** | Option<**String**> | The automatically generated ID of a parent project. This is the internal property and is not intented to be used. | [optional]
**href** | Option<**String**> | The short link (without the address of a TeamCity server) to this build configuration. | [optional]
**web_url** | Option<**String**> | The regular URL for this build configuration or template. | [optional]
**inherited** | Option<**bool**> | Returns *null* for build configurations, *true* for configuration templates inherited from other templates, and *false* for regular templates extracted from build configurations. You can only create templates inherited from other base templates in Kotlin. | [optional]
**links** | Option<[**models::Links**](Links.md)> |  | [optional]
**project** | Option<[**models::Project**](Project.md)> |  | [optional]
**templates** | Option<[**models::BuildTypes**](BuildTypes.md)> |  | [optional]
**template** | Option<[**models::BuildType**](BuildType.md)> |  | [optional]
**vcs_root_entries** | Option<[**models::VcsRootEntries**](VcsRootEntries.md)> |  | [optional]
**settings** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**parameters** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**output_parameters** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**steps** | Option<[**models::Steps**](Steps.md)> |  | [optional]
**features** | Option<[**models::Features**](Features.md)> |  | [optional]
**triggers** | Option<[**models::Triggers**](Triggers.md)> |  | [optional]
**snapshot_dependencies** | Option<[**models::SnapshotDependencies**](SnapshotDependencies.md)> |  | [optional]
**artifact_dependencies** | Option<[**models::ArtifactDependencies**](ArtifactDependencies.md)> |  | [optional]
**agent_requirements** | Option<[**models::AgentRequirements**](AgentRequirements.md)> |  | [optional]
**branches** | Option<[**models::Branches**](Branches.md)> |  | [optional]
**builds** | Option<[**models::Builds**](Builds.md)> |  | [optional]
**investigations** | Option<[**models::Investigations**](Investigations.md)> |  | [optional]
**compatible_agents** | Option<[**models::Agents**](Agents.md)> |  | [optional]
**compatible_cloud_images** | Option<[**models::CloudImages**](CloudImages.md)> |  | [optional]
**vcs_root_instances** | Option<[**models::VcsRootInstances**](VcsRootInstances.md)> |  | [optional]
**external_status_allowed** | Option<**bool**> | This is the internal property and is not intented to be used. | [optional]
**pause_comment** | Option<[**models::Comment**](Comment.md)> |  | [optional]
**locator** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


