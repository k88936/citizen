# \NodeApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_node_responsibility**](NodeApi.md#change_node_responsibility) | **PUT** /app/rest/server/nodes/{nodeLocator}/enabledResponsibilities/{name} | Enables or disables responsibility for a node.
[**get_all_nodes**](NodeApi.md#get_all_nodes) | **GET** /app/rest/server/nodes | Get all TeamCity nodes.
[**get_disabled_responsibilities**](NodeApi.md#get_disabled_responsibilities) | **GET** /app/rest/server/nodes/{nodeLocator}/disabledResponsibilities | Get all effective responsibilities for a node matching the locator.
[**get_effective_responsibilities**](NodeApi.md#get_effective_responsibilities) | **GET** /app/rest/server/nodes/{nodeLocator}/effectiveResponsibilities | Get all effective responsibilities for a node matching the locator.
[**get_enabled_responsibilities**](NodeApi.md#get_enabled_responsibilities) | **GET** /app/rest/server/nodes/{nodeLocator}/enabledResponsibilities | Get all enabled responsibilities for a node matching the locator.
[**get_node**](NodeApi.md#get_node) | **GET** /app/rest/server/nodes/{nodeLocator} | Get a node matching the locator.



## change_node_responsibility

> models::EnabledResponsibilities change_node_responsibility(node_locator, name, body)
Enables or disables responsibility for a node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

[**models::EnabledResponsibilities**](enabledResponsibilities.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_nodes

> models::Nodes get_all_nodes(locator, fields)
Get all TeamCity nodes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Nodes**](nodes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_disabled_responsibilities

> models::DisabledResponsibilities get_disabled_responsibilities(node_locator, fields)
Get all effective responsibilities for a node matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::DisabledResponsibilities**](disabledResponsibilities.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_effective_responsibilities

> models::EffectiveResponsibilities get_effective_responsibilities(node_locator, fields)
Get all effective responsibilities for a node matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::EffectiveResponsibilities**](effectiveResponsibilities.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_enabled_responsibilities

> models::EnabledResponsibilities get_enabled_responsibilities(node_locator, fields)
Get all enabled responsibilities for a node matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::EnabledResponsibilities**](enabledResponsibilities.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_node

> models::Node get_node(node_locator, fields)
Get a node matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**node_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Node**](node.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

