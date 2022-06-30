# \PublicHolidayApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**public_holiday_is_today_public_holiday**](PublicHolidayApi.md#public_holiday_is_today_public_holiday) | **GET** /api/v3/IsTodayPublicHoliday/{countryCode} | Is today a public holiday
[**public_holiday_next_public_holidays**](PublicHolidayApi.md#public_holiday_next_public_holidays) | **GET** /api/v3/NextPublicHolidays/{countryCode} | Returns the upcoming public holidays for the next 365 days for the given country
[**public_holiday_next_public_holidays_worldwide**](PublicHolidayApi.md#public_holiday_next_public_holidays_worldwide) | **GET** /api/v3/NextPublicHolidaysWorldwide | Returns the upcoming public holidays for the next 7 days
[**public_holiday_public_holidays_v3**](PublicHolidayApi.md#public_holiday_public_holidays_v3) | **GET** /api/v3/PublicHolidays/{year}/{countryCode} | Get public holidays



## public_holiday_is_today_public_holiday

> public_holiday_is_today_public_holiday(country_code, county_code, offset)
Is today a public holiday

The calculation is made on the basis of UTC time to adjust the time please use the offset.<br />  This is a special endpoint for `curl`<br /><br />  200 = Today is a public holiday<br />  204 = Today is not a public holiday<br /><br />  `STATUSCODE=$(curl --silent --output /dev/stderr --write-out \"%{http_code}\" https://date.nager.at/Api/v2/IsTodayPublicHoliday/AT)`<br /><br />  `if [ $STATUSCODE -ne 200 ]; then # error handling; fi`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** |  | [required] |
**county_code** | Option<**String**> |  |  |
**offset** | Option<**i32**> | utc timezone offset |  |[default to 0]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holiday_next_public_holidays

> Vec<crate::models::PublicHolidayV3Dto> public_holiday_next_public_holidays(country_code)
Returns the upcoming public holidays for the next 365 days for the given country

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PublicHolidayV3Dto>**](PublicHolidayV3Dto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holiday_next_public_holidays_worldwide

> Vec<crate::models::PublicHolidayV3Dto> public_holiday_next_public_holidays_worldwide()
Returns the upcoming public holidays for the next 7 days

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PublicHolidayV3Dto>**](PublicHolidayV3Dto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_holiday_public_holidays_v3

> Vec<crate::models::PublicHolidayV3Dto> public_holiday_public_holidays_v3(year, country_code)
Get public holidays

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |
**country_code** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PublicHolidayV3Dto>**](PublicHolidayV3Dto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

