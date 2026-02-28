# \VcsRootApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_vcs_root**](VcsRootApi.md#add_vcs_root) | **POST** /app/rest/vcs-roots | Add a new VCS root.
[**delete_all_vcs_root_properties**](VcsRootApi.md#delete_all_vcs_root_properties) | **DELETE** /app/rest/vcs-roots/{vcsRootLocator}/properties | Delete all properties of the matching VCS root.
[**delete_vcs_root**](VcsRootApi.md#delete_vcs_root) | **DELETE** /app/rest/vcs-roots/{vcsRootLocator} | Remove VCS root matching the locator.
[**delete_vcs_root_property**](VcsRootApi.md#delete_vcs_root_property) | **DELETE** /app/rest/vcs-roots/{vcsRootLocator}/properties/{name} | Delete a property of the matching VCS root.
[**get_all_vcs_root_properties**](VcsRootApi.md#get_all_vcs_root_properties) | **GET** /app/rest/vcs-roots/{vcsRootLocator}/properties | Get all properties of the matching VCS root.
[**get_all_vcs_roots**](VcsRootApi.md#get_all_vcs_roots) | **GET** /app/rest/vcs-roots | Get all VCS roots.
[**get_root_endpoints**](VcsRootApi.md#get_root_endpoints) | **GET** /app/rest/vcs-roots/{vcsRootLocator} | Get root endpoints.
[**get_vcs_root_field**](VcsRootApi.md#get_vcs_root_field) | **GET** /app/rest/vcs-roots/{vcsRootLocator}/{field} | Get a field of the matching VCS root.
[**get_vcs_root_instances**](VcsRootApi.md#get_vcs_root_instances) | **GET** /app/rest/vcs-roots/{vcsRootLocator}/instances | Get all VCS root instances of the matching VCS root.
[**get_vcs_root_property**](VcsRootApi.md#get_vcs_root_property) | **GET** /app/rest/vcs-roots/{vcsRootLocator}/properties/{name} | Get a property on the matching VCS root.
[**get_vcs_root_settings_file**](VcsRootApi.md#get_vcs_root_settings_file) | **GET** /app/rest/vcs-roots/{vcsRootLocator}/settingsFile | Get the settings file of the matching VCS root.
[**set_vcs_root_field**](VcsRootApi.md#set_vcs_root_field) | **PUT** /app/rest/vcs-roots/{vcsRootLocator}/{field} | Update a field of the matching VCS root.
[**set_vcs_root_properties**](VcsRootApi.md#set_vcs_root_properties) | **PUT** /app/rest/vcs-roots/{vcsRootLocator}/properties | Update all properties of the matching VCS root.
[**set_vcs_root_property**](VcsRootApi.md#set_vcs_root_property) | **PUT** /app/rest/vcs-roots/{vcsRootLocator}/properties/{name} | Update a property of the matching VCS root.



## add_vcs_root

> models::VcsRoot add_vcs_root(fields, body)
Add a new VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**VcsRoot**](VcsRoot.md)> |  |  |

### Return type

[**models::VcsRoot**](vcs-root.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_vcs_root_properties

> delete_all_vcs_root_properties(vcs_root_locator)
Delete all properties of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vcs_root

> delete_vcs_root(vcs_root_locator)
Remove VCS root matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vcs_root_property

> delete_vcs_root_property(vcs_root_locator, name)
Delete a property of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_vcs_root_properties

> models::Properties get_all_vcs_root_properties(vcs_root_locator, fields)
Get all properties of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_vcs_roots

> models::VcsRoots get_all_vcs_roots(locator, fields)
Get all VCS roots.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::VcsRoots**](vcs-roots.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_root_endpoints

> models::VcsRoot get_root_endpoints(vcs_root_locator, fields)
Get root endpoints.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::VcsRoot**](vcs-root.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_field

> String get_vcs_root_field(vcs_root_locator, field)
Get a field of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_instances

> models::VcsRootInstances get_vcs_root_instances(vcs_root_locator, fields)
Get all VCS root instances of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::VcsRootInstances**](vcs-root-instances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_property

> String get_vcs_root_property(vcs_root_locator, name)
Get a property on the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vcs_root_settings_file

> String get_vcs_root_settings_file(vcs_root_locator)
Get the settings file of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_vcs_root_field

> String set_vcs_root_field(vcs_root_locator, field, body)
Update a field of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_vcs_root_properties

> models::Properties set_vcs_root_properties(vcs_root_locator, fields, body)
Update all properties of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Properties**](Properties.md)> |  |  |

### Return type

[**models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_vcs_root_property

> String set_vcs_root_property(vcs_root_locator, name, body)
Update a property of the matching VCS root.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**vcs_root_locator** | **String** |  | [required] |
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

