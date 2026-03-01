# ServerGlobalSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_vcs_check_interval** | Option<**i32**> | An Integer value that specifies how often (in seconds) TeamCity polls VCS repositories for changes. The default value is 60 seconds. | [optional]
**enforce_default_vcs_check_interval** | Option<**bool**> | **true**, if project administrators cannot set the polling interval in VCS root settings lower than the `defaultVCSCheckInterval`; otherwise, **false**. | [optional]
**use_encryption** | Option<**bool**> | **true** if TeamCity encrypts all sensitive values using a custom key; **false** if it uses the defaul scrambled mechanism for these values. | [optional]
**artifacts_domain_isolation** | Option<**bool**> | **true** if TeamCity publishes build artifacts to a separate domain; **false** if the artifacts can be accessed from the regular TeamCity server URL. | [optional]
**artifacts_url** | Option<**String**> | If `artifactsDomainIsolation` is set to **true**, this property returns the URL of the artifacts isolation domain. | [optional]
**encryption_key** | Option<**String**> | Returns the encryption key. This property is hidden from the payload. | [optional]
**artifact_directories** | Option<**String**> | A String containing the concatenated paths to the artifact directories used by the server. Paths are separated using the new line characters. | [optional]
**root_url** | Option<**String**> | The TeamCity server URL. | [optional]
**default_quiet_period** | Option<**i32**> | The delay (in seconds) between a moment TeamCity detects a new VCS change and a moment a new build processing this change starts. Individual build configuration triggers can override this global setting. | [optional]
**default_execution_timeout** | Option<**i32**> | The maximum allowed build duration. Zero or negative values mean no time limit. | [optional]
**max_artifacts_number** | Option<**i64**> | The maximum number of artifacts that can be published per build, or `-1` if there is no limit. This number does not include hidden artifacts. | [optional]
**max_artifact_size** | Option<**i64**> | The maximum allowed artifact size (in bytes), or `-1` if there is no size limit. Artifacts that exceed these limit will not be published. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


