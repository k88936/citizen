# \CloudInstanceApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**forse_terminate_instance**](CloudInstanceApi.md#forse_terminate_instance) | **POST** /app/rest/cloud/instances/{instanceLocator}/actions/forceStop | Terminates existing cloud instance immediately
[**get_all_cloud_images**](CloudInstanceApi.md#get_all_cloud_images) | **GET** /app/rest/cloud/images | Get all cloud images.
[**get_all_cloud_instances**](CloudInstanceApi.md#get_all_cloud_instances) | **GET** /app/rest/cloud/instances | Get all cloud instances.
[**get_all_cloud_profiles**](CloudInstanceApi.md#get_all_cloud_profiles) | **GET** /app/rest/cloud/profiles | Get all cloud profiles.
[**get_cloud_image**](CloudInstanceApi.md#get_cloud_image) | **GET** /app/rest/cloud/images/{imageLocator} | Get cloud image matching the locator.
[**get_cloud_instance**](CloudInstanceApi.md#get_cloud_instance) | **GET** /app/rest/cloud/instances/{instanceLocator} | Get cloud instance matching the locator.
[**get_cloud_profile**](CloudInstanceApi.md#get_cloud_profile) | **GET** /app/rest/cloud/profiles/{profileLocator} | Get cloud profile matching the locator.
[**start_instance**](CloudInstanceApi.md#start_instance) | **POST** /app/rest/cloud/instances | Start a new cloud instance.
[**stop_instance**](CloudInstanceApi.md#stop_instance) | **DELETE** /app/rest/cloud/instances/{instanceLocator} | Stop cloud instance matching the locator.
[**terminate_instance**](CloudInstanceApi.md#terminate_instance) | **POST** /app/rest/cloud/instances/{instanceLocator}/actions/stop | Schedules existing cloud instance for termination



## forse_terminate_instance

> forse_terminate_instance(instance_locator)
Terminates existing cloud instance immediately

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_cloud_images

> models::CloudImages get_all_cloud_images(locator, fields)
Get all cloud images.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::CloudImages**](cloudImages.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_cloud_instances

> models::CloudInstances get_all_cloud_instances(locator, fields)
Get all cloud instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::CloudInstances**](cloudInstances.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_cloud_profiles

> models::CloudProfiles get_all_cloud_profiles(locator, fields)
Get all cloud profiles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::CloudProfiles**](cloudProfiles.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_image

> models::CloudImage get_cloud_image(image_locator, fields)
Get cloud image matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::CloudImage**](cloudImage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_instance

> models::CloudInstance get_cloud_instance(instance_locator, fields)
Get cloud instance matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::CloudInstance**](cloudInstance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cloud_profile

> models::CloudProfile get_cloud_profile(profile_locator, fields)
Get cloud profile matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::CloudProfile**](cloudProfile.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_instance

> start_instance(fields, body)
Start a new cloud instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<[**CloudInstance**](CloudInstance.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_instance

> stop_instance(instance_locator)
Stop cloud instance matching the locator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_instance

> terminate_instance(instance_locator)
Schedules existing cloud instance for termination

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_locator** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

