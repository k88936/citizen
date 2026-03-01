# \VersionedSettingsApi

All URIs are relative to *http://teamcity.k88936.top*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_versioned_settings_tokens**](VersionedSettingsApi.md#add_versioned_settings_tokens) | **POST** /app/rest/projects/{locator}/versionedSettings/tokens | Add Versioned Settings Tokens.
[**check_for_versioned_settings_changes**](VersionedSettingsApi.md#check_for_versioned_settings_changes) | **POST** /app/rest/projects/{locator}/versionedSettings/checkForChanges | Check for changes in Versioned Settings.
[**commit_current_settings**](VersionedSettingsApi.md#commit_current_settings) | **POST** /app/rest/projects/{locator}/versionedSettings/commitCurrentSettings | Perform Versioned Settings action: Commit current settings to VCS.
[**delete_versioned_settings_config_parameter**](VersionedSettingsApi.md#delete_versioned_settings_config_parameter) | **DELETE** /app/rest/projects/{locator}/versionedSettings/config/parameters/{name} | Delete Versioned Settings config parameter value.
[**delete_versioned_settings_tokens**](VersionedSettingsApi.md#delete_versioned_settings_tokens) | **DELETE** /app/rest/projects/{locator}/versionedSettings/tokens | Delete Versioned Settings Tokens.
[**get_versioned_settings_config**](VersionedSettingsApi.md#get_versioned_settings_config) | **GET** /app/rest/projects/{locator}/versionedSettings/config | Get Versioned Settings config.
[**get_versioned_settings_config_parameter**](VersionedSettingsApi.md#get_versioned_settings_config_parameter) | **GET** /app/rest/projects/{locator}/versionedSettings/config/parameters/{name} | Get Versioned Settings config parameter value.
[**get_versioned_settings_context_parameters**](VersionedSettingsApi.md#get_versioned_settings_context_parameters) | **GET** /app/rest/projects/{locator}/versionedSettings/contextParameters | Get Versioned Settings Context Parameters.
[**get_versioned_settings_projects_to_load**](VersionedSettingsApi.md#get_versioned_settings_projects_to_load) | **GET** /app/rest/projects/{locator}/versionedSettings/affectedProjects | Get a list of projects that are affected by Load Settings from VCS action.
[**get_versioned_settings_status**](VersionedSettingsApi.md#get_versioned_settings_status) | **GET** /app/rest/projects/{locator}/versionedSettings/status | Get current status of Versioned Settings.
[**get_versioned_settings_tokens**](VersionedSettingsApi.md#get_versioned_settings_tokens) | **GET** /app/rest/projects/{locator}/versionedSettings/tokens | Get Versioned Settings Tokens.
[**load_settings_from_vcs**](VersionedSettingsApi.md#load_settings_from_vcs) | **POST** /app/rest/projects/{locator}/versionedSettings/loadSettings | Perform Versioned Settings action: Load Setting from VCS.
[**set_versioned_settings_config**](VersionedSettingsApi.md#set_versioned_settings_config) | **PUT** /app/rest/projects/{locator}/versionedSettings/config | Set Verseioned Settings config.
[**set_versioned_settings_config_parameter**](VersionedSettingsApi.md#set_versioned_settings_config_parameter) | **PUT** /app/rest/projects/{locator}/versionedSettings/config/parameters/{name} | Set Versioned Settings config parameter value.
[**set_versioned_settings_context_parameters**](VersionedSettingsApi.md#set_versioned_settings_context_parameters) | **PUT** /app/rest/projects/{locator}/versionedSettings/contextParameters | Set Versioned Settings Context Parameters.



## add_versioned_settings_tokens

> models::VersionedSettingsTokens add_versioned_settings_tokens(locator, body)
Add Versioned Settings Tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**body** | Option<[**VersionedSettingsTokens**](VersionedSettingsTokens.md)> |  |  |

### Return type

[**models::VersionedSettingsTokens**](versionedSettingsTokens.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_for_versioned_settings_changes

> check_for_versioned_settings_changes(locator)
Check for changes in Versioned Settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## commit_current_settings

> commit_current_settings(locator)
Perform Versioned Settings action: Commit current settings to VCS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_versioned_settings_config_parameter

> delete_versioned_settings_config_parameter(locator, name)
Delete Versioned Settings config parameter value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_versioned_settings_tokens

> models::VersionedSettingsTokens delete_versioned_settings_tokens(locator, body)
Delete Versioned Settings Tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**body** | Option<[**VersionedSettingsTokens**](VersionedSettingsTokens.md)> |  |  |

### Return type

[**models::VersionedSettingsTokens**](versionedSettingsTokens.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versioned_settings_config

> models::VersionedSettingsConfig get_versioned_settings_config(locator, fields)
Get Versioned Settings config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::VersionedSettingsConfig**](versionedSettingsConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versioned_settings_config_parameter

> String get_versioned_settings_config_parameter(locator, name)
Get Versioned Settings config parameter value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versioned_settings_context_parameters

> models::VersionedSettingsContextParameters get_versioned_settings_context_parameters(locator)
Get Versioned Settings Context Parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |

### Return type

[**models::VersionedSettingsContextParameters**](versionedSettingsContextParameters.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versioned_settings_projects_to_load

> models::Projects get_versioned_settings_projects_to_load(locator, fields)
Get a list of projects that are affected by Load Settings from VCS action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Projects**](projects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versioned_settings_status

> models::VersionedSettingsStatus get_versioned_settings_status(locator, fields)
Get current status of Versioned Settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::VersionedSettingsStatus**](versionedSettingsStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versioned_settings_tokens

> models::VersionedSettingsTokens get_versioned_settings_tokens(locator, status)
Get Versioned Settings Tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**status** | Option<**String**> |  |  |

### Return type

[**models::VersionedSettingsTokens**](versionedSettingsTokens.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## load_settings_from_vcs

> models::Projects load_settings_from_vcs(locator, fields)
Perform Versioned Settings action: Load Setting from VCS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Projects**](projects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_versioned_settings_config

> models::VersionedSettingsConfig set_versioned_settings_config(locator, fields, body)
Set Verseioned Settings config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**VersionedSettingsConfig**](VersionedSettingsConfig.md)> |  |  |

### Return type

[**models::VersionedSettingsConfig**](versionedSettingsConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_versioned_settings_config_parameter

> String set_versioned_settings_config_parameter(locator, name, body)
Set Versioned Settings config parameter value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_versioned_settings_context_parameters

> models::VersionedSettingsContextParameters set_versioned_settings_context_parameters(locator, body)
Set Versioned Settings Context Parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**body** | Option<[**VersionedSettingsContextParameters**](VersionedSettingsContextParameters.md)> |  |  |

### Return type

[**models::VersionedSettingsContextParameters**](versionedSettingsContextParameters.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

