# \ReportApi

All URIs are relative to *http://api.bondora.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**report_generate_report**](ReportApi.md#report_generate_report) | **Post** /api/v1/report | Request to generate specified report type for set period.
[**report_get_public_dataset**](ReportApi.md#report_get_public_dataset) | **Get** /api/v1/publicdataset | Provides daily public dataset of all loan data that is not covered by the data protection laws.
[**report_get_report**](ReportApi.md#report_get_report) | **Get** /api/v1/report/{id} | Get report data for specified report identificator.
[**report_get_report_list**](ReportApi.md#report_get_report_list) | **Get** /api/v1/reports | List of all reports


# **report_generate_report**
> ::models::ApiResultCreateReport report_generate_report(ctx, request)
Request to generate specified report type for set period.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **request** | [**ReportCreateRequest**](ReportCreateRequest.md)|  | 

### Return type

[**::models::ApiResultCreateReport**](ApiResultCreateReport.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **report_get_public_dataset**
> ::models::ApiResultPublicDataset report_get_public_dataset(optional)
Provides daily public dataset of all loan data that is not covered by the data protection laws.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request_loan_ids** | [**Vec&lt;String&gt;**](String.md)| Specific loans to search | 
 **request_countries** | [**Vec&lt;String&gt;**](String.md)| Two letter iso code for country of origin: EE, ES, FI | 
 **request_ratings** | [**Vec&lt;String&gt;**](String.md)| Bondora&#39;s rating: AA, A, B, C, D, E, F, HR | 
 **request_loan_date_from** | **String**| Loan start date from | 
 **request_loan_date_to** | **String**| Loan start date to | 
 **request_page_size** | **i32**| Max items in result, up to 10000 | 
 **request_page_nr** | **i32**| Result page nr | 

### Return type

[**::models::ApiResultPublicDataset**](ApiResultPublicDataset.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **report_get_report**
> ::models::ApiResultReport report_get_report(ctx, id)
Get report data for specified report identificator.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | [**String**](.md)| ReportId | 

### Return type

[**::models::ApiResultReport**](ApiResultReport.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **report_get_report_list**
> ::models::ApiResultReportList report_get_report_list(ctx, )
List of all reports

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ApiResultReportList**](ApiResultReportList.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

