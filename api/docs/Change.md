# Change

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The unique ID of this change object. | [optional]
**version** | Option<**String**> | The full revision SHA that corresponds to this change, or a time stamp of a personal build that processed a custom diff | [optional]
**internal_version** | Option<**String**> |  | [optional]
**username** | Option<**String**> | This property is obsolete. To identify the change author, read the `commiter` property instead. | [optional]
**date** | Option<**String**> | Returns the timestamp that corresponds to the moment this change was pushed to the VCS. If this is a custom change uploaded to a personal build, returns the timestamp of this build instead. | [optional]
**registration_date** | Option<**String**> | Returns the timestamp that corresponds to the moment this change was registered in the TeamCity database. | [optional]
**personal** | Option<**bool**> | Returns *true* if this change was a custom patch uploaded to a personal build. If this is a regular change that comes from a VCS (even if the build that processed it is a personal one), returns *false*. | [optional]
**href** | Option<**String**> | Returns the shortened (without the server URL) link to the current change. | [optional]
**web_url** | Option<**String**> | The TeamCity server link to this change. | [optional]
**comment** | Option<**String**> | The comment an author of the change left. | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]
**r#type** | Option<**String**> | Specifies the origin of the change.  * *VCS_CHANGE* — the regular change that originates from a repository related to the target build. * *SNAPSHOT_DEPENDENCY_VCS_CHANGE* — the change is processed in another build on which the target build depends (via the snapshot dependency). Add the `changesFromDependencies:true` locator to your request if you want the response to include changes processed in upstream chain builds. | [optional]
**snapshot_dependency_link** | Option<[**models::SnapshotDependencyLink**](SnapshotDependencyLink.md)> |  | [optional]
**files** | Option<[**models::FileChanges**](FileChanges.md)> |  | [optional]
**vcs_root_instance** | Option<[**models::VcsRootInstance**](VcsRootInstance.md)> |  | [optional]
**parent_changes** | Option<[**models::Changes**](Changes.md)> |  | [optional]
**parent_revisions** | Option<[**models::Items**](Items.md)> |  | [optional]
**attributes** | Option<[**models::Properties**](Properties.md)> |  | [optional]
**stores_project_settings** | Option<**bool**> |  | [optional]
**status** | Option<[**models::ChangeStatus**](ChangeStatus.md)> |  | [optional]
**commiter** | Option<[**models::Commiter**](Commiter.md)> |  | [optional]
**can_edit_comment** | Option<**bool**> |  | [optional]
**locator** | Option<**String**> | This property supports the internal infrastructure and is not intended to be used in your code. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


