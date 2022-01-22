# MyInvestmentItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auction_bid_type** | **i32** | Auction bid type | [optional] [default to null]
**loan_status_active_from** | **String** | Loan status active from | [optional] [default to null]
**purchase_date** | **String** | Date when investment was made or purchased from second market | [optional] [default to null]
**sold_date** | **String** | Investment selling date | [optional] [default to null]
**purchase_price** | **f64** | Investment amount or secondary market purchase price | [optional] [default to null]
**sale_price** | **f64** | SecondMarket sale price | [optional] [default to null]
**listed_in_second_market_on** | **String** | Date when item was listed on secondary market | [optional] [default to null]
**latest_debt_management_stage** | **i32** | Latest debt management stage | [optional] [default to null]
**latest_debt_management_sub_stage** | **i32** | Latest stage of debt management | [optional] [default to null]
**latest_debt_management_stage_type** | **i32** | Latest debt management stage type | [optional] [default to null]
**latest_debt_management_date** | **String** | Latest debt management date | [optional] [default to null]
**note_loan_transfers_main_amount** | **f64** | Note owner received loan transfers principal amount | [optional] [default to null]
**note_loan_transfers_interest_amount** | **f64** | Note owner received loan transfers interest amount | [optional] [default to null]
**note_loan_late_charges_paid** | **f64** | Note owner received loan transfers penalties amount | [optional] [default to null]
**note_loan_transfers_earnings_amount** | **f64** | Note owner received loan transfers earned interest, penalties total amount | [optional] [default to null]
**note_loan_transfers_total_repaiments_amount** | **f64** | Note owner received loan transfers total amount | [optional] [default to null]
**bought_from_id** | **String** | From who the note was bought from | [optional] [default to null]
**loan_id** | **String** | Loan unique identifier | [optional] [default to null]
**loan_duration** | **i32** | Loan original lenght | [optional] [default to null]
**credit_score_es_micro_l** | **String** | A score that is specifically designed for risk classifying subprime borrowers (defined by Equifax as borrowers that do not have access to bank loans).  A measure of the probability of default one month ahead.  &lt;para&gt;The score is given on a 10-grade scale, from the best score to the worst:&lt;/para&gt;&lt;para&gt;M1, M2, M3, M4, M5, M6, M7, M8, M9, M10&lt;/para&gt; | [optional] [default to null]
**credit_score_es_equifax_risk** | **String** | Generic score for the loan applicants that do not have active past due operations in ASNEF.  A measure of the probability of default one year ahead.  The score is given on a 6-grade scale.  &lt;para&gt;AAA Very low&lt;/para&gt;&lt;para&gt;AA Low&lt;/para&gt;&lt;para&gt;A Average&lt;/para&gt;&lt;para&gt;B Average High&lt;/para&gt;&lt;para&gt;C High&lt;/para&gt;&lt;para&gt;D Very High&lt;/para&gt; | [optional] [default to null]
**credit_score_fi_asiakas_tieto_risk_grade** | **String** | Credit Scoring model for Finnish Asiakastieto  &lt;para&gt;RL1 Very low risk 01-20&lt;/para&gt;&lt;para&gt;RL2 Low risk 21-40&lt;/para&gt;&lt;para&gt;RL3 Average risk 41-60&lt;/para&gt;&lt;para&gt;RL4 Big risk 61-80&lt;/para&gt;&lt;para&gt;RL5 Huge risk 81-100&lt;/para&gt; | [optional] [default to null]
**credit_score_ee_mini** | **String** | Credit scoring for Estonian loans  &lt;para&gt;1000 No previous payments problems&lt;/para&gt;&lt;para&gt;900 Payments problems finished 24-36 months ago&lt;/para&gt;&lt;para&gt;800 Payments problems finished 12-24 months ago&lt;/para&gt;&lt;para&gt;700 Payments problems finished 6-12 months ago&lt;/para&gt;&lt;para&gt;600 Payment problems finished &amp;lt;6 months ago&lt;/para&gt;&lt;para&gt;500 Active payment problems&lt;/para&gt; | [optional] [default to null]
**loan_part_id** | **String** | LoanPart unique identifier | [optional] [default to null]
**amount** | **f64** | Investment amount | [optional] [default to null]
**auction_id** | **String** | Auction unique identifier | [optional] [default to null]
**auction_name** | **String** | Auction name | [optional] [default to null]
**auction_number** | **i32** | Auction number | [optional] [default to null]
**auction_bid_number** | **i32** | Auction bid number | [optional] [default to null]
**investment_number** | **String** | Auction number + Auction bid number combined | [optional] [default to null]
**country** | **String** | Residency of the borrower | [optional] [default to null]
**credit_score** | **f64** | &lt;para&gt;    1000 No previous payments problems&lt;/para&gt;  &lt;para&gt;    900 Payments problems finished 24-36 months ago&lt;/para&gt;  &lt;para&gt;    800 Payments problems finished 12-24 months ago&lt;/para&gt;  &lt;para&gt;    700 Payments problems finished 6-12 months ago&lt;/para&gt;  &lt;para&gt;    600 Payment problems finished &amp;lt;6 months ago&lt;/para&gt;  &lt;para&gt;    500 Active payment problems&lt;/para&gt; | [optional] [default to null]
**rating** | **String** | Bondora Rating issued by the Rating model | [optional] [default to null]
**interest** | **f64** | Current interest rate | [optional] [default to null]
**use_of_loan** | **i32** | Use of loan | [optional] [default to null]
**income_verification_status** | **i32** | Income verification type | [optional] [default to null]
**loan_status_code** | **i32** | Loan status code  &lt;para&gt;0 Reserved&lt;/para&gt;&lt;para&gt;2 Current&lt;/para&gt;&lt;para&gt;3 Cancelled&lt;/para&gt;&lt;para&gt;100 Overdue&lt;/para&gt;&lt;para&gt;5 60+ days overdue&lt;/para&gt;&lt;para&gt;4 Repaid&lt;/para&gt;&lt;para&gt;8 Released&lt;/para&gt; | [optional] [default to null]
**user_name** | **String** | Borrower&#39;s username | [optional] [default to null]
**gender** | **i32** | Borrower&#39;s Gender | [optional] [default to null]
**date_of_birth** | **String** | Borrower&#39;s date of birth | [optional] [default to null]
**signed_date** | **String** | Loan issued date | [optional] [default to null]
**re_scheduled_on** | **String** | Last rescheduling date | [optional] [default to null]
**debt_occured_on** | **String** | Date and time when the principal part of the payment is overdue (PrincipalLateAmount is greater than zero). | [optional] [default to null]
**debt_occured_on_for_secondary** | **String** | Date and time when loan part payment is overdue (principal, interest or penalty) aka when the dept occured for the loan part (LateAmountTotal is greater than zero). | [optional] [default to null]
**next_payment_nr** | **i32** | Next scheduled payment number | [optional] [default to null]
**next_payment_date** | **String** | Next scheduled payment date | [optional] [default to null]
**next_payment_sum** | **f64** | Next scheduled payment amount | [optional] [default to null]
**nr_of_scheduled_payments** | **i32** | Total number of scheduled payments | [optional] [default to null]
**last_payment_date** | **String** | Last payment date | [optional] [default to null]
**principal_repaid** | **f64** | Total principal repaid amount | [optional] [default to null]
**interest_repaid** | **f64** | Total interest repaid amount | [optional] [default to null]
**late_amount_paid** | **f64** | Total late charges paid amount | [optional] [default to null]
**principal_remaining** | **f64** | Remaining principal amount | [optional] [default to null]
**principal_late_amount** | **f64** | Principal debt amount | [optional] [default to null]
**interest_late_amount** | **f64** | Interest debt amount | [optional] [default to null]
**penalty_late_amount** | **f64** | Late charges debt amount | [optional] [default to null]
**late_amount_total** | **f64** | Late amount total | [optional] [default to null]
**principal_write_off_amount** | **f64** | Total amount of principal written off | [optional] [default to null]
**interest_write_off_amount** | **f64** | Total amount of interest written off | [optional] [default to null]
**penalty_write_off_amount** | **f64** | Total amount of penalty written off | [optional] [default to null]
**debt_servicing_cost_main_amount** | **f64** | Total amount of principal debt servicing cost | [optional] [default to null]
**debt_servicing_cost_interest_amount** | **f64** | Total amount of interest debt servicing cost | [optional] [default to null]
**debt_servicing_cost_penalty_amount** | **f64** | Total amount of penalty debt servicing cost | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

