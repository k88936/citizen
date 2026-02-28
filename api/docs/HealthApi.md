# \HealthApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_categories**](HealthApi.md#get_categories) | **GET** /app/rest/health/category | 
[**get_health_items**](HealthApi.md#get_health_items) | **GET** /app/rest/health | 
[**get_single_category**](HealthApi.md#get_single_category) | **GET** /app/rest/health/category/{locator} | 
[**get_single_health_item**](HealthApi.md#get_single_health_item) | **GET** /app/rest/health/{locator} | 



## get_categories

> models::HealthCategories get_categories(locator, fields)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::HealthCategories**](healthCategories.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_health_items

> models::HealthStatusItems get_health_items(locator, fields)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::HealthStatusItems**](healthStatusItems.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_single_category

> models::HealthCategory get_single_category(locator, fields)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::HealthCategory**](healthCategory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_single_health_item

> models::HealthItem get_single_health_item(locator, fields)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::HealthItem**](healthItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

