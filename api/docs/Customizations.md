# Customizations

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parameters** | Option<**std::collections::HashMap<String, String>**> | The list of parameters added by user via the Run Custom Build dialog. Existing parameters whose values were customized are not included. | [optional]
**changes** | Option<**std::collections::HashMap<String, String>**> | A unique changeset processed by the build. For example, a manually uploaded .diff patch (for personal builds) or an older revision that was already processed before (for history builds) | [optional]
**artifact_dependencies** | Option<**std::collections::HashMap<String, String>**> | The list of artifact dependency customizations. See `build.artifactDependencyChanges` for more information. | [optional]
**dependencies** | Option<**std::collections::HashMap<String, String>**> | The list of custom snapshot dependency settings for this build, such as when an upstream build was explicitly re-run, bypassing the default reuse setting, or skipped. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


