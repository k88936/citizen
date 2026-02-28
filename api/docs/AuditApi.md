# \AuditApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_audit_events**](AuditApi.md#get_all_audit_events) | **GET** /app/rest/audit | Get all audit events.
[**get_audit_event**](AuditApi.md#get_audit_event) | **GET** /app/rest/audit/{auditEventLocator} | Get audit event matching the locator.



## get_all_audit_events

> models::AuditEvents get_all_audit_events(locator, fields)
Get all audit events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::AuditEvents**](auditEvents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audit_event

> models::AuditEvent get_audit_event(audit_event_locator, fields)
Get audit event matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**audit_event_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::AuditEvent**](auditEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

