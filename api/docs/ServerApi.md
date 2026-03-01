# \ServerApi

All URIs are relative to *http://teamcity.k88936.top*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_license_keys**](ServerApi.md#add_license_keys) | **POST** /app/rest/server/licensingData/licenseKeys | Add license keys.
[**delete_license_key**](ServerApi.md#delete_license_key) | **DELETE** /app/rest/server/licensingData/licenseKeys/{licenseKey} | Delete a license key.
[**download_file_of_server**](ServerApi.md#download_file_of_server) | **GET** /app/rest/server/files/{areaId}/files{path} | Download specific file.
[**get_all_metrics**](ServerApi.md#get_all_metrics) | **GET** /app/rest/server/metrics | Get metrics.
[**get_all_plugins**](ServerApi.md#get_all_plugins) | **GET** /app/rest/server/plugins | Get all plugins.
[**get_backup_status**](ServerApi.md#get_backup_status) | **GET** /app/rest/server/backup | Get the latest backup status.
[**get_cleanup_settings**](ServerApi.md#get_cleanup_settings) | **GET** /app/rest/server/cleanup | Get clean-up settings.
[**get_file_metadata_of_server**](ServerApi.md#get_file_metadata_of_server) | **GET** /app/rest/server/files/{areaId}/metadata{path} | Get metadata of specific file.
[**get_files_list_for_subpath_of_server**](ServerApi.md#get_files_list_for_subpath_of_server) | **GET** /app/rest/server/files/{areaId}/{path} | List files under this path.
[**get_files_list_of_server**](ServerApi.md#get_files_list_of_server) | **GET** /app/rest/server/files/{areaId} | List all files.
[**get_license_key**](ServerApi.md#get_license_key) | **GET** /app/rest/server/licensingData/licenseKeys/{licenseKey} | Get a license key.
[**get_license_keys**](ServerApi.md#get_license_keys) | **GET** /app/rest/server/licensingData/licenseKeys | Get all license keys.
[**get_licensing_data**](ServerApi.md#get_licensing_data) | **GET** /app/rest/server/licensingData | Get the licensing data.
[**get_server_field**](ServerApi.md#get_server_field) | **GET** /app/rest/server/{field} | Get a field of the server info.
[**get_server_info**](ServerApi.md#get_server_info) | **GET** /app/rest/server | Get the server info.
[**get_zipped_file_of_server**](ServerApi.md#get_zipped_file_of_server) | **GET** /app/rest/server/files/{areaId}/archived{path} | Get specific file zipped.
[**set_cleanup_settings**](ServerApi.md#set_cleanup_settings) | **PUT** /app/rest/server/cleanup | Set clean-up settings.
[**start_backup**](ServerApi.md#start_backup) | **POST** /app/rest/server/backup | Start a new backup.



## add_license_keys

> models::LicenseKeys add_license_keys(fields, body)
Add license keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**body** | Option<**String**> |  |  |

### Return type

[**models::LicenseKeys**](licenseKeys.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_license_key

> delete_license_key(license_key)
Delete a license key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_file_of_server

> download_file_of_server(path, area_id)
Download specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**area_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_metrics

> models::Metrics get_all_metrics(fields)
Get metrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |

### Return type

[**models::Metrics**](metrics.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_plugins

> models::Plugins get_all_plugins(fields, locator)
Get all plugins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |

### Return type

[**models::Plugins**](plugins.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backup_status

> String get_backup_status()
Get the latest backup status.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cleanup_settings

> models::Cleanup get_cleanup_settings()
Get clean-up settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Cleanup**](cleanup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_metadata_of_server

> models::File get_file_metadata_of_server(path, area_id, fields)
Get metadata of specific file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**area_id** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::File**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list_for_subpath_of_server

> models::Files get_files_list_for_subpath_of_server(path, area_id, base_path, locator, fields)
List files under this path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**area_id** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Files**](files.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_files_list_of_server

> models::Files get_files_list_of_server(area_id, base_path, locator, fields)
List all files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**area_id** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**fields** | Option<**String**> |  |  |

### Return type

[**models::Files**](files.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_key

> models::LicenseKey get_license_key(license_key, fields)
Get a license key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key** | **String** |  | [required] |
**fields** | Option<**String**> |  |  |

### Return type

[**models::LicenseKey**](licenseKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_keys

> models::LicenseKeys get_license_keys(fields)
Get all license keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |

### Return type

[**models::LicenseKeys**](licenseKeys.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_licensing_data

> models::LicensingData get_licensing_data(fields)
Get the licensing data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |

### Return type

[**models::LicensingData**](licensingData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_field

> String get_server_field(field)
Get a field of the server info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_info

> models::Server get_server_info(fields)
Get the server info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> |  |  |

### Return type

[**models::Server**](server.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zipped_file_of_server

> get_zipped_file_of_server(path, area_id, base_path, locator, name)
Get specific file zipped.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |
**area_id** | **String** |  | [required] |
**base_path** | Option<**String**> |  |  |
**locator** | Option<**String**> |  |  |
**name** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_cleanup_settings

> models::Cleanup set_cleanup_settings(body)
Set clean-up settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Cleanup**](Cleanup.md)> |  |  |

### Return type

[**models::Cleanup**](cleanup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/xml, application/json
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_backup

> String start_backup(file_name, add_timestamp, include_configs, include_database, include_build_logs, include_personal_changes, include_running_builds, include_supplimentary_data)
Start a new backup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_name** | Option<**String**> |  |  |
**add_timestamp** | Option<**bool**> |  |  |
**include_configs** | Option<**bool**> |  |  |
**include_database** | Option<**bool**> |  |  |
**include_build_logs** | Option<**bool**> |  |  |
**include_personal_changes** | Option<**bool**> |  |  |
**include_running_builds** | Option<**bool**> |  |  |
**include_supplimentary_data** | Option<**bool**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

