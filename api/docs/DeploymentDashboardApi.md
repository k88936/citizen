# \DeploymentDashboardApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_dashboard**](DeploymentDashboardApi.md#create_dashboard) | **POST** /app/rest/deploymentDashboards | Create a new deployment dashboard.
[**create_instance**](DeploymentDashboardApi.md#create_instance) | **POST** /app/rest/deploymentDashboards/{deploymentDashboardLocator}/instances | Create a new deployment instance.
[**delete_dashboard**](DeploymentDashboardApi.md#delete_dashboard) | **DELETE** /app/rest/deploymentDashboards/{deploymentDashboardLocator} | Delete the deployment dashboard matching the locator.
[**delete_instance**](DeploymentDashboardApi.md#delete_instance) | **DELETE** /app/rest/deploymentDashboards/{deploymentDashboardLocator}/instances/{deploymentInstanceLocator} | Delete the deployment instance matching the locator.
[**get_all_dashboards**](DeploymentDashboardApi.md#get_all_dashboards) | **GET** /app/rest/deploymentDashboards | Get all deployment dashboards.
[**get_dashboard**](DeploymentDashboardApi.md#get_dashboard) | **GET** /app/rest/deploymentDashboards/{deploymentDashboardLocator} | Get the deployment dashboard matching the locator.
[**get_instance**](DeploymentDashboardApi.md#get_instance) | **GET** /app/rest/deploymentDashboards/{deploymentDashboardLocator}/instances/{deploymentInstanceLocator} | Get the deployment instance matching the locator.
[**get_instances**](DeploymentDashboardApi.md#get_instances) | **GET** /app/rest/deploymentDashboards/{deploymentDashboardLocator}/instances | Get deployment instances for a given deployment dashboard.
[**report_new_deployment_for_instance**](DeploymentDashboardApi.md#report_new_deployment_for_instance) | **POST** /app/rest/deploymentDashboards/{deploymentDashboardLocator}/instances/{deploymentInstanceLocator} | Report a new deployment for instance.



## create_dashboard

> models::DeploymentDashboard create_dashboard(body)
Create a new deployment dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**DeploymentDashboard**](DeploymentDashboard.md)> |  |  |

### Return type

[**models::DeploymentDashboard**](deploymentDashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> models::DeploymentInstance create_instance(deployment_dashboard_locator, body)
Create a new deployment instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_dashboard_locator** | **String** |  | [required] |
**body** | Option<[**DeploymentInstance**](DeploymentInstance.md)> |  |  |

### Return type

[**models::DeploymentInstance**](deploymentInstance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dashboard

> delete_dashboard(deployment_dashboard_locator)
Delete the deployment dashboard matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_dashboard_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> delete_instance(deployment_dashboard_locator, deployment_instance_locator)
Delete the deployment instance matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_dashboard_locator** | **String** |  | [required] |
**deployment_instance_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_dashboards

> models::DeploymentDashboards get_all_dashboards(locator, fields)
Get all deployment dashboards.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::DeploymentDashboards**](deploymentDashboards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard

> models::DeploymentDashboard get_dashboard(deployment_dashboard_locator, fields)
Get the deployment dashboard matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_dashboard_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::DeploymentDashboard**](deploymentDashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance

> models::DeploymentInstance get_instance(deployment_dashboard_locator, deployment_instance_locator, fields)
Get the deployment instance matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_dashboard_locator** | **String** |  | [required] |
**deployment_instance_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::DeploymentInstance**](deploymentInstance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instances

> models::DeploymentInstances get_instances(deployment_dashboard_locator, locator, fields)
Get deployment instances for a given deployment dashboard.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_dashboard_locator** | **String** |  | [required] |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::DeploymentInstances**](deploymentInstances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## report_new_deployment_for_instance

> models::DeploymentInstance report_new_deployment_for_instance(deployment_dashboard_locator, deployment_instance_locator, body)
Report a new deployment for instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_dashboard_locator** | **String** |  | [required] |
**deployment_instance_locator** | **String** |  | [required] |
**body** | Option<[**DeploymentStateEntry**](DeploymentStateEntry.md)> |  |  |

### Return type

[**models::DeploymentInstance**](deploymentInstance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

