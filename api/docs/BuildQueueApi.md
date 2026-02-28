# \BuildQueueApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_build_to_queue**](BuildQueueApi.md#add_build_to_queue) | **POST** /app/rest/buildQueue | Add a new build to the queue.
[**add_tags_to_build_of_build_queue**](BuildQueueApi.md#add_tags_to_build_of_build_queue) | **POST** /app/rest/buildQueue/{buildLocator}/tags | Add tags to the matching build.
[**approve_queued_build**](BuildQueueApi.md#approve_queued_build) | **POST** /app/rest/buildQueue/{buildLocator}/approve | Approve queued build with approval feature enabled.
[**cancel_queued_build**](BuildQueueApi.md#cancel_queued_build) | **POST** /app/rest/buildQueue/{queuedBuildLocator} | Cancel a queued matching build.
[**delete_all_queued_builds**](BuildQueueApi.md#delete_all_queued_builds) | **DELETE** /app/rest/buildQueue | Delete all queued builds.
[**delete_queued_build**](BuildQueueApi.md#delete_queued_build) | **DELETE** /app/rest/buildQueue/{queuedBuildLocator} | Delete a queued matching build.
[**get_all_queued_builds**](BuildQueueApi.md#get_all_queued_builds) | **GET** /app/rest/buildQueue | Get all queued builds.
[**get_approval_info**](BuildQueueApi.md#get_approval_info) | **GET** /app/rest/buildQueue/{buildLocator}/approvalInfo | Get approval info of a queued matching build.
[**get_compatible_agents_for_build**](BuildQueueApi.md#get_compatible_agents_for_build) | **GET** /app/rest/buildQueue/{queuedBuildLocator}/compatibleAgents | Get compatible agents for a queued matching build.
[**get_queued_build**](BuildQueueApi.md#get_queued_build) | **GET** /app/rest/buildQueue/{queuedBuildLocator} | Get a queued matching build.
[**get_queued_build_position**](BuildQueueApi.md#get_queued_build_position) | **GET** /app/rest/buildQueue/order/{queuePosition} | Get the queue position of a queued matching build.
[**get_queued_build_tags**](BuildQueueApi.md#get_queued_build_tags) | **GET** /app/rest/buildQueue/{buildLocator}/tags | Get tags of the queued matching build.
[**set_queued_build_position**](BuildQueueApi.md#set_queued_build_position) | **PUT** /app/rest/buildQueue/order/{queuePosition} | Update the queue position of a queued matching build.
[**set_queued_builds_order**](BuildQueueApi.md#set_queued_builds_order) | **PUT** /app/rest/buildQueue/order | Update the build queue order.



## add_build_to_queue

> models::Build add_build_to_queue(move_to_top, body)
Add a new build to the queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**move_to_top** | Option<**bool**> |  |  |
**body** | Option<[**Build**](Build.md)> |  |  |

### Return type

[**models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_tags_to_build_of_build_queue

> add_tags_to_build_of_build_queue(build_locator, body)
Add tags to the matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**body** | Option<[**Tags**](Tags.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_queued_build

> models::ApprovalInfo approve_queued_build(build_locator, fields, approve_all, body)
Approve queued build with approval feature enabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**approve_all** | Option<**bool**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

[**models::ApprovalInfo**](approvalInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_queued_build

> models::Build cancel_queued_build(queued_build_locator, body)
Cancel a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queued_build_locator** | **String** |  | [required] |
**body** | Option<[**BuildCancelRequest**](BuildCancelRequest.md)> |  |  |

### Return type

[**models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_queued_builds

> delete_all_queued_builds(locator, fields)
Delete all queued builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_queued_build

> delete_queued_build(queued_build_locator)
Delete a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queued_build_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_queued_builds

> models::Builds get_all_queued_builds(locator, fields)
Get all queued builds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Builds**](builds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approval_info

> models::ApprovalInfo get_approval_info(build_locator, fields)
Get approval info of a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::ApprovalInfo**](approvalInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compatible_agents_for_build

> models::Agents get_compatible_agents_for_build(queued_build_locator, fields)
Get compatible agents for a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queued_build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Agents**](agents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queued_build

> models::Build get_queued_build(queued_build_locator, fields)
Get a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queued_build_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queued_build_position

> models::Build get_queued_build_position(queue_position, fields)
Get the queue position of a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_position** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queued_build_tags

> models::Tags get_queued_build_tags(build_locator, locator, fields)
Get tags of the queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**build_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Tags**](tags.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_queued_build_position

> models::Build set_queued_build_position(queue_position, fields, body)
Update the queue position of a queued matching build.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_position** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Build**](Build.md)> |  |  |

### Return type

[**models::Build**](build.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_queued_builds_order

> models::Builds set_queued_builds_order(fields, body)
Update the build queue order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Builds**](Builds.md)> |  |  |

### Return type

[**models::Builds**](builds.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

