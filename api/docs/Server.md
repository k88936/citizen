# Server

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<**String**> | The full TeamCity server version, including the build number and, if present, the EAP (early access preview) flag. | [optional]
**version_major** | Option<**i32**> | The major TeamCity version that points to the year when this version was released. | [optional]
**version_minor** | Option<**i32**> | The major TeamCity version that points to the month of `versionMajor` when this version was released. | [optional]
**start_time** | Option<**String**> | The most recent date this TeamCity server was started. | [optional]
**current_time** | Option<**String**> | The current TeamCity server time, which corresponds to the server's machine time. | [optional]
**build_number** | Option<**String**> | The TeamCity build number. | [optional]
**build_date** | Option<**String**> | The date when this TeamCity version was built by JetBrains. | [optional]
**internal_id** | Option<**String**> | The internally used read-only server ID. | [optional]
**role** | Option<**String**> | In a multi-node setup, returns the role of the current TeamCity server. Returns `main_node` for single-node setups. | [optional]
**web_url** | Option<**String**> | Returns the regular web URL of the server, including the server port. | [optional]
**projects** | Option<[**models::Href**](Href.md)> |  | [optional]
**vcs_roots** | Option<[**models::Href**](Href.md)> |  | [optional]
**builds** | Option<[**models::Href**](Href.md)> |  | [optional]
**users** | Option<[**models::Href**](Href.md)> |  | [optional]
**user_groups** | Option<[**models::Href**](Href.md)> |  | [optional]
**agents** | Option<[**models::Href**](Href.md)> |  | [optional]
**build_queue** | Option<[**models::Href**](Href.md)> |  | [optional]
**agent_pools** | Option<[**models::Href**](Href.md)> |  | [optional]
**investigations** | Option<[**models::Href**](Href.md)> |  | [optional]
**mutes** | Option<[**models::Href**](Href.md)> |  | [optional]
**artifacts_url** | Option<**String**> | Returns the artifacts isolation URL: the URL of a separate domain that stores build artifacts and mitigates the risk of TeamCity server attacks carried out by users who access these artifacts. If the isolation URL is not set (default setup), returns an empty string. | [optional]
**nodes** | Option<[**models::Href**](Href.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


