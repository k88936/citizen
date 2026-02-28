# \GlobalServerSettingsApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_global_settings**](GlobalServerSettingsApi.md#get_global_settings) | **GET** /app/rest/server/globalSettings | Get global settings.
[**set_global_settings**](GlobalServerSettingsApi.md#set_global_settings) | **PUT** /app/rest/server/globalSettings | Set global settings.



## get_global_settings

> models::ServerGlobalSettings get_global_settings()
Get global settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerGlobalSettings**](serverGlobalSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_global_settings

> models::ServerGlobalSettings set_global_settings(body)
Set global settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ServerGlobalSettings**](ServerGlobalSettings.md)> |  |  |

### Return type

[**models::ServerGlobalSettings**](serverGlobalSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

