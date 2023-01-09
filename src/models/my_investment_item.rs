/// MyInvestmentItem : MyInvestments list item's information
#[derive(Debug, Serialize, Deserialize)]
pub struct MyInvestmentItem {
    /// Auction bid type
    #[serde(rename = "AuctionBidType")]
    pub auction_bid_type: Option<i32>,
    /// Loan status active from
    #[serde(rename = "LoanStatusActiveFrom")]
    pub loan_status_active_from: Option<String>,
    /// Date when investment was made or purchased from second market
    #[serde(rename = "PurchaseDate")]
    pub purchase_date: Option<String>,
    /// Investment selling date
    #[serde(rename = "SoldDate")]
    pub sold_date: Option<String>,
    /// Investment amount or secondary market purchase price
    #[serde(rename = "PurchasePrice")]
    pub purchase_price: Option<f64>,
    /// SecondMarket sale price
    #[serde(rename = "SalePrice")]
    pub sale_price: Option<f64>,
    /// Date when item was listed on secondary market
    #[serde(rename = "ListedInSecondMarketOn")]
    pub listed_in_second_market_on: Option<String>,
    /// Latest debt management stage
    #[serde(rename = "LatestDebtManagementStage")]
    pub latest_debt_management_stage: Option<i32>,
    /// Latest stage of debt management
    #[serde(rename = "LatestDebtManagementSubStage")]
    pub latest_debt_management_sub_stage: Option<i32>,
    /// Latest debt management stage type
    #[serde(rename = "LatestDebtManagementStageType")]
    pub latest_debt_management_stage_type: Option<i32>,
    /// Latest debt management date
    #[serde(rename = "LatestDebtManagementDate")]
    pub latest_debt_management_date: Option<String>,
    /// Note owner received loan transfers principal amount
    #[serde(rename = "NoteLoanTransfersMainAmount")]
    pub note_loan_transfers_main_amount: Option<f64>,
    /// Note owner received loan transfers interest amount
    #[serde(rename = "NoteLoanTransfersInterestAmount")]
    pub note_loan_transfers_interest_amount: Option<f64>,
    /// Note owner received loan transfers penalties amount
    #[serde(rename = "NoteLoanLateChargesPaid")]
    pub note_loan_late_charges_paid: Option<f64>,
    /// Note owner received loan transfers earned interest, penalties total amount
    #[serde(rename = "NoteLoanTransfersEarningsAmount")]
    pub note_loan_transfers_earnings_amount: Option<f64>,
    /// Note owner received loan transfers total amount
    #[serde(rename = "NoteLoanTransfersTotalRepaimentsAmount")]
    pub note_loan_transfers_total_repaiments_amount: Option<f64>,
    /// From who the note was bought from
    #[serde(rename = "BoughtFromId")]
    pub bought_from_id: Option<String>,
    /// Loan unique identifier
    #[serde(rename = "LoanId")]
    pub loan_id: Option<String>,
    /// Loan original lenght
    #[serde(rename = "LoanDuration")]
    pub loan_duration: Option<i32>,
    /// A score that is specifically designed for risk classifying subprime borrowers (defined by Equifax as borrowers that do not have access to bank loans).  A measure of the probability of default one month ahead.  <para>The score is given on a 10-grade scale, from the best score to the worst:</para><para>M1, M2, M3, M4, M5, M6, M7, M8, M9, M10</para>
    #[serde(rename = "CreditScoreEsMicroL")]
    pub credit_score_es_micro_l: Option<String>,
    /// Generic score for the loan applicants that do not have active past due operations in ASNEF.  A measure of the probability of default one year ahead.  The score is given on a 6-grade scale.  <para>AAA Very low</para><para>AA Low</para><para>A Average</para><para>B Average High</para><para>C High</para><para>D Very High</para>
    #[serde(rename = "CreditScoreEsEquifaxRisk")]
    pub credit_score_es_equifax_risk: Option<String>,
    /// Credit Scoring model for Finnish Asiakastieto  <para>RL1 Very low risk 01-20</para><para>RL2 Low risk 21-40</para><para>RL3 Average risk 41-60</para><para>RL4 Big risk 61-80</para><para>RL5 Huge risk 81-100</para>
    #[serde(rename = "CreditScoreFiAsiakasTietoRiskGrade")]
    pub credit_score_fi_asiakas_tieto_risk_grade: Option<String>,
    /// Credit scoring for Estonian loans  <para>1000 No previous payments problems</para><para>900 Payments problems finished 24-36 months ago</para><para>800 Payments problems finished 12-24 months ago</para><para>700 Payments problems finished 6-12 months ago</para><para>600 Payment problems finished &lt;6 months ago</para><para>500 Active payment problems</para>
    #[serde(rename = "CreditScoreEeMini")]
    pub credit_score_ee_mini: Option<String>,
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
    pub debt_servicing_cost_penalty_amount: Option<f64>,
}
