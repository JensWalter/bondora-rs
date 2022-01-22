# AuctionRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**countries** | **Vec<String>** | Two letter iso code for country of origin: EE, ES, FI | [optional] [default to null]
**ratings** | **Vec<String>** | Bondora&#39;s rating: AA, A, B, C, D, E, F, HR | [optional] [default to null]
**gender** | **i32** | Borrower&#39;s gender: Male 0, Female 1, Unknown 2 | [optional] [default to null]
**sum_min** | **i32** | Minimal loan amount | [optional] [default to null]
**sum_max** | **i32** | Maximum loan amount | [optional] [default to null]
**terms** | **Vec<i32>** | Loan length: 3, 9, 12, 18, 24, 36, 48, 60 months | [optional] [default to null]
**age_min** | **i32** | Minimal age | [optional] [default to null]
**age_max** | **i32** | Maximum age | [optional] [default to null]
**loan_number** | **i32** | Loan number | [optional] [default to null]
**user_name** | **String** | Username | [optional] [default to null]
**application_date_from** | **String** | Loan application started date from | [optional] [default to null]
**application_date_to** | **String** | Loan application started date to | [optional] [default to null]
**credit_score_min** | **i32** | Minimum credit score | [optional] [default to null]
**credit_score_max** | **i32** | Maximum credit score | [optional] [default to null]
**credit_scores_ee_mini** | **Vec<String>** | Credit score for EE loans | [optional] [default to null]
**interest_min** | **f64** | Minimum interest | [optional] [default to null]
**interest_max** | **f64** | Maximum interest | [optional] [default to null]
**income_total_min** | **f64** | Minimal total income | [optional] [default to null]
**income_total_max** | **f64** | Maximum total income | [optional] [default to null]
**model_version** | **i32** | Model version | [optional] [default to null]
**expected_loss_min** | **f64** | Minimal expected loss | [optional] [default to null]
**expected_loss_max** | **f64** | Maximum expected loss | [optional] [default to null]
**listed_on_utc_from** | **String** | Date when auction was published from | [optional] [default to null]
**listed_on_utc_to** | **String** | Date when auction was published to | [optional] [default to null]
**page_size** | **i32** | Max items in result, up to 20000 | [optional] [default to null]
**page_nr** | **i32** | Result page nr | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


