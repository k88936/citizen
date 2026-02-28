# \ProjectApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_agent_pools_project**](ProjectApi.md#add_agent_pools_project) | **POST** /app/rest/projects/{projectLocator}/agentPools | Assign the matching project to the agent pool.
[**add_build_type**](ProjectApi.md#add_build_type) | **POST** /app/rest/projects/{projectLocator}/buildTypes | Add a build configuration to the matching project.
[**add_feature**](ProjectApi.md#add_feature) | **POST** /app/rest/projects/{projectLocator}/projectFeatures | Add a feature.
[**add_project**](ProjectApi.md#add_project) | **POST** /app/rest/projects | Create a new project.
[**add_secure_token**](ProjectApi.md#add_secure_token) | **POST** /app/rest/projects/{projectLocator}/secure/tokens | Creates a new [secure token](https://www.jetbrains.com/help/teamcity/storing-project-settings-in-version-control.html#Managing+Tokens) to store the sensitive value passed in the request body. Returns the scrambled value that is the new token name. This operation is available only for users with the EDIT_PROJECT permission (included in the Project Administrator role by default).
[**add_template**](ProjectApi.md#add_template) | **POST** /app/rest/projects/{projectLocator}/templates | Add a build configuration template to the matching project.
[**create_build_parameter**](ProjectApi.md#create_build_parameter) | **POST** /app/rest/projects/{projectLocator}/parameters | Create a build parameter.
[**delete_build_parameter**](ProjectApi.md#delete_build_parameter) | **DELETE** /app/rest/projects/{projectLocator}/parameters/{name} | Delete build parameter.
[**delete_build_parameters**](ProjectApi.md#delete_build_parameters) | **DELETE** /app/rest/projects/{projectLocator}/parameters | Delete all build parameters.
[**delete_feature**](ProjectApi.md#delete_feature) | **DELETE** /app/rest/projects/{projectLocator}/projectFeatures/{featureLocator} | Delete a matching feature.
[**delete_project**](ProjectApi.md#delete_project) | **DELETE** /app/rest/projects/{projectLocator} | Delete project matching the locator.
[**get_agent_pools_project**](ProjectApi.md#get_agent_pools_project) | **GET** /app/rest/projects/{projectLocator}/agentPools | Get agent pools appointed to the matching project.
[**get_all_branches**](ProjectApi.md#get_all_branches) | **GET** /app/rest/projects/{projectLocator}/branches | Get all branches of the matching project.
[**get_all_build_types_ordered**](ProjectApi.md#get_all_build_types_ordered) | **GET** /app/rest/projects/{projectLocator}/order/buildTypes | Get all build configurations from the matching project, with custom ordering applied.
[**get_all_projects**](ProjectApi.md#get_all_projects) | **GET** /app/rest/projects | Get all projects.
[**get_all_subprojects_ordered**](ProjectApi.md#get_all_subprojects_ordered) | **GET** /app/rest/projects/{projectLocator}/order/projects | Get all subprojects of the matching project, with custom ordering applied.
[**get_build_parameter**](ProjectApi.md#get_build_parameter) | **GET** /app/rest/projects/{projectLocator}/parameters/{name} | Get build parameter.
[**get_build_parameter_specification**](ProjectApi.md#get_build_parameter_specification) | **GET** /app/rest/projects/{projectLocator}/parameters/{name}/type/rawValue | Get build parameter specification.
[**get_build_parameter_type**](ProjectApi.md#get_build_parameter_type) | **GET** /app/rest/projects/{projectLocator}/parameters/{name}/type | Get type of build parameter.
[**get_build_parameter_value**](ProjectApi.md#get_build_parameter_value) | **GET** /app/rest/projects/{projectLocator}/parameters/{name}/value | Get value of build parameter.
[**get_build_parameters**](ProjectApi.md#get_build_parameters) | **GET** /app/rest/projects/{projectLocator}/parameters | Get build parameters.
[**get_default_template**](ProjectApi.md#get_default_template) | **GET** /app/rest/projects/{projectLocator}/defaultTemplate | Get the default template of the matching project.
[**get_default_value_sets**](ProjectApi.md#get_default_value_sets) | **GET** /app/rest/projects/{projectLocator}/defaultValueSets | getDefaultValueSets
[**get_deployment_dashboard_in_project**](ProjectApi.md#get_deployment_dashboard_in_project) | **GET** /app/rest/projects/{projectLocator}/deploymentDashboards/{dashboardLocator} | getDeploymentDashboardInProject
[**get_deployment_dashboards_in_project**](ProjectApi.md#get_deployment_dashboards_in_project) | **GET** /app/rest/projects/{projectLocator}/deploymentDashboards | getDeploymentDashboardsInProjet
[**get_feature**](ProjectApi.md#get_feature) | **GET** /app/rest/projects/{projectLocator}/projectFeatures/{featureLocator} | Get a matching feature.
[**get_features**](ProjectApi.md#get_features) | **GET** /app/rest/projects/{projectLocator}/projectFeatures | Get all features.
[**get_project**](ProjectApi.md#get_project) | **GET** /app/rest/projects/{projectLocator} | Get project matching the locator.
[**get_project_field**](ProjectApi.md#get_project_field) | **GET** /app/rest/projects/{projectLocator}/{field} | Get a field of the matching project.
[**get_project_parent_project**](ProjectApi.md#get_project_parent_project) | **GET** /app/rest/projects/{projectLocator}/parentProject | Get the parent project of the matching project.
[**get_project_settings_file**](ProjectApi.md#get_project_settings_file) | **GET** /app/rest/projects/{projectLocator}/settingsFile | Get the settings file of the matching build configuration.
[**get_project_templates**](ProjectApi.md#get_project_templates) | **GET** /app/rest/projects/{projectLocator}/templates | Get all templates of the matching project.
[**get_secure_value**](ProjectApi.md#get_secure_value) | **GET** /app/rest/projects/{projectLocator}/secure/values/{token} | Returns the value of the given [secure token](https://www.jetbrains.com/help/teamcity/storing-project-settings-in-version-control.html#Managing+Tokens).This operation is available only for users with the CHANGE_SERVER_SETTINGS permission (included only in System Administrator role by default).
[**remove_default_template**](ProjectApi.md#remove_default_template) | **DELETE** /app/rest/projects/{projectLocator}/defaultTemplate | Remove the default template from the matching project.
[**remove_project_from_agent_pool**](ProjectApi.md#remove_project_from_agent_pool) | **DELETE** /app/rest/projects/{projectLocator}/agentPools/{agentPoolLocator} | Unassign a project from the matching agent pool.
[**set_agent_pools_project**](ProjectApi.md#set_agent_pools_project) | **PUT** /app/rest/projects/{projectLocator}/agentPools | Update agent pools apppointed to the matching project.
[**set_build_types_order**](ProjectApi.md#set_build_types_order) | **PUT** /app/rest/projects/{projectLocator}/order/buildTypes | Update custom ordering of build configurations of the matching project.
[**set_default_template**](ProjectApi.md#set_default_template) | **PUT** /app/rest/projects/{projectLocator}/defaultTemplate | Update the default template of the matching project.
[**set_parent_project**](ProjectApi.md#set_parent_project) | **PUT** /app/rest/projects/{projectLocator}/parentProject | Update the parent project of the matching project.
[**set_project_field**](ProjectApi.md#set_project_field) | **PUT** /app/rest/projects/{projectLocator}/{field} | Update a field of the matching project.
[**set_subprojects_order**](ProjectApi.md#set_subprojects_order) | **PUT** /app/rest/projects/{projectLocator}/order/projects | Update custom ordering of subprojects of the matching project.
[**update_build_parameter**](ProjectApi.md#update_build_parameter) | **PUT** /app/rest/projects/{projectLocator}/parameters/{name} | Update build parameter.
[**update_build_parameter_specification**](ProjectApi.md#update_build_parameter_specification) | **PUT** /app/rest/projects/{projectLocator}/parameters/{name}/type/rawValue | Update build parameter specification.
[**update_build_parameter_type**](ProjectApi.md#update_build_parameter_type) | **PUT** /app/rest/projects/{projectLocator}/parameters/{name}/type | Update type of build parameter.
[**update_build_parameter_value**](ProjectApi.md#update_build_parameter_value) | **PUT** /app/rest/projects/{projectLocator}/parameters/{name}/value | Update value of build parameter.
[**update_build_parameters**](ProjectApi.md#update_build_parameters) | **PUT** /app/rest/projects/{projectLocator}/parameters | Update build parameters.
[**update_feature**](ProjectApi.md#update_feature) | **PUT** /app/rest/projects/{projectLocator}/projectFeatures/{featureLocator} | Update a matching feature.
[**update_features**](ProjectApi.md#update_features) | **PUT** /app/rest/projects/{projectLocator}/projectFeatures | Update all features.



## add_agent_pools_project

> models::AgentPool add_agent_pools_project(project_locator, body)
Assign the matching project to the agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**body** | Option<[**AgentPool**](AgentPool.md)> |  |  |

### Return type

[**models::AgentPool**](agentPool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_build_type

> models::BuildType add_build_type(project_locator, fields, body)
Add a build configuration to the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**NewBuildTypeDescription**](NewBuildTypeDescription.md)> |  |  |

### Return type

[**models::BuildType**](buildType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_feature

> serde_json::Value add_feature(project_locator, fields, body)
Add a feature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**ProjectFeature**](ProjectFeature.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_project

> models::Project add_project(body)
Create a new project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**NewProjectDescription**](NewProjectDescription.md)> |  |  |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_secure_token

> String add_secure_token(project_locator, body)
Creates a new [secure token](https://www.jetbrains.com/help/teamcity/storing-project-settings-in-version-control.html#Managing+Tokens) to store the sensitive value passed in the request body. Returns the scrambled value that is the new token name. This operation is available only for users with the EDIT_PROJECT permission (included in the Project Administrator role by default).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_template

> models::BuildType add_template(project_locator, fields, body)
Add a build configuration template to the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**NewBuildTypeDescription**](NewBuildTypeDescription.md)> |  |  |

### Return type

[**models::BuildType**](buildType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_build_parameter

> models::Property create_build_parameter(project_locator, fields, body)
Create a build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Property**](Property.md)> |  |  |

### Return type

[**models::Property**](property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build_parameter

> delete_build_parameter(name, project_locator)
Delete build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_build_parameters

> delete_build_parameters(project_locator)
Delete all build parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feature

> delete_feature(feature_locator, project_locator)
Delete a matching feature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_locator** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_project

> delete_project(project_locator)
Delete project matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_agent_pools_project

> models::AgentPools get_agent_pools_project(project_locator, fields)
Get agent pools appointed to the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::AgentPools**](agentPools.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_branches

> models::Branches get_all_branches(project_locator, locator, fields)
Get all branches of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Branches**](branches.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_build_types_ordered

> models::BuildTypes get_all_build_types_ordered(project_locator, field)
Get all build configurations from the matching project, with custom ordering applied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**field** | Option<**String**> |  |  |

### Return type

[**models::BuildTypes**](buildTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_projects

> models::Projects get_all_projects(locator, fields)
Get all projects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Projects**](projects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_subprojects_ordered

> models::Projects get_all_subprojects_ordered(project_locator, field)
Get all subprojects of the matching project, with custom ordering applied.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**field** | Option<**String**> |  |  |

### Return type

[**models::Projects**](projects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameter

> models::Property get_build_parameter(name, project_locator, fields)
Get build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Property**](property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameter_specification

> String get_build_parameter_specification(name, project_locator)
Get build parameter specification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameter_type

> models::Type get_build_parameter_type(name, project_locator)
Get type of build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |

### Return type

[**models::Type**](type.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameter_value

> String get_build_parameter_value(name, project_locator)
Get value of build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_parameters

> models::Properties get_build_parameters(project_locator, locator, fields)
Get build parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Properties**](properties.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_template

> models::BuildType get_default_template(project_locator, fields)
Get the default template of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::BuildType**](buildType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_value_sets

> models::TypedValueSets get_default_value_sets(project_locator, fields)
getDefaultValueSets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::TypedValueSets**](typedValueSets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_dashboard_in_project

> models::DeploymentDashboard get_deployment_dashboard_in_project(project_locator, dashboard_locator, fields)
getDeploymentDashboardInProject

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**dashboard_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::DeploymentDashboard**](deploymentDashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_dashboards_in_project

> models::DeploymentDashboards get_deployment_dashboards_in_project(project_locator, fields)
getDeploymentDashboardsInProjet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::DeploymentDashboards**](deploymentDashboards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature

> serde_json::Value get_feature(feature_locator, project_locator, fields)
Get a matching feature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_locator** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_features

> serde_json::Value get_features(project_locator, locator, fields)
Get all features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project

> models::Project get_project(project_locator, fields)
Get project matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_field

> String get_project_field(project_locator, field)
Get a field of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_parent_project

> models::Project get_project_parent_project(project_locator, fields)
Get the parent project of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_settings_file

> String get_project_settings_file(project_locator)
Get the settings file of the matching build configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_project_templates

> models::BuildTypes get_project_templates(project_locator, fields)
Get all templates of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::BuildTypes**](buildTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_secure_value

> String get_secure_value(project_locator, token)
Returns the value of the given [secure token](https://www.jetbrains.com/help/teamcity/storing-project-settings-in-version-control.html#Managing+Tokens).This operation is available only for users with the CHANGE_SERVER_SETTINGS permission (included only in System Administrator role by default).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**token** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_default_template

> remove_default_template(project_locator, fields)
Remove the default template from the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_project_from_agent_pool

> remove_project_from_agent_pool(project_locator, agent_pool_locator)
Unassign a project from the matching agent pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**agent_pool_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_agent_pools_project

> models::AgentPools set_agent_pools_project(project_locator, fields, body)
Update agent pools apppointed to the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**AgentPools**](AgentPools.md)> |  |  |

### Return type

[**models::AgentPools**](agentPools.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_build_types_order

> models::BuildTypes set_build_types_order(project_locator, field, body)
Update custom ordering of build configurations of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**field** | Option<**String**> |  |  |
**body** | Option<[**BuildTypes**](BuildTypes.md)> |  |  |

### Return type

[**models::BuildTypes**](buildTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_default_template

> models::BuildType set_default_template(project_locator, fields, body)
Update the default template of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**BuildType**](BuildType.md)> |  |  |

### Return type

[**models::BuildType**](buildType.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_parent_project

> models::Project set_parent_project(project_locator, fields, body)
Update the parent project of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Project**](Project.md)> |  |  |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_project_field

> String set_project_field(project_locator, field, body)
Update a field of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
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


## set_subprojects_order

> models::Projects set_subprojects_order(project_locator, field, body)
Update custom ordering of subprojects of the matching project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**field** | Option<**String**> |  |  |
**body** | Option<[**Projects**](Projects.md)> |  |  |

### Return type

[**models::Projects**](projects.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameter

> models::Property update_build_parameter(name, project_locator, fields, body)
Update build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**Property**](Property.md)> |  |  |

### Return type

[**models::Property**](property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameter_specification

> String update_build_parameter_specification(name, project_locator, body)
Update build parameter specification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameter_type

> models::Type update_build_parameter_type(name, project_locator, body)
Update type of build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |
**body** | Option<[**Type**](Type.md)> |  |  |

### Return type

[**models::Type**](type.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameter_value

> String update_build_parameter_value(name, project_locator, body)
Update value of build parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_build_parameters

> models::Properties update_build_parameters(project_locator, fields, body)
Update build parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
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


## update_feature

> serde_json::Value update_feature(feature_locator, project_locator, fields, body)
Update a matching feature.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_locator** | **String** |  | [required] |
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**ProjectFeature**](ProjectFeature.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_features

> serde_json::Value update_features(project_locator, fields, body)
Update all features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |
**body** | Option<[**ProjectFeatures**](ProjectFeatures.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

