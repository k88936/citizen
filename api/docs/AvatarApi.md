# \AvatarApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_avatar**](AvatarApi.md#delete_avatar) | **DELETE** /app/rest/avatars/{userLocator} | Delete a users avatar
[**get_avatar**](AvatarApi.md#get_avatar) | **GET** /app/rest/avatars/{userLocator}/{size}/avatar.png | Get a users avatar
[**get_avatar_with_hash**](AvatarApi.md#get_avatar_with_hash) | **GET** /app/rest/avatars/{userLocator}/{size}/avatar.{hash}.png | Get a users avatar
[**put_avatar**](AvatarApi.md#put_avatar) | **PUT** /app/rest/avatars/{userLocator} | Update a users avatar



## delete_avatar

> delete_avatar(user_locator)
Delete a users avatar

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


## get_avatar

> get_avatar(user_locator, size)
Get a users avatar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**size** | **i32** | avatar's size | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_avatar_with_hash

> get_avatar_with_hash(user_locator, size, hash)
Get a users avatar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**size** | **i32** | avatar's size | [required] |
**hash** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_avatar

> put_avatar(user_locator, avatar)
Update a users avatar

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_locator** | **String** |  | [required] |
**avatar** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

