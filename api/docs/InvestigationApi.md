# \InvestigationApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_investigation**](InvestigationApi.md#add_investigation) | **POST** /app/rest/investigations | Create a new investigation.
[**add_multiple_investigations**](InvestigationApi.md#add_multiple_investigations) | **POST** /app/rest/investigations/multiple | Create multiple new investigations.
[**delete_investigation**](InvestigationApi.md#delete_investigation) | **DELETE** /app/rest/investigations/{investigationLocator} | Delete investigation matching the locator.
[**get_all_investigations**](InvestigationApi.md#get_all_investigations) | **GET** /app/rest/investigations | Get all investigations.
[**get_investigation**](InvestigationApi.md#get_investigation) | **GET** /app/rest/investigations/{investigationLocator} | Get investigation matching the locator.
[**replace_investigation**](InvestigationApi.md#replace_investigation) | **PUT** /app/rest/investigations/{investigationLocator} | Update investigation matching the locator.



## add_investigation

> models::Investigation add_investigation(fields, body)
Create a new investigation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Investigation**](Investigation.md)> |  |  |

### Return type

[**models::Investigation**](investigation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_multiple_investigations

> models::Investigations add_multiple_investigations(fields, body)
Create multiple new investigations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Investigations**](Investigations.md)> |  |  |

### Return type

[**models::Investigations**](investigations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_investigation

> delete_investigation(investigation_locator)
Delete investigation matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**investigation_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_investigations

> models::Investigations get_all_investigations(locator, fields)
Get all investigations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Investigations**](investigations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_investigation

> models::Investigation get_investigation(investigation_locator, fields)
Get investigation matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**investigation_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Investigation**](investigation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_investigation

> models::Investigation replace_investigation(investigation_locator, fields, body)
Update investigation matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**investigation_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Investigation**](Investigation.md)> |  |  |

### Return type

[**models::Investigation**](investigation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

