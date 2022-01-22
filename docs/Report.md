# Report

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**result** | [**Vec<Value>**](Value.md) | &lt;para&gt;List of Report items. Item type depends on {Bondora.Core.Enums.ReportType} value:&lt;/para&gt;  &lt;list type&#x3D;\&quot;bullet\&quot;&gt;    &lt;item&gt;      &lt;term&gt;SecondMarketArchive&lt;/term&gt;      &lt;description&gt;the type is {Sobralaen.Api.Models.SecondMarketArchiveReportLine}&lt;/description&gt;    &lt;/item&gt;    &lt;item&gt;      &lt;term&gt;AccountStatement&lt;/term&gt;      &lt;description&gt;the type is {Sobralaen.Api.Models.AccountStatementReportLine}&lt;/description&gt;    &lt;/item&gt;    &lt;item&gt;      &lt;term&gt;Repayments&lt;/term&gt;      &lt;description&gt;the type is {Sobralaen.Api.Models.RepaymentsReportLine}&lt;/description&gt;    &lt;/item&gt;    &lt;item&gt;      &lt;term&gt;Investments&lt;/term&gt;      &lt;description&gt;the type is {Sobralaen.Api.Models.InvestmentsListReportLine}&lt;/description&gt;    &lt;/item&gt;    &lt;item&gt;      &lt;term&gt;PlannedFutureCashflows&lt;/term&gt;      &lt;description&gt;the type is {Sobralaen.Api.Models.FutureCashflowsReportLine}&lt;/description&gt;    &lt;/item&gt;    &lt;item&gt;      &lt;term&gt;InvestmentsV2&lt;/term&gt;      &lt;description&gt;the type is {Sobralaen.Api.Models.InvestmentsListReportLineV2}&lt;/description&gt;    &lt;/item&gt;  &lt;/list&gt; | [optional] [default to null]
**report_id** | **String** | Reports unique identifier | [optional] [default to null]
**created_on** | **String** | Report created date | [optional] [default to null]
**generated_on** | **String** | Report generated date | [optional] [default to null]
**period_start** | **String** | Report period end date | [optional] [default to null]
**period_end** | **String** | Report period start date | [optional] [default to null]
**report_type** | **i32** | Report&#39;s type | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


