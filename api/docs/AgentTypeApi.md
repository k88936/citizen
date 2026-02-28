# \AgentTypeApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_agent_type**](AgentTypeApi.md#get_agent_type) | **GET** /app/rest/agentTypes/{agentTypeLocator} | Get agent type matching the locator.



## get_agent_type

> models::AgentType get_agent_type(agent_type_locator, fields)
Get agent type matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_type_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::AgentType**](agentType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

