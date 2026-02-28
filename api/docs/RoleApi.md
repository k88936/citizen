# \RoleApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_included_role**](RoleApi.md#add_included_role) | **PUT** /app/rest/roles/id:{roleId}/included/{includedId} | Add an included role.
[**add_permission**](RoleApi.md#add_permission) | **PUT** /app/rest/roles/id:{roleId}/permissions/{permissionId} | Add a permission to a role.
[**create_role**](RoleApi.md#create_role) | **POST** /app/rest/roles | Create a new role.
[**delete_role**](RoleApi.md#delete_role) | **DELETE** /app/rest/roles/id:{id} | Delete a role matching the id.
[**get_role**](RoleApi.md#get_role) | **GET** /app/rest/roles/id:{id} | Get a role with specified id.
[**get_roles**](RoleApi.md#get_roles) | **GET** /app/rest/roles | Get all roles.
[**remove_included_role**](RoleApi.md#remove_included_role) | **DELETE** /app/rest/roles/id:{roleId}/included/{includedId} | Remove an included role.
[**remove_permission**](RoleApi.md#remove_permission) | **DELETE** /app/rest/roles/id:{roleId}/permissions/{permissionId} | Remove a permission from a role.



## add_included_role

> models::Role add_included_role(role_id, included_id, fields)
Add an included role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**included_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_permission

> models::Role add_permission(role_id, permission_id, fields)
Add a permission to a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**permission_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> models::Role create_role(fields, body)
Create a new role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Role**](Role.md)> |  |  |

### Return type

[**models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role

> delete_role(id)
Delete a role matching the id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> models::Role get_role(id, fields)
Get a role with specified id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_roles

> models::Roles get_roles(fields)
Get all roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |

### Return type

[**models::Roles**](roles.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_included_role

> models::Role remove_included_role(role_id, included_id, fields)
Remove an included role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**included_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_permission

> models::Role remove_permission(role_id, permission_id, fields)
Remove a permission from a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** |  | [required] |
**permission_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Role**](role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

