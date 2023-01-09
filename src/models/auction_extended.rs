/// AuctionExtended : Auction related data with debts and liabilities
#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionExtended {
    /// Borrower's liabilities
    #[serde(rename = "Liabilities")]
    pub liabilities: Option<Vec<crate::models::Liability>>,
    /// Borrower's debts
    #[serde(rename = "Debts")]
    pub debts: Option<Vec<crate::models::Debt>>,
    /// Borrower's history
    #[serde(rename = "BorrowerHistory")]
    pub borrower_history: Option<crate::models::BorrowerHistory>,
    /// Unique loan identificator
    #[serde(rename = "LoanId")]
    pub loan_id: Option<String>,
    /// Unique auction identificator
    #[serde(rename = "AuctionId")]
    pub auction_id: Option<String>,
    /// Number of the loan
    #[serde(rename = "LoanNumber")]
    pub loan_number: Option<i32>,
    /// Customer's Bondora username
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
    /// Did the customer have prior credit history in Bondora  <para>0 Customer had at least 3 months of credit history in Bondora</para><para>1 No prior credit history in Bondora</para>
    #[serde(rename = "NewCreditCustomer")]
    pub new_credit_customer: Option<i32>,
    /// Date when the loan application was started
    #[serde(rename = "LoanApplicationStartedDate")]
    pub loan_application_started_date: Option<String>,
    /// Date and time when the auction is closed, if it's not funded 100% before that.  Auction will be closed before that, if auction is funded 100%.
    #[serde(rename = "PlannedCloseDate")]
    pub planned_close_date: Option<String>,
    /// Hour of signing the loan application
    #[serde(rename = "ApplicationSignedHour")]
    pub application_signed_hour: Option<i32>,
    /// Weekday of signing the loan application
    #[serde(rename = "ApplicationSignedWeekday")]
    pub application_signed_weekday: Option<i32>,
    /// Verification type
    #[serde(rename = "VerificationType")]
    pub verification_type: Option<i32>,
    /// Two letter language code
    #[serde(rename = "LanguageCode")]
    pub language_code: Option<i32>,
    /// Age of the borrower (years)
    #[serde(rename = "Age")]
    pub age: Option<i32>,
    /// Borrower's date of birth
    #[serde(rename = "DateOfBirth")]
    pub date_of_birth: Option<String>,
    /// Gender
    #[serde(rename = "Gender")]
    pub gender: Option<i32>,
    /// Residency of the borrower
    #[serde(rename = "Country")]
    pub country: Option<String>,
    /// A score that is specifically designed for risk classifying subprime borrowers (defined by Equifax as borrowers that do not have access to bank loans).   A measure of the probability of default one month ahead.  <para>The score is given on a 10-grade scale, from the best score to the worst:</para><para>M1, M2, M3, M4, M5, M6, M7, M8, M9, M10</para>
    #[serde(rename = "CreditScoreEsMicroL")]
    pub credit_score_es_micro_l: Option<String>,
    /// Generic score for the loan applicants that do not have active past due operations in ASNEF.  A measure of the probability of default one year ahead.   The score is given on a 6-grade scale.  <para>AAA Very low</para><para>AA Low</para><para>A Average</para><para>B Average High</para><para>C High</para><para>D Very High</para>
    #[serde(rename = "CreditScoreEsEquifaxRisk")]
    pub credit_score_es_equifax_risk: Option<String>,
    /// Credit Scoring model for Finnish Asiakastieto  <para>RL1 Very low risk 01-20</para><para>RL2 Low risk 21-40</para><para>RL3 Average risk 41-60</para><para>RL4 Big risk 61-80</para><para>RL5 Huge risk 81-100</para>
    #[serde(rename = "CreditScoreFiAsiakasTietoRiskGrade")]
    pub credit_score_fi_asiakas_tieto_risk_grade: Option<String>,
    /// Credit scoring for Estonian loans  <para>1000 No previous payments problems</para><para>900 Payments problems finished 24-36 months ago</para><para>800 Payments problems finished 12-24 months ago</para><para>700 Payments problems finished 6-12 months ago</para><para>600 Payment problems finished &lt;6 months ago</para><para>500 Active payment problems</para>
    #[serde(rename = "CreditScoreEeMini")]
    pub credit_score_ee_mini: Option<String>,
    /// The amount borrower applied for originally
    #[serde(rename = "AppliedAmount")]
    pub applied_amount: Option<f64>,
    /// Maximum interest rate accepted in the loan application
    #[serde(rename = "Interest")]
    pub interest: Option<f64>,
    /// The loan term
    #[serde(rename = "LoanDuration")]
    pub loan_duration: Option<i32>,
    /// County of the borrower
    #[serde(rename = "County")]
    pub county: Option<String>,
    /// City of the borrower
    #[serde(rename = "City")]
    pub city: Option<String>,
    /// Education
    #[serde(rename = "Education")]
    pub education: Option<i32>,
    /// Employment time with the current employer
    #[serde(rename = "EmploymentDurationCurrentEmployer")]
    pub employment_duration_current_employer: Option<String>,
    /// Type of home ownership
    #[serde(rename = "HomeOwnershipType")]
    pub home_ownership_type: Option<i32>,
    /// Total income
    #[serde(rename = "IncomeTotal")]
    pub income_total: Option<f64>,
    /// Loan monthly payment amount.
    #[serde(rename = "MonthlyPayment")]
    pub monthly_payment: Option<f64>,
    /// The day of the month the loan payments are scheduled for.  The actual date is adjusted for weekends and bank holidays.  E.g. if 10th is a Sunday then the payment will be made on the 11th in that month.
    #[serde(rename = "MonthlyPaymentDay")]
    pub monthly_payment_day: Option<i32>,
    /// The version of the Rating model used for issuing the Bondora Rating
    #[serde(rename = "ModelVersion")]
    pub model_version: Option<i32>,
    /// Expected Loss calculated by the Rating model
    #[serde(rename = "ExpectedLoss")]
    pub expected_loss: Option<f64>,
    /// Bondora Rating issued by the Rating model
    #[serde(rename = "Rating")]
    pub rating: Option<String>,
    /// Gives the percentage of outstanding exposure at the time of default that an investor is likely to lose if a loan actually defaults.   This means the proportion of funds lost for the investor after all expected recovery and accounting for the time value of the money recovered.   In general, LGD parameter is intended to be estimated based on the historical recoveries. However, in new markets where limited experience does not allow us more precise loss given default estimates, a LGD of 90% is assumed.
    #[serde(rename = "LossGivenDefault")]
    pub loss_given_default: Option<f64>,
    /// Probability of Default, refers to a loan’s probability of default within one year horizon.
    #[serde(rename = "ProbabilityOfDefault")]
    pub probability_of_default: Option<f64>,
    /// Expected return alpha
    #[serde(rename = "ExpectedReturnAlpha")]
    pub expected_return_alpha: Option<f64>,
    /// Total liabilities
    #[serde(rename = "LiabilitiesTotal")]
    pub liabilities_total: Option<f64>,
    /// Date when auction was published
    #[serde(rename = "ListedOnUTC")]
    pub listed_on_utc: Option<String>,
    /// Date and time when the auction was actually closed. Is null, if auction is active.
    #[serde(rename = "ActualCloseDate")]
    pub actual_close_date: Option<String>,
    /// The amount that auction is fulfilled, taken amount only bids where investors have enough funds.  This is preliminary calculated amount and can change when trying to close auction (auction is closed, when auction is funded 100% or PlannedCloseDate is reached) and specific investor(s) do not have enough funds.
    #[serde(rename = "WinningBidsAmount")]
    pub winning_bids_amount: Option<f64>,
    /// The amount that is remaining to be funded (AppliedAmount - WinningBidsAmount).
    #[serde(rename = "RemainingAmount")]
    pub remaining_amount: Option<f64>,
    /// How many bids current user has bidden into the auction
    #[serde(rename = "UserBids")]
    pub user_bids: Option<i32>,
    /// How much current user has bidden into the auction
    #[serde(rename = "UserBidAmount")]
    pub user_bid_amount: Option<f64>,
    /// Precentage, how much the auction is fulfilled. Can be more than 100%, if overfunded.
    #[serde(rename = "Fullfilled")]
    pub fullfilled: Option<f64>,
    /// <para>    1000 No previous payments problems</para>  <para>    900 Payments problems finished 24-36 months ago</para>  <para>    800 Payments problems finished 12-24 months ago</para>  <para>    700 Payments problems finished 6-12 months ago</para>  <para>    600 Payment problems finished &lt;6 months ago</para>  <para>    500 Active payment problems</para>
    #[serde(rename = "CreditScore")]
    pub credit_score: Option<i32>,
    /// Date when the Rating was calculated for this loan
    #[serde(rename = "ScoringDate")]
    pub scoring_date: Option<String>,
    /// Use of loan  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
    #[serde(rename = "UseOfLoan")]
    pub use_of_loan: Option<i32>,
    /// Marital status  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
    #[serde(rename = "MaritalStatus")]
    pub marital_status: Option<i32>,
    /// Number of children or other dependants  <para>Only Value for new Auctions since 1st of june 2017 is NULL</para>
    #[serde(rename = "NrOfDependants")]
    pub nr_of_dependants: Option<String>,
    /// Employment status  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
    #[serde(rename = "EmploymentStatus")]
    pub employment_status: Option<i32>,
    /// Employment position  <para>Only Value for new Auctions since 1st of june 2017 is NULL</para>
    #[serde(rename = "EmploymentPosition")]
    pub employment_position: Option<String>,
    /// Work experience in total  <para>Only Value for new Auctions since 1st of june 2017 is NULL</para>
    #[serde(rename = "WorkExperience")]
    pub work_experience: Option<String>,
    /// Occupation area  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
    #[serde(rename = "OccupationArea")]
    pub occupation_area: Option<i32>,
    /// Salary  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "IncomeFromPrincipalEmployer")]
    pub income_from_principal_employer: Option<f64>,
    /// Pension  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "IncomeFromPension")]
    pub income_from_pension: Option<f64>,
    /// Family allowance  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "IncomeFromFamilyAllowance")]
    pub income_from_family_allowance: Option<f64>,
    /// Social welfare  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "IncomeFromSocialWelfare")]
    pub income_from_social_welfare: Option<f64>,
    /// Leave pay  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "IncomeFromLeavePay")]
    pub income_from_leave_pay: Option<f64>,
    /// Child support  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "IncomeFromChildSupport")]
    pub income_from_child_support: Option<f64>,
    /// Other income  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "IncomeOther")]
    pub income_other: Option<f64>,
    /// Discretionary Income  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "FreeCash")]
    pub free_cash: Option<f64>,
    /// Debt to income ratio  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
    #[serde(rename = "DebtToIncome")]
    pub debt_to_income: Option<f64>,
    /// Exposure at Default (expressed as a percentage of the original loan amount), indicates outstanding investor exposure at the time of default, including outstanding principal amount plus accrued but unpaid interests.
    #[serde(rename = "EADRate")]
    pub ead_rate: Option<f64>,
    /// Maturity Factor M of 1.3 is assumed for loans with duration exceeding one year.
    #[serde(rename = "MaturityFactor")]
    pub maturity_factor: Option<f64>,
    /// Interest rate alpha
    #[serde(rename = "InterestRateAlpha")]
    pub interest_rate_alpha: Option<f64>,
}
