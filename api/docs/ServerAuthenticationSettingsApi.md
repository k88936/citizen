# \ServerAuthenticationSettingsApi

All URIs are relative to *http://teamcity.k88936.top*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_auth_settings**](ServerAuthenticationSettingsApi.md#get_auth_settings) | **GET** /app/rest/server/authSettings | Get authentication settings.
[**set_auth_settings**](ServerAuthenticationSettingsApi.md#set_auth_settings) | **PUT** /app/rest/server/authSettings | Set authentication settings.



## get_auth_settings

> models::ServerAuthSettings get_auth_settings()
Get authentication settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerAuthSettings**](serverAuthSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_auth_settings

> models::ServerAuthSettings set_auth_settings(body)
Set authentication settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ServerAuthSettings**](ServerAuthSettings.md)> |  |  |

### Return type

[**models::ServerAuthSettings**](serverAuthSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

