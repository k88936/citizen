# \TestApi

All URIs are relative to *http://teamcity.k88936.top*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_test**](TestApi.md#get_test) | **GET** /app/rest/tests/{testLocator} | Get a matching test.
[**get_tests**](TestApi.md#get_tests) | **GET** /app/rest/tests | Get all tests.



## get_test

> models::Test get_test(test_locator, fields)
Get a matching test.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Test**](test.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tests

> models::Tests get_tests(locator, fields)
Get all tests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Tests**](tests.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

