# VcsRootInstanceLocator

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affected_project** | Option<**String**> | Project (direct or indirect parent) locator. | [optional]
**build** | Option<**String**> | Build locator. | [optional]
**build_type** | Option<**String**> | Build type locator. | [optional]
**count** | Option<**i32**> | For paginated calls, how many entities to return per page. | [optional]
**id** | Option<**String**> | Entity ID. | [optional]
**item** | Option<**String**> | Supply multiple locators and return a union of the results. | [optional]
**lookup_limit** | Option<**i32**> | Limit processing to the latest `<lookupLimit>` entities. | [optional]
**project** | Option<**String**> | Project (direct parent) locator. | [optional]
**property** | Option<**Property**> |  (enum: exists, not-exists, equals, does-not-equal, starts-with, contains, does-not-contain, ends-with, any, matches, does-not-match, more-than, no-more-than, less-than, no-less-than, ver-more-than, ver-no-more-than, ver-less-than, ver-no-less-than) | [optional]
**start** | Option<**i32**> | For paginated calls, from which entity to start rendering the page. | [optional]
**r#type** | Option<**String**> | Type of VCS (e.g. jetbrains.git). | [optional]
**vcs_root** | Option<**String**> | VCS root locator. | [optional]
**versioned_settings** | Option<**bool**> | Is used for versioned settings. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


