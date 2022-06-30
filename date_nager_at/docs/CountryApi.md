# \CountryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**country_available_countries**](CountryApi.md#country_available_countries) | **GET** /api/v3/AvailableCountries | Get all available countries
[**country_country_info**](CountryApi.md#country_country_info) | **GET** /api/v3/CountryInfo/{countryCode} | Get country info for the given country



## country_available_countries

> Vec<crate::models::CountryV3Dto> country_available_countries()
Get all available countries

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CountryV3Dto>**](CountryV3Dto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## country_country_info

> crate::models::CountryInfoDto country_country_info(country_code)
Get country info for the given country

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** |  | [required] |

### Return type

[**crate::models::CountryInfoDto**](CountryInfoDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

