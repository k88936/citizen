# \GroupApi

All URIs are relative to *http://teamcity.k88936.top*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group**](GroupApi.md#add_group) | **POST** /app/rest/userGroups | Add a new user group.
[**add_role_at_scope_to_group**](GroupApi.md#add_role_at_scope_to_group) | **POST** /app/rest/userGroups/{groupLocator}/roles/{roleId}/{scope} | Add a role with the specific scope to the matching user group.
[**add_role_to_group**](GroupApi.md#add_role_to_group) | **POST** /app/rest/userGroups/{groupLocator}/roles | Add a role to the matching user group.
[**delete_group**](GroupApi.md#delete_group) | **DELETE** /app/rest/userGroups/{groupLocator} | Delete user group matching the locator.
[**get_all_groups**](GroupApi.md#get_all_groups) | **GET** /app/rest/userGroups | Get all user groups.
[**get_group_parent_groups**](GroupApi.md#get_group_parent_groups) | **GET** /app/rest/userGroups/{groupLocator}/parent-groups | Get parent groups of the matching user group.
[**get_group_properties**](GroupApi.md#get_group_properties) | **GET** /app/rest/userGroups/{groupLocator}/properties | Get properties of the matching user group.
[**get_group_property**](GroupApi.md#get_group_property) | **GET** /app/rest/userGroups/{groupLocator}/properties/{name} | Get a property of the matching user group.
[**get_group_role_at_scope**](GroupApi.md#get_group_role_at_scope) | **GET** /app/rest/userGroups/{groupLocator}/roles/{roleId}/{scope} | Get a role with the specific scope of the matching user group.
[**get_group_roles**](GroupApi.md#get_group_roles) | **GET** /app/rest/userGroups/{groupLocator}/roles | Get all roles of the matching user group.
[**get_user_group_of_group**](GroupApi.md#get_user_group_of_group) | **GET** /app/rest/userGroups/{groupLocator} | Get user group matching the locator.
[**remove_group_property**](GroupApi.md#remove_group_property) | **DELETE** /app/rest/userGroups/{groupLocator}/properties/{name} | Remove a property of the matching user group.
[**remove_role_at_scope_from_group**](GroupApi.md#remove_role_at_scope_from_group) | **DELETE** /app/rest/userGroups/{groupLocator}/roles/{roleId}/{scope} | Remove a role with the specific scope from the matching user group.
[**set_group_parent_groups**](GroupApi.md#set_group_parent_groups) | **PUT** /app/rest/userGroups/{groupLocator}/parent-groups | Update parent groups of the matching user group.
[**set_group_property**](GroupApi.md#set_group_property) | **PUT** /app/rest/userGroups/{groupLocator}/properties/{name} | Update a property of the matching user group.
[**set_group_roles**](GroupApi.md#set_group_roles) | **PUT** /app/rest/userGroups/{groupLocator}/roles | Update roles of the matching user group.



## add_group

> models::Group add_group(fields, body)
Add a new user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**Group**](Group.md)> |  |  |

### Return type

[**models::Group**](group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_role_at_scope_to_group

> models::RoleAssignment add_role_at_scope_to_group(group_locator, role_id, scope)
Add a role with the specific scope to the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**scope** | **String** |  | [required] |

### Return type

[**models::RoleAssignment**](roleAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_role_to_group

> models::RoleAssignment add_role_to_group(group_locator, body)
Add a role to the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**body** | Option<[**RoleAssignment**](RoleAssignment.md)> |  |  |

### Return type

[**models::RoleAssignment**](roleAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> delete_group(group_locator)
Delete user group matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_groups

> models::Groups get_all_groups(fields)
Get all user groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |

### Return type

[**models::Groups**](groups.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_parent_groups

> models::Groups get_group_parent_groups(group_locator, fields)
Get parent groups of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Groups**](groups.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_properties

> models::Properties get_group_properties(group_locator, fields)
Get properties of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_property

> String get_group_property(group_locator, name)
Get a property of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_role_at_scope

> models::RoleAssignment get_group_role_at_scope(group_locator, role_id, scope)
Get a role with the specific scope of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**scope** | **String** |  | [required] |

### Return type

[**models::RoleAssignment**](roleAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_roles

> models::RoleAssignments get_group_roles(group_locator)
Get all roles of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |

### Return type

[**models::RoleAssignments**](roleAssignments.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group_of_group

> models::Group get_user_group_of_group(group_locator, fields)
Get user group matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Group**](group.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_property

> remove_group_property(group_locator, name)
Remove a property of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_role_at_scope_from_group

> remove_role_at_scope_from_group(group_locator, role_id, scope)
Remove a role with the specific scope from the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**role_id** | **String** |  | [required] |
**scope** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_parent_groups

> models::Groups set_group_parent_groups(group_locator, fields, body)
Update parent groups of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Groups**](Groups.md)> |  |  |

### Return type

[**models::Groups**](groups.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_property

> String set_group_property(group_locator, name, body)
Update a property of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
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


## set_group_roles

> models::RoleAssignments set_group_roles(group_locator, body)
Update roles of the matching user group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_locator** | **String** |  | [required] |
**body** | Option<[**RoleAssignments**](RoleAssignments.md)> |  |  |

### Return type

[**models::RoleAssignments**](roleAssignments.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

