# BidSummary

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Bid unique identifier | [optional] [default to null]
**auction_id** | **String** | Id of auction to bid into | [optional] [default to null]
**requested_bid_amount** | **f64** | Amount that was requested to bid | [optional] [default to null]
**actual_bid_amount** | **f64** | Amount that is bidded | [optional] [default to null]
**requested_bid_minimum_limit** | **f64** | Minimum amount that was specified for auction | [optional] [default to null]
**bid_requested_date** | **String** | When bid was requested | [optional] [default to null]
**bid_processed_date** | **String** | When bid was processed | [optional] [default to null]
**is_request_being_processed** | **bool** | Is request currently processed | [optional] [default to null]
**status_code** | **i32** | Status of bid  &lt;para&gt;0 Pending&lt;/para&gt;&lt;para&gt;1 Open&lt;/para&gt;&lt;para&gt;2 Successful&lt;/para&gt;&lt;para&gt;3 Failed&lt;/para&gt;&lt;para&gt;4 Cancelled&lt;/para&gt;&lt;para&gt;5 Accepted&lt;/para&gt; | [optional] [default to null]
**failure_reason** | **i32** | Why bid failed | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


