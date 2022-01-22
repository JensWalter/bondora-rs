# \SecondMarketApi

All URIs are relative to *http://api.bondora.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**second_market_buy**](SecondMarketApi.md#second_market_buy) | **Post** /api/v1/secondarymarket/buy | Buy loans from secondary market.
[**second_market_cancel**](SecondMarketApi.md#second_market_cancel) | **Post** /api/v1/secondarymarket/{id}/cancel | Remove your loans from secondary market.
[**second_market_cancel_multiple**](SecondMarketApi.md#second_market_cancel_multiple) | **Post** /api/v1/secondarymarket/cancel | Remove your loans from secondary market.
[**second_market_get**](SecondMarketApi.md#second_market_get) | **Get** /api/v1/loanpart/{id} | Gets LoanPartDetails info by identifier
[**second_market_get_active**](SecondMarketApi.md#second_market_get_active) | **Get** /api/v1/secondarymarket | Gets list of active secondary market items
[**second_market_get_item**](SecondMarketApi.md#second_market_get_item) | **Get** /api/v1/secondarymarket/{id} | Get the secondary market item summary
[**second_market_get_item_list**](SecondMarketApi.md#second_market_get_item_list) | **Get** /api/v1/secondarymarket/list | Get the secondary market item summaries in a list
[**second_market_get_item_list2**](SecondMarketApi.md#second_market_get_item_list2) | **Post** /api/v1/secondarymarket/list | Get the secondary market item summaries in a list
[**second_market_get_list**](SecondMarketApi.md#second_market_get_list) | **Get** /api/v1/loanpart/list | Gets LoanPartDetails info by identifiers in a list (up to 1000 items).
[**second_market_get_list2**](SecondMarketApi.md#second_market_get_list2) | **Post** /api/v1/loanpart/list | Gets LoanPartDetails info by identifiers in a list (up to 1000 items).
[**second_market_sell**](SecondMarketApi.md#second_market_sell) | **Post** /api/v1/secondarymarket/sell | Sell your loans to secondary market.


# **second_market_buy**
> ::models::ApiResult second_market_buy(ctx, buy_request)
Buy loans from secondary market.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **buy_request** | [**SecondMarketBuyRequest**](SecondMarketBuyRequest.md)|  | 

### Return type

[**::models::ApiResult**](ApiResult.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_cancel**
> ::models::ApiResult second_market_cancel(ctx, id)
Remove your loans from secondary market.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | [**String**](.md)|  | 

### Return type

[**::models::ApiResult**](ApiResult.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_cancel_multiple**
> ::models::ApiResult second_market_cancel_multiple(ctx, cancel_request)
Remove your loans from secondary market.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cancel_request** | [**SecondMarketCancelRequest**](SecondMarketCancelRequest.md)|  | 

### Return type

[**::models::ApiResult**](ApiResult.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_get**
> ::models::ApiResultLoanPartDetails second_market_get(id)
Gets LoanPartDetails info by identifier

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | [**String**](.md)| LoanPartDetails&#39;s identifier | 

### Return type

[**::models::ApiResultLoanPartDetails**](ApiResultLoanPartDetails.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_get_active**
> ::models::ApiResultSecondMarket second_market_get_active(optional)
Gets list of active secondary market items

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
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
 **request_has_debt** | **bool**| Is overdue | 
 **request_loan_status_code** | [**Vec&lt;i32&gt;**](i32.md)| Loan status code  &lt;para&gt;2 Current&lt;/para&gt;&lt;para&gt;100 Overdue&lt;/para&gt;&lt;para&gt;5 60+ days overdue&lt;/para&gt; | 
 **request_loan_debt_management_stage_type** | **i32**| Latest debt management stage type | 
 **request_loan_debt_management_date_active_from** | **String**| Latest debt management date active from | 
 **request_loan_debt_management_date_active_to** | **String**| Latest debt management date active to | 
 **request_late_principal_amount_min** | **f64**| Principal debt amount min | 
 **request_late_principal_amount_max** | **f64**| Principal debt amount max | 
 **request_price_min** | **f64**| Price amount min | 
 **request_price_max** | **f64**| Price amount max | 
 **request_use_of_loan** | **i32**| Use of loan | 
 **request_has_new_schedule** | **bool**| Has been rescheduled | 
 **request_countries** | [**Vec&lt;String&gt;**](String.md)| Two letter iso code for country of origin: EE, ES, FI | 
 **request_ratings** | [**Vec&lt;String&gt;**](String.md)| Bondora&#39;s rating: AA, A, B, C, D, E, F, HR | 
 **request_credit_score_min** | **i32**| Minimum credit score | 
 **request_credit_score_max** | **i32**| Maximum credit score | 
 **request_user_name** | **String**| Borrower&#39;s username | 
 **request_gender** | **i32**| Borrower&#39;s gender: Male 0, Female 1, Unknown 2 | 
 **request_age_min** | **i32**| Minimal age | 
 **request_age_max** | **i32**| Maximum age | 
 **request_income_verification_status** | **i32**| Income verification type | 
 **request_show_my_items** | **bool**| Can find your own items from market: Value Null &#x3D; ALL, True &#x3D; only your items, False &#x3D; other user items | 
 **request_auction_id** | [**String**](.md)| Can find specific auction from market | 
 **request_listed_on_date_from** | **String**| Date when item was published from | 
 **request_listed_on_date_to** | **String**| Date when item was published to | 
 **request_debt_occured_on_from** | **String**| Principal debt started date from | 
 **request_debt_occured_on_to** | **String**| Principal debt started date to | 
 **request_debt_occured_on_for_secondary_from** | **String**| Interest debt started date from | 
 **request_debt_occured_on_for_secondary_to** | **String**| Interest debt started date to | 
 **request_defaulted_date_from** | **String**| Defaulted date from | 
 **request_defaulted_date_to** | **String**| Defaulted date to | 
 **request_rescheduled_from** | **String**| Rescheduled date from | 
 **request_rescheduled_to** | **String**| Rescheduled date to | 
 **request_last_payment_date_from** | **String**| Last payment date from | 
 **request_last_payment_date_to** | **String**| Last payment date to | 
 **request_next_payment_date_from** | **String**| Next payment date from | 
 **request_next_payment_date_to** | **String**| Next payment date to | 
 **request_desired_discount_rate_min** | **f64**| Minimal DesiredDiscountRate | 
 **request_desired_discount_rate_max** | **f64**| Maximal DesiredDiscountRate | 
 **request_xirr_min** | **f64**| Minimal Xirr | 
 **request_xirr_max** | **f64**| Maximal Xirr | 
 **request_page_size** | **i32**| Max items in result, up to 100000 | 
 **request_page_nr** | **i32**| Result page nr | 

### Return type

[**::models::ApiResultSecondMarket**](ApiResultSecondMarket.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_get_item**
> ::models::ApiResultSecondMarketItemSummary second_market_get_item(id)
Get the secondary market item summary

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | [**String**](.md)| SecondaryMarket item identificator | 

### Return type

[**::models::ApiResultSecondMarketItemSummary**](ApiResultSecondMarketItemSummary.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_get_item_list**
> ::models::ApiResultSecondMarketItemSummaryList second_market_get_item_list(optional)
Get the secondary market item summaries in a list

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request_item_ids** | [**Vec&lt;String&gt;**](String.md)| Secondary market item ID&#39;s to list. Limited to 1000 items. | 

### Return type

[**::models::ApiResultSecondMarketItemSummaryList**](ApiResultSecondMarketItemSummaryList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_get_item_list2**
> ::models::ApiResultSecondMarketItemSummaryList second_market_get_item_list2(request)
Get the secondary market item summaries in a list

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **request** | [**SecondMarketListingRequest**](SecondMarketListingRequest.md)| SecondaryMarket item identificators.  This endpoint supports both GET and POST methods.  If using this endpoint with the GET method the request data must be sent with the request body, even though it is a GET request.  The \&quot;Content-Type\&quot; header must be set so the server knows how to decode the data. | 

### Return type

[**::models::ApiResultSecondMarketItemSummaryList**](ApiResultSecondMarketItemSummaryList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_get_list**
> ::models::ApiResultLoanPartDetailsList second_market_get_list(optional)
Gets LoanPartDetails info by identifiers in a list (up to 1000 items).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request_item_ids** | [**Vec&lt;String&gt;**](String.md)| LoanPart ID&#39;s to list. Limited to 1000 items. | 

### Return type

[**::models::ApiResultLoanPartDetailsList**](ApiResultLoanPartDetailsList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_get_list2**
> ::models::ApiResultLoanPartDetailsList second_market_get_list2(request)
Gets LoanPartDetails info by identifiers in a list (up to 1000 items).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **request** | [**LoanPartDetailsRequest**](LoanPartDetailsRequest.md)| LoanPartDetails identifiers list.  This endpoint supports both GET and POST methods.  If using this endpoint with the GET method the request data must be sent with the request body, even though it is a GET request.  The \&quot;Content-Type\&quot; header must be set so the server knows how to decode the data. | 

### Return type

[**::models::ApiResultLoanPartDetailsList**](ApiResultLoanPartDetailsList.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **second_market_sell**
> ::models::ApiResultSecondMarketSale second_market_sell(ctx, sale_request)
Sell your loans to secondary market.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **sale_request** | [**SecondMarketSaleRequest**](SecondMarketSaleRequest.md)|  | 

### Return type

[**::models::ApiResultSecondMarketSale**](ApiResultSecondMarketSale.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

