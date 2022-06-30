# \LongWeekendApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**long_weekend_long_weekend**](LongWeekendApi.md#long_weekend_long_weekend) | **GET** /api/v3/LongWeekend/{year}/{countryCode} | Get long weekends for a given country



## long_weekend_long_weekend

> Vec<crate::models::LongWeekendV3Dto> long_weekend_long_weekend(year, country_code)
Get long weekends for a given country

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |
**country_code** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LongWeekendV3Dto>**](LongWeekendV3Dto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

