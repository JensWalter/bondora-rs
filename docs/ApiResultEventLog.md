# ApiResultEventLog

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page_size** | **i32** | Requested Max items in result | [optional] [default to null]
**page_nr** | **i32** | Requested page nr | [optional] [default to null]
**total_count** | **i32** | Total number of items found | [default to null]
**count** | **i32** | Number of items returned | [default to null]
**payload** | [**Vec<crate::models::EventLogItem>**](EventLogItem.md) | The payload of the response. Type depends on the API request. | [optional] [default to null]
**success** | **bool** | Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise. | [default to null]
**errors** | [**Vec<crate::models::ApiError>**](ApiError.md) | Error(s) accociated with the API request. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


