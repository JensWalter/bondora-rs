/// SecondMarketItem : SecondaryMarket list item's information
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketItem {
  /// Item unique identifier
  #[serde(rename = "Id")]
  pub id: Option<String>,
  /// Loan status active from
  #[serde(rename = "LoanStatusActiveFrom")]
  pub loan_status_active_from: Option<String>,
  /// Latest debt management stage type
  #[serde(rename = "LatestDebtManagementStageType")]
  pub latest_debt_management_stage_type: Option<i32>,
  /// Latest debt management date
  #[serde(rename = "LatestDebtManagementDate")]
  pub latest_debt_management_date: Option<String>,
  /// Outstanding principal balance +/- discount or mark-up
  #[serde(rename = "Price")]
  pub price: Option<f64>,
  /// Secondary market purchase fee paid to Bondora
  #[serde(rename = "Fee")]
  pub fee: Option<f64>,
  /// Total amount paid for purchase
  #[serde(rename = "TotalCost")]
  pub total_cost: Option<f64>,
  /// Total amount still to be repaid by the borrower. This includes the principal balance, accrued interest and late charges as well as any future scheduled interest payments
  #[serde(rename = "OutstandingPayments")]
  pub outstanding_payments: Option<f64>,
  /// Discount rate percent
  #[serde(rename = "DesiredDiscountRate")]
  pub desired_discount_rate: Option<f64>,
  /// XIRR (extended internal rate of return) is a methodology to calculate the net return using the loan issued date and amount,   loan repayment dates and amounts and the principal balance according to the original repayment date.   All overdue principal payments are written off immediately. No provisions for future losses are made and only received (not accrued or scheduled)   interest payments are taken into account.
  #[serde(rename = "Xirr")]
  pub xirr: Option<f64>,
  /// Date when item was published
  #[serde(rename = "ListedOnDate")]
  pub listed_on_date: Option<String>,
  /// Webhook only:  Debt managment event collection
  #[serde(rename = "DebtManagmentEvents")]
  pub debt_managment_events: Option<Vec<crate::models::DebtManagementEvent>>,
  /// Webhook only:  Collection of all loan payments
  #[serde(rename = "LoanTransfers")]
  pub loan_transfers: Option<Vec<crate::models::LoanTransfer>>,
  /// Webhook only:  Collection of all loan scheduled payments.   Contains previous period values before rescheduling was made
  #[serde(rename = "ScheduledPayments")]
  pub scheduled_payments: Option<Vec<crate::models::ScheduledPayment>>,
  /// LoanPart unique identifier
  #[serde(rename = "LoanPartId")]
  pub loan_part_id: Option<String>,
  /// Investment amount
  #[serde(rename = "Amount")]
  pub amount: Option<f64>,
  /// Auction unique identifier
  #[serde(rename = "AuctionId")]
  pub auction_id: Option<String>,
  /// Auction name
  #[serde(rename = "AuctionName")]
  pub auction_name: Option<String>,
  /// Auction number
  #[serde(rename = "AuctionNumber")]
  pub auction_number: Option<i32>,
  /// Auction bid number
  #[serde(rename = "AuctionBidNumber")]
  pub auction_bid_number: Option<i32>,
  /// Auction number + Auction bid number combined
  #[serde(rename = "InvestmentNumber")]
  pub investment_number: Option<String>,
  /// Residency of the borrower
  #[serde(rename = "Country")]
  pub country: Option<String>,
  /// <para>    1000 No previous payments problems</para>  <para>    900 Payments problems finished 24-36 months ago</para>  <para>    800 Payments problems finished 12-24 months ago</para>  <para>    700 Payments problems finished 6-12 months ago</para>  <para>    600 Payment problems finished &lt;6 months ago</para>  <para>    500 Active payment problems</para>
  #[serde(rename = "CreditScore")]
  pub credit_score: Option<f64>,
  /// Bondora Rating issued by the Rating model
  #[serde(rename = "Rating")]
  pub rating: Option<String>,
  /// Current interest rate
  #[serde(rename = "Interest")]
  pub interest: Option<f64>,
  /// Use of loan
  #[serde(rename = "UseOfLoan")]
  pub use_of_loan: Option<i32>,
  /// Income verification type
  #[serde(rename = "IncomeVerificationStatus")]
  pub income_verification_status: Option<i32>,
  /// Loan status code  <para>0 Reserved</para><para>2 Current</para><para>3 Cancelled</para><para>100 Overdue</para><para>5 60+ days overdue</para><para>4 Repaid</para><para>8 Released</para>
  #[serde(rename = "LoanStatusCode")]
  pub loan_status_code: Option<i32>,
  /// Borrower's username
  #[serde(rename = "UserName")]
  pub user_name: Option<String>,
  /// Borrower's Gender
  #[serde(rename = "Gender")]
  pub gender: Option<i32>,
  /// Borrower's date of birth
  #[serde(rename = "DateOfBirth")]
  pub date_of_birth: Option<String>,
  /// Loan issued date
  #[serde(rename = "SignedDate")]
  pub signed_date: Option<String>,
  /// Last rescheduling date
  #[serde(rename = "ReScheduledOn")]
  pub re_scheduled_on: Option<String>,
  /// Date and time when the principal part of the payment is overdue (PrincipalLateAmount is greater than zero).
  #[serde(rename = "DebtOccuredOn")]
  pub debt_occured_on: Option<String>,
  /// Date and time when loan part payment is overdue (principal, interest or penalty) aka when the dept occured for the loan part (LateAmountTotal is greater than zero).
  #[serde(rename = "DebtOccuredOnForSecondary")]
  pub debt_occured_on_for_secondary: Option<String>,
  /// Next scheduled payment number
  #[serde(rename = "NextPaymentNr")]
  pub next_payment_nr: Option<i32>,
  /// Next scheduled payment date
  #[serde(rename = "NextPaymentDate")]
  pub next_payment_date: Option<String>,
  /// Next scheduled payment amount
  #[serde(rename = "NextPaymentSum")]
  pub next_payment_sum: Option<f64>,
  /// Total number of scheduled payments
  #[serde(rename = "NrOfScheduledPayments")]
  pub nr_of_scheduled_payments: Option<i32>,
  /// Last payment date
  #[serde(rename = "LastPaymentDate")]
  pub last_payment_date: Option<String>,
  /// Total principal repaid amount
  #[serde(rename = "PrincipalRepaid")]
  pub principal_repaid: Option<f64>,
  /// Total interest repaid amount
  #[serde(rename = "InterestRepaid")]
  pub interest_repaid: Option<f64>,
  /// Total late charges paid amount
  #[serde(rename = "LateAmountPaid")]
  pub late_amount_paid: Option<f64>,
  /// Remaining principal amount
  #[serde(rename = "PrincipalRemaining")]
  pub principal_remaining: Option<f64>,
  /// Principal debt amount
  #[serde(rename = "PrincipalLateAmount")]
  pub principal_late_amount: Option<f64>,
  /// Interest debt amount
  #[serde(rename = "InterestLateAmount")]
  pub interest_late_amount: Option<f64>,
  /// Late charges debt amount
  #[serde(rename = "PenaltyLateAmount")]
  pub penalty_late_amount: Option<f64>,
  /// Late amount total
  #[serde(rename = "LateAmountTotal")]
  pub late_amount_total: Option<f64>,
  /// Total amount of principal written off
  #[serde(rename = "PrincipalWriteOffAmount")]
  pub principal_write_off_amount: Option<f64>,
  /// Total amount of interest written off
  #[serde(rename = "InterestWriteOffAmount")]
  pub interest_write_off_amount: Option<f64>,
  /// Total amount of penalty written off
  #[serde(rename = "PenaltyWriteOffAmount")]
  pub penalty_write_off_amount: Option<f64>,
  /// Total amount of principal debt servicing cost
  #[serde(rename = "DebtServicingCostMainAmount")]
  pub debt_servicing_cost_main_amount: Option<f64>,
  /// Total amount of interest debt servicing cost
  #[serde(rename = "DebtServicingCostInterestAmount")]
  pub debt_servicing_cost_interest_amount: Option<f64>,
  /// Total amount of penalty debt servicing cost
  #[serde(rename = "DebtServicingCostPenaltyAmount")]
  pub debt_servicing_cost_penalty_amount: Option<f64>
}
