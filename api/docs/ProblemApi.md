# \ProblemApi

All URIs are relative to *http://teamcity.jetbrains.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_build_problems**](ProblemApi.md#get_all_build_problems) | **GET** /app/rest/problems | Get all build problems.
[**get_build_problem**](ProblemApi.md#get_build_problem) | **GET** /app/rest/problems/{problemLocator} | Get a matching build problem.



## get_all_build_problems

> models::Problems get_all_build_problems(locator, fields)
Get all build problems.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Problems**](problems.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_build_problem

> models::Problem get_build_problem(problem_locator, fields)
Get a matching build problem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**problem_locator** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Problem**](problem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

