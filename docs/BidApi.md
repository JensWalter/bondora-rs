# \BidApi

All URIs are relative to *http://api.bondora.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bid_cancel_bid**](BidApi.md#bid_cancel_bid) | **Post** /api/v1/bid/{id}/cancel | Cancel the Bid
[**bid_get_bid**](BidApi.md#bid_get_bid) | **Get** /api/v1/bid/{id} | Get the Bid
[**bid_get_bid_summaries**](BidApi.md#bid_get_bid_summaries) | **Get** /api/v1/bids | Gets list of bids the investor has made.
[**bid_make_bids**](BidApi.md#bid_make_bids) | **Post** /api/v1/bid | Makes bid(s) into specified auction(s).


# **bid_cancel_bid**
> ::models::ApiResult bid_cancel_bid(ctx, id)
Cancel the Bid

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | [**String**](.md)| Bid identificator | 

### Return type

[**::models::ApiResult**](ApiResult.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **bid_get_bid**
> ::models::ApiResultBid bid_get_bid(ctx, id)
Get the Bid

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | [**String**](.md)| Bid identificator | 

### Return type

[**::models::ApiResultBid**](ApiResultBid.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **bid_get_bid_summaries**
> ::models::ApiResultBids bid_get_bid_summaries(ctx, optional)
Gets list of bids the investor has made.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **request_bid_status_code** | **i32**| Bid status code | 
 **request_start_date** | **String**| Start date | 
 **request_end_date** | **String**| End date | 
 **request_page_size** | **i32**| Max items in result, up to 20000 | 
 **request_page_nr** | **i32**| Result page nr | 

### Return type

[**::models::ApiResultBids**](ApiResultBids.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **bid_make_bids**
> ::models::ApiResultMakeBids bid_make_bids(ctx, bid_request)
Makes bid(s) into specified auction(s).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **bid_request** | [**BidRequest**](BidRequest.md)|  | 

### Return type

[**::models::ApiResultMakeBids**](ApiResultMakeBids.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json, text/json, application/xml, text/xml, application/x-www-form-urlencoded
 - **Accept**: application/json, text/json, application/xml, text/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

