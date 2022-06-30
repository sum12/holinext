# PublicHolidayV3Dto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | Option<[**String**](string.md)> | The date | [optional]
**local_name** | Option<**String**> | Local name | [optional]
**name** | Option<**String**> | English name | [optional]
**country_code** | Option<**String**> | ISO 3166-1 alpha-2 | [optional]
**fixed** | Option<**bool**> | Is this public holiday every year on the same date | [optional]
**global** | Option<**bool**> | Is this public holiday in every county (federal state) | [optional]
**counties** | Option<**Vec<String>**> | ISO-3166-2 - Federal states | [optional]
**launch_year** | Option<**i32**> | The launch year of the public holiday | [optional]
**types** | Option<[**Vec<crate::models::PublicHolidayType>**](PublicHolidayType.md)> | A list of types the public holiday it is valid | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


