# \AccountApi

All URIs are relative to *http://api.bondora.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_get_active**](AccountApi.md#account_get_active) | **Get** /api/v1/account/investments | Gets list of your investments
[**account_get_balance**](AccountApi.md#account_get_balance) | **Get** /api/v1/account/balance | Gets your account balance information
[**account_get_event_log**](AccountApi.md#account_get_event_log) | **Get** /api/v1/eventlog | Gets events that have been made with this application (related to current access token)


# **account_get_active**
> ::models::ApiResultMyInvestments account_get_active(ctx, optional)
Gets list of your investments

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request_loan_issued_date_from** | **String**| Loan issued start date from | 
 **request_loan_issued_date_to** | **String**| Loan issued start date to | 
 **request_principal_min** | **f64**| Remaining principal amount min | 
 **request_principal_max** | **f64**| Remaining principal amount max | 
 **request_interest_min** | **f64**| Interest rate min | 
 **request_interest_max** | **f64**| Interest rate max | 
 **request_length_max** | **i32**| Loan lenght min | 
 **request_length_min** | **i32**| Loan lenght max | 
 **request_late_principal_amount_min** | **f64**| Principal debt amount min | 
 **request_late_principal_amount_max** | **f64**| Principal debt amount max | 
 **request_debt_occured_on_from** | **String**| Principal debt started date from | 
 **request_debt_occured_on_to** | **String**| Principal debt started date to | 
 **request_debt_occured_on_for_secondary_from** | **String**| Interest debt started date from | 
 **request_debt_occured_on_for_secondary_to** | **String**| Interest debt started date to | 
 **request_defaulted_date_from** | **String**| Defaulted date from | 
 **request_defaulted_date_to** | **String**| Defaulted date to | 
 **request_rescheduled_from** | **String**| Defaulted date from | 
 **request_rescheduled_to** | **String**| Defaulted date to | 
 **request_sold_date_from** | **String**| When it was sold on Secondary market from | 
 **request_sold_date_to** | **String**| When it was sold on Secondary market to | 
 **request_purchase_date_from** | **String**| When you received the investment Auctions/Secondary market from | 
 **request_purchase_date_to** | **String**| When you received the investment Auctions/Secondary market to | 
 **request_next_payment_date_to** | **String**| Next payment date to | 
 **request_next_payment_date_from** | **String**| Next payment date from | 
 **request_last_payment_date_from** | **String**| Last payment date from | 
 **request_last_payment_date_to** | **String**| Last payment date to | 
 **request_countries** | [**Vec&lt;String&gt;**](String.md)| Two letter iso code for country of origin: EE, ES, FI | 
 **request_ratings** | [**Vec&lt;String&gt;**](String.md)| Bondora&#39;s rating: AA, A, B, C, D, E, F, HR | 
 **request_credit_score_min** | **i32**| Minimum credit score | 
 **request_credit_score_max** | **i32**| Maximum credit score | 
 **request_user_name** | **String**| Borrower&#39;s username | 
 **request_loan_status_code** | [**Vec&lt;i32&gt;**](i32.md)| Loan status code  &lt;para&gt;0 Reserved&lt;/para&gt;&lt;para&gt;2 Current&lt;/para&gt;&lt;para&gt;3 Cancelled&lt;/para&gt;&lt;para&gt;100 Overdue&lt;/para&gt;&lt;para&gt;5 60+ days overdue&lt;/para&gt;&lt;para&gt;4 Repaid&lt;/para&gt;&lt;para&gt;8 Released&lt;/para&gt; | 
 **request_income_verification_status** | **i32**| Income verification type | 
 **request_loan_debt_management_stage** | **i32**| Latest debt management stage | 
 **request_loan_debt_management_stage_type** | **i32**| Latest debt management stage type | 
 **request_loan_debt_management_date_active_from** | **String**| Latest debt management date active from | 
 **request_loan_debt_management_date_active_to** | **String**| Latest debt management date active to | 
 **request_auction_bid_type** | **i32**| Auction bid type | 
 **request_sales_status** | **i32**| Second market sale status  &lt;para&gt;NULL All active&lt;/para&gt;&lt;para&gt;0 Bought investments&lt;/para&gt;&lt;para&gt;1 Sold investments&lt;/para&gt;&lt;para&gt;2 Investment is on sale&lt;/para&gt;&lt;para&gt;3 Investment is not on sale&lt;/para&gt; | 
 **request_is_in_repayment** | **bool**| Search only active in repayment loans, StatusCodes (2, 5, 100) | 
 **request_page_size** | **i32**| Max items in result, up to 50000 | 
 **request_page_nr** | **i32**| Result page nr | 

### Return type

[**::models::ApiResultMyInvestments**](ApiResultMyInvestments.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_get_balance**
> ::models::ApiResultMyAccountBalance account_get_balance()
Gets your account balance information

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ApiResultMyAccountBalance**](ApiResultMyAccountBalance.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **account_get_event_log**
> ::models::ApiResultEventLog account_get_event_log(optional)
Gets events that have been made with this application (related to current access token)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request_event_date_from** | **String**| Start datetime | 
 **request_event_date_to** | **String**| end datetime | 
 **request_event_type** | **i32**| Event type | 
 **request_ip_address** | **String**| IP address | 
 **request_page_size** | **i32**| Max items in result, up to 20000 | 
 **request_page_nr** | **i32**| Result page nr | 

### Return type

[**::models::ApiResultEventLog**](ApiResultEventLog.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

