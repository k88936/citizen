# \UserApi

All URIs are relative to *http://teamcity.k88936.top*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_role_to_user**](UserApi.md#add_role_to_user) | **POST** /app/rest/users/{userLocator}/roles | Add a role to the matching user.
[**add_role_to_user_at_scope**](UserApi.md#add_role_to_user_at_scope) | **PUT** /app/rest/users/{userLocator}/roles/{roleId}/{scope} | Add a role with the specific scope to the matching user.
[**add_user**](UserApi.md#add_user) | **POST** /app/rest/users | Create a new user.
[**add_user_token**](UserApi.md#add_user_token) | **POST** /app/rest/users/{userLocator}/tokens | Create a new authentication token for the matching user.
[**delete_user**](UserApi.md#delete_user) | **DELETE** /app/rest/users/{userLocator} | Delete user matching the locator.
[**delete_user_field**](UserApi.md#delete_user_field) | **DELETE** /app/rest/users/{userLocator}/{field} | Remove a property of the matching user.
[**delete_user_token**](UserApi.md#delete_user_token) | **DELETE** /app/rest/users/{userLocator}/tokens/{name} | Remove an authentication token from the matching user.
[**get_all_user_groups**](UserApi.md#get_all_user_groups) | **GET** /app/rest/users/{userLocator}/groups | Get all groups of the matching user.
[**get_all_user_roles**](UserApi.md#get_all_user_roles) | **GET** /app/rest/users/{userLocator}/roles | Get all user roles of the matching user.
[**get_all_users**](UserApi.md#get_all_users) | **GET** /app/rest/users | Get all users.
[**get_user**](UserApi.md#get_user) | **GET** /app/rest/users/{userLocator} | Get user matching the locator.
[**get_user_field**](UserApi.md#get_user_field) | **GET** /app/rest/users/{userLocator}/{field} | Get a field of the matching user.
[**get_user_group**](UserApi.md#get_user_group) | **GET** /app/rest/users/{userLocator}/groups/{groupLocator} | Get a user group of the matching user.
[**get_user_permissions**](UserApi.md#get_user_permissions) | **GET** /app/rest/users/{userLocator}/permissions | Get all permissions effective for the matching user.
[**get_user_properties**](UserApi.md#get_user_properties) | **GET** /app/rest/users/{userLocator}/properties | Get all properties of the matching user.
[**get_user_property**](UserApi.md#get_user_property) | **GET** /app/rest/users/{userLocator}/properties/{name} | Get a property of the matching user.
[**get_user_roles_at_scope**](UserApi.md#get_user_roles_at_scope) | **GET** /app/rest/users/{userLocator}/roles/{roleId}/{scope} | Get a user role with the specific scope from the matching user.
[**get_user_tokens**](UserApi.md#get_user_tokens) | **GET** /app/rest/users/{userLocator}/tokens | Get all authentication tokens of the matching user.
[**logout_user**](UserApi.md#logout_user) | **POST** /app/rest/users/{userLocator}/logout | Terminate all current sessions of the matching user.
[**remove_user_from_group**](UserApi.md#remove_user_from_group) | **DELETE** /app/rest/users/{userLocator}/groups/{groupLocator} | Remove the matching user from the specific group.
[**remove_user_property**](UserApi.md#remove_user_property) | **DELETE** /app/rest/users/{userLocator}/properties/{name} | Remove a property of the matching user.
[**remove_user_remember_me**](UserApi.md#remove_user_remember_me) | **DELETE** /app/rest/users/{userLocator}/debug/rememberMe | Remove the RememberMe data of the matching user.
[**remove_user_role_at_scope**](UserApi.md#remove_user_role_at_scope) | **DELETE** /app/rest/users/{userLocator}/roles/{roleId}/{scope} | Remove a role with the specific scope from the matching user.
[**replace_user**](UserApi.md#replace_user) | **PUT** /app/rest/users/{userLocator} | Update user matching the locator.
[**set_user_field**](UserApi.md#set_user_field) | **PUT** /app/rest/users/{userLocator}/{field} | Update a field of the matching user.
[**set_user_groups**](UserApi.md#set_user_groups) | **PUT** /app/rest/users/{userLocator}/groups | Update groups of the matching user.
[**set_user_property**](UserApi.md#set_user_property) | **PUT** /app/rest/users/{userLocator}/properties/{name} | Update a property of the matching user.
[**set_user_roles**](UserApi.md#set_user_roles) | **PUT** /app/rest/users/{userLocator}/roles | Update user roles of the matching user.



## add_role_to_user

> models::RoleAssignment add_role_to_user(user_locator, body)
Add a role to the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**body** | Option<[**RoleAssignment**](RoleAssignment.md)> |  |  |

### Return type

[**models::RoleAssignment**](roleAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_role_to_user_at_scope

> models::RoleAssignment add_role_to_user_at_scope(user_locator, role_id, scope)
Add a role with the specific scope to the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## add_user

> models::User add_user(fields, body)
Create a new user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**User**](User.md)> |  |  |

### Return type

[**models::User**](user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_token

> models::Token add_user_token(user_locator, fields, body)
Create a new authentication token for the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Token**](Token.md)> |  |  |

### Return type

[**models::Token**](token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_locator)
Delete user matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_field

> delete_user_field(user_locator, field)
Remove a property of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_token

> delete_user_token(user_locator, name)
Remove an authentication token from the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_user_groups

> models::Groups get_all_user_groups(user_locator, fields)
Get all groups of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Groups**](groups.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_user_roles

> models::RoleAssignments get_all_user_roles(user_locator)
Get all user roles of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |

### Return type

[**models::RoleAssignments**](roleAssignments.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_users

> models::Users get_all_users(locator, fields)
Get all users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Users**](users.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::User get_user(user_locator, fields)
Get user matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::User**](user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_field

> String get_user_field(user_locator, field)
Get a field of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_group

> models::Group get_user_group(user_locator, group_locator, fields)
Get a user group of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## get_user_permissions

> models::PermissionAssignments get_user_permissions(user_locator, locator, fields)
Get all permissions effective for the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::PermissionAssignments**](permissionAssignments.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_properties

> models::Properties get_user_properties(user_locator, fields)
Get all properties of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_property

> String get_user_property(user_locator, name)
Get a property of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_roles_at_scope

> models::RoleAssignment get_user_roles_at_scope(user_locator, role_id, scope)
Get a user role with the specific scope from the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## get_user_tokens

> models::Tokens get_user_tokens(user_locator, fields)
Get all authentication tokens of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Tokens**](tokens.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_user

> logout_user(user_locator)
Terminate all current sessions of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_group

> remove_user_from_group(user_locator, group_locator, fields)
Remove the matching user from the specific group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**group_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_property

> remove_user_property(user_locator, name)
Remove a property of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_remember_me

> remove_user_remember_me(user_locator)
Remove the RememberMe data of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_role_at_scope

> remove_user_role_at_scope(user_locator, role_id, scope)
Remove a role with the specific scope from the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## replace_user

> models::User replace_user(user_locator, fields, body)
Update user matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**User**](User.md)> |  |  |

### Return type

[**models::User**](user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_field

> String set_user_field(user_locator, field, body)
Update a field of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## set_user_groups

> models::Groups set_user_groups(user_locator, fields, body)
Update groups of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## set_user_property

> String set_user_property(user_locator, name, body)
Update a property of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
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


## set_user_roles

> models::RoleAssignments set_user_roles(user_locator, body)
Update user roles of the matching user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**body** | Option<[**RoleAssignments**](RoleAssignments.md)> |  |  |

### Return type

[**models::RoleAssignments**](roleAssignments.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

