# \AuctionApi

All URIs are relative to *http://api.bondora.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auction_get**](AuctionApi.md#auction_get) | **Get** /api/v1/auction/{id} | Gets Auction info by auction identifier
[**auction_get_active**](AuctionApi.md#auction_get_active) | **Get** /api/v1/auctions | Gets list of active Auctions


# **auction_get**
> ::models::ApiResultExtendedAuction auction_get(id)
Gets Auction info by auction identifier

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | [**String**](.md)| Auction&#39;s identifier | 

### Return type

[**::models::ApiResultExtendedAuction**](ApiResultExtendedAuction.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **auction_get_active**
> ::models::ApiResultAuctions auction_get_active(optional)
Gets list of active Auctions

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request_countries** | [**Vec&lt;String&gt;**](String.md)| Two letter iso code for country of origin: EE, ES, FI | 
 **request_ratings** | [**Vec&lt;String&gt;**](String.md)| Bondora&#39;s rating: AA, A, B, C, D, E, F, HR | 
 **request_gender** | **i32**| Borrower&#39;s gender: Male 0, Female 1, Unknown 2 | 
 **request_sum_min** | **i32**| Minimal loan amount | 
 **request_sum_max** | **i32**| Maximum loan amount | 
 **request_terms** | [**Vec&lt;i32&gt;**](i32.md)| Loan length: 3, 9, 12, 18, 24, 36, 48, 60 months | 
 **request_age_min** | **i32**| Minimal age | 
 **request_age_max** | **i32**| Maximum age | 
 **request_loan_number** | **i32**| Loan number | 
 **request_user_name** | **String**| Username | 
 **request_application_date_from** | **String**| Loan application started date from | 
 **request_application_date_to** | **String**| Loan application started date to | 
 **request_credit_score_min** | **i32**| Minimum credit score | 
 **request_credit_score_max** | **i32**| Maximum credit score | 
 **request_credit_scores_ee_mini** | [**Vec&lt;String&gt;**](String.md)| Credit score for EE loans | 
 **request_interest_min** | **f64**| Minimum interest | 
 **request_interest_max** | **f64**| Maximum interest | 
 **request_income_total_min** | **f64**| Minimal total income | 
 **request_income_total_max** | **f64**| Maximum total income | 
 **request_model_version** | **i32**| Model version | 
 **request_expected_loss_min** | **f64**| Minimal expected loss | 
 **request_expected_loss_max** | **f64**| Maximum expected loss | 
 **request_listed_on_utc_from** | **String**| Date when auction was published from | 
 **request_listed_on_utc_to** | **String**| Date when auction was published to | 
 **request_page_size** | **i32**| Max items in result, up to 20000 | 
 **request_page_nr** | **i32**| Result page nr | 

### Return type

[**::models::ApiResultAuctions**](ApiResultAuctions.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

