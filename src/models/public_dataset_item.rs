/// PublicDatasetItem : Represents Public dataset fields
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicDatasetItem {
    /// A unique ID given to a loan
    #[serde(rename = "LoanId")]
    pub loan_id: Option<String>,
    /// A unique loan number displayed in Bondora's system
    #[serde(rename = "LoanNumber")]
    pub loan_number: Option<i32>,
    /// Date when the loan application appeared on Primary Market
    #[serde(rename = "ListedOnUTC")]
    pub listed_on_utc: Option<String>,
    /// Date when the auction bidding started on
    #[serde(rename = "BiddingStartedOn")]
    pub bidding_started_on: Option<String>,
    /// The amount of investment offers made by Portfolio Managers
    #[serde(rename = "BidsPortfolioManager")]
    pub bids_portfolio_manager: Option<f64>,
    /// The amount of investment offers made via Api
    #[serde(rename = "BidsApi")]
    pub bids_api: Option<f64>,
    /// The amount of investment offers made manually
    #[serde(rename = "BidsManual")]
    pub bids_manual: Option<f64>,
    /// Customer's Bondora username
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
    /// <para>Did the customer have prior credit history in Bondora</para>  <para>    false Customer had at least 3 months of credit history in Bondora</para>  <para>    true No prior credit history in Bondora</para>
    #[serde(rename = "NewCreditCustomer")]
    pub new_credit_customer: Option<bool>,
    /// Date when the loan application was started
    #[serde(rename = "LoanApplicationStartedDate")]
    pub loan_application_started_date: Option<String>,
    /// Date when the loan was issued
    #[serde(rename = "LoanDate")]
    pub loan_date: Option<String>,
    /// Date when the loan contract ended
    #[serde(rename = "ContractEndDate")]
    pub contract_end_date: Option<String>,
    /// First payment date according to initial loan schedule
    #[serde(rename = "FirstPaymentDate")]
    pub first_payment_date: Option<String>,
    /// Loan maturity date according to the original loan schedule
    #[serde(rename = "MaturityDate_Original")]
    pub maturity_date_original: Option<String>,
    /// Loan maturity date as of the report generation date
    #[serde(rename = "MaturityDate_Last")]
    pub maturity_date_last: Option<String>,
    /// Hour of signing the loan application
    #[serde(rename = "ApplicationSignedHour")]
    pub application_signed_hour: Option<i32>,
    /// Weekday of signing the loan application
    #[serde(rename = "ApplicationSignedWeekday")]
    pub application_signed_weekday: Option<i32>,
    /// Method used for loan application data verification
    #[serde(rename = "VerificationType")]
    pub verification_type: Option<i32>,
    /// Customer two letter language code
    #[serde(rename = "LanguageCode")]
    pub language_code: Option<i32>,
    /// Age of the borrower (years)
    #[serde(rename = "Age")]
    pub age: Option<i32>,
    /// DateOfBirth of the borrower
    #[serde(rename = "DateOfBirth")]
    pub date_of_birth: Option<String>,
    /// Borrower gender
    #[serde(rename = "Gender")]
    pub gender: Option<i32>,
    /// Residency of the borrower
    #[serde(rename = "Country")]
    pub country: Option<String>,
    /// County of the borrower
    #[serde(rename = "County")]
    pub county: Option<String>,
    /// City of the borrower
    #[serde(rename = "City")]
    pub city: Option<String>,
    /// Amount applied
    #[serde(rename = "AppliedAmount")]
    pub applied_amount: Option<f64>,
    /// Amount the borrower received
    #[serde(rename = "Amount")]
    pub amount: Option<f64>,
    /// Interest rate
    #[serde(rename = "Interest")]
    pub interest: Option<f64>,
    /// The loan term
    #[serde(rename = "LoanDuration")]
    pub loan_duration: Option<i32>,
    /// Estimated amount the borrower has to pay every month
    #[serde(rename = "MonthlyPayment")]
    pub monthly_payment: Option<i32>,
    /// Use of loan
    #[serde(rename = "UseOfLoan")]
    pub use_of_loan: Option<i32>,
    /// Education
    #[serde(rename = "Education")]
    pub education: Option<i32>,
    /// Marital status
    #[serde(rename = "MaritalStatus")]
    pub marital_status: Option<i32>,
    /// Number of children or other dependants
    #[serde(rename = "NrOfDependants")]
    pub nr_of_dependants: Option<String>,
    /// Employment status
    #[serde(rename = "EmploymentStatus")]
    pub employment_status: Option<i32>,
    /// Employment time with the current employer
    #[serde(rename = "EmploymentDurationCurrentEmployer")]
    pub employment_duration_current_employer: Option<String>,
    /// Employment position with the current employer
    #[serde(rename = "EmploymentPosition")]
    pub employment_position: Option<String>,
    /// Work experience in total
    #[serde(rename = "WorkExperience")]
    pub work_experience: Option<String>,
    /// Occupation area
    #[serde(rename = "OccupationArea")]
    pub occupation_area: Option<i32>,
    /// Home ownership type
    #[serde(rename = "HomeOwnershipType")]
    pub home_ownership_type: Option<i32>,
    /// Salary
    #[serde(rename = "IncomeFromPrincipalEmployer")]
    pub income_from_principal_employer: Option<f64>,
    /// Pension
    #[serde(rename = "IncomeFromPension")]
    pub income_from_pension: Option<f64>,
    /// Family allowance
    #[serde(rename = "IncomeFromFamilyAllowance")]
    pub income_from_family_allowance: Option<f64>,
    /// Social welfare
    #[serde(rename = "IncomeFromSocialWelfare")]
    pub income_from_social_welfare: Option<f64>,
    /// Leave pay
    #[serde(rename = "IncomeFromLeavePay")]
    pub income_from_leave_pay: Option<f64>,
    /// Child support
    #[serde(rename = "IncomeFromChildSupport")]
    pub income_from_child_support: Option<f64>,
    /// Other income
    #[serde(rename = "IncomeOther")]
    pub income_other: Option<f64>,
    /// Total income
    #[serde(rename = "IncomeTotal")]
    pub income_total: Option<f64>,
    /// Borrower's number of existing liabilities
    #[serde(rename = "ExistingLiabilities")]
    pub existing_liabilities: Option<i32>,
    /// The total amount of liabilities after refinancing
    #[serde(rename = "RefinanceLiabilities")]
    pub refinance_liabilities: Option<i32>,
    /// Total monthly liabilities
    #[serde(rename = "LiabilitiesTotal")]
    pub liabilities_total: Option<f64>,
    /// Debt to income ratio
    #[serde(rename = "DebtToIncome")]
    pub debt_to_income: Option<f64>,
    /// Discretionary income after monthly liabilities
    #[serde(rename = "FreeCash")]
    pub free_cash: Option<f64>,
    /// The day of the month the loan payments are scheduled for The actual date is adjusted for weekends and bank holidays (e.g. if 10th is Sunday then the payment will be made on the 11th in that month)
    #[serde(rename = "MonthlyPaymentDay")]
    pub monthly_payment_day: Option<i32>,
    /// Whether the first payment date has been reached according to the active schedule
    #[serde(rename = "ActiveScheduleFirstPaymentReached")]
    pub active_schedule_first_payment_reached: Option<bool>,
    /// According to active schedule the amount of principal the investment should have received
    #[serde(rename = "PlannedPrincipalTillDate")]
    pub planned_principal_till_date: Option<f64>,
    /// According to active schedule the amount of interest the investment should have received
    #[serde(rename = "PlannedInterestTillDate")]
    pub planned_interest_till_date: Option<f64>,
    /// The date of the current last payment received from the borrower
    #[serde(rename = "LastPaymentOn")]
    pub last_payment_on: Option<String>,
    /// How long the loan has been in Principal Debt
    #[serde(rename = "CurrentDebtDaysPrimary")]
    pub current_debt_days_primary: Option<i32>,
    /// The date when Principal Debt occurred
    #[serde(rename = "DebtOccuredOn")]
    pub debt_occured_on: Option<String>,
    /// How long the loan has been in Interest Debt
    #[serde(rename = "CurrentDebtDaysSecondary")]
    pub current_debt_days_secondary: Option<i32>,
    /// The date when Interest Debt occurred
    #[serde(rename = "DebtOccuredOnForSecondary")]
    pub debt_occured_on_for_secondary: Option<String>,
    /// Expected Loss calculated by the current Rating model
    #[serde(rename = "ExpectedLoss")]
    pub expected_loss: Option<f64>,
    /// Gives the percentage of outstanding exposure at the time of default that an investor is likely to lose if a loan actually defaults.  This means the proportion of funds lost for the investor after all expected recovery and accounting for the time value of the money recovered.  In general, LGD parameter is intended to be estimated based on the historical recoveries.  However, in new markets where limited experience does not allow us more precise loss given default estimates, a LGD of 90% is assumed.
    #[serde(rename = "LossGivenDefault")]
    pub loss_given_default: Option<f64>,
    /// Expected Return calculated by the current Rating model
    #[serde(rename = "ExpectedReturn")]
    pub expected_return: Option<f64>,
    /// Probability of Default, refers to a loan’s probability of default within one year horizon.
    #[serde(rename = "ProbabilityOfDefault")]
    pub probability_of_default: Option<f64>,
    /// The date when loan went into defaulted state and collection process was started
    #[serde(rename = "DefaultDate")]
    pub default_date: Option<String>,
    /// According to the current schedule, principal that is overdue
    #[serde(rename = "PrincipalOverdueBySchedule")]
    pub principal_overdue_by_schedule: Option<f64>,
    /// The amount of principal that was planned to be received after the default occurred
    #[serde(rename = "PlannedPrincipalPostDefault")]
    pub planned_principal_post_default: Option<f64>,
    /// The amount of interest that was planned to be received after the default occurred
    #[serde(rename = "PlannedInterestPostDefault")]
    pub planned_interest_post_default: Option<f64>,
    /// Exposure at default, outstanding principal at default
    #[serde(rename = "EAD1")]
    pub ead1: Option<f64>,
    /// Exposure at default, loan amount less all payments prior to default
    #[serde(rename = "EAD2")]
    pub ead2: Option<f64>,
    /// Principal recovered due to collection process from in debt loans
    #[serde(rename = "PrincipalRecovery")]
    pub principal_recovery: Option<f64>,
    /// Interest recovered due to collection process from in debt loans
    #[serde(rename = "InterestRecovery")]
    pub interest_recovery: Option<f64>,
    /// Current stage according to the recovery model 1 Collection 2 Recovery 3 Write Off
    #[serde(rename = "RecoveryStage")]
    pub recovery_stage: Option<i32>,
    /// How long the current recovery stage has been active
    #[serde(rename = "StageActiveSince")]
    pub stage_active_since: Option<String>,
    /// The version of the Rating model used for issuing the Bondora Rating
    #[serde(rename = "ModelVersion")]
    pub model_version: Option<i32>,
    /// Bondora Rating issued by the Rating model
    #[serde(rename = "Rating")]
    pub rating: Option<String>,
    /// Expected loss calculated by the specified version of Rating model
    #[serde(rename = "EL_V0")]
    pub el_v0: Option<f64>,
    /// Bondora Rating issued by version 0 of the Rating model
    #[serde(rename = "Rating_V0")]
    pub rating_v0: Option<String>,
    /// Expected loss calculated by the specified version of Rating model
    #[serde(rename = "EL_V1")]
    pub el_v1: Option<f64>,
    /// Bondora Rating issued by version 1 of the Rating model
    #[serde(rename = "Rating_V1")]
    pub rating_v1: Option<String>,
    /// Expected loss calculated by the specified version of Rating model
    #[serde(rename = "EL_V2")]
    pub el_v2: Option<f64>,
    /// Bondora Rating issued by version 2 of the Rating model
    #[serde(rename = "Rating_V2")]
    pub rating_v2: Option<String>,
    /// If Loan was cancelled
    #[serde(rename = "LoanCancelled")]
    pub loan_cancelled: Option<bool>,
    /// The current status of the loan application
    #[serde(rename = "Status")]
    pub status: Option<String>,
    /// The original maturity date of the loan has been increased by more than 60 days
    #[serde(rename = "Restructured")]
    pub restructured: Option<bool>,
    /// When a loan is in Principal Debt then it will be categorized by Principal Debt days
    #[serde(rename = "ActiveLateCategory")]
    pub active_late_category: Option<String>,
    /// Displays the last longest period of days when the loan was in Principal Debt
    #[serde(rename = "WorseLateCategory")]
    pub worse_late_category: Option<String>,
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
    /// Note owner received loan transfers principal amount
    #[serde(rename = "PrincipalPaymentsMade")]
    pub principal_payments_made: Option<f64>,
    /// Note owner received loan transfers earned interest, penalties total amount
    #[serde(rename = "InterestAndPenaltyPaymentsMade")]
    pub interest_and_penalty_payments_made: Option<f64>,
    /// Principal that was written off on the investment
    #[serde(rename = "PrincipalWriteOffs")]
    pub principal_write_offs: Option<f64>,
    /// Interest that was written off on the investment
    #[serde(rename = "InterestAndPenaltyWriteOffs")]
    pub interest_and_penalty_write_offs: Option<f64>,
    /// Service cost related to the recovery of the debt based on the principal of the investment
    #[serde(rename = "PrincipalDebtServicingCost")]
    pub principal_debt_servicing_cost: Option<f64>,
    /// Service cost related to the recovery of the debt based on the interest and penalties of the investment
    #[serde(rename = "InterestAndPenaltyDebtServicingCost")]
    pub interest_and_penalty_debt_servicing_cost: Option<f64>,
    /// Principal that still needs to be paid by the borrower
    #[serde(rename = "PrincipalBalance")]
    pub principal_balance: Option<f64>,
    /// Unpaid interest and penalties
    #[serde(rename = "InterestAndPenaltyBalance")]
    pub interest_and_penalty_balance: Option<f64>,
    /// Number of previous loans
    #[serde(rename = "NoOfPreviousLoansBeforeLoan")]
    pub no_of_previous_loans_before_loan: Option<i32>,
    /// Value of previous loans
    #[serde(rename = "AmountOfPreviousLoansBeforeLoan")]
    pub amount_of_previous_loans_before_loan: Option<f64>,
    /// How much the borrower had repaid before the loan
    #[serde(rename = "PreviousRepaymentsBeforeLoan")]
    pub previous_repayments_before_loan: Option<f64>,
    /// Previous early repaid amount before this loan
    #[serde(rename = "PreviousEarlyRepaymentsBeforeLoan")]
    pub previous_early_repayments_before_loan: Option<f64>,
    /// Previous early repaid loans before this loan
    #[serde(rename = "PreviousEarlyRepaymentsCountBeforeLoan")]
    pub previous_early_repayments_count_before_loan: Option<i32>,
    /// Date of the beginning of Grace period
    #[serde(rename = "GracePeriodStart")]
    pub grace_period_start: Option<String>,
    /// Date of the end of Grace period
    #[serde(rename = "GracePeriodEnd")]
    pub grace_period_end: Option<String>,
    /// According to schedule the next date for borrower to make their payment
    #[serde(rename = "NextPaymentDate")]
    pub next_payment_date: Option<String>,
    /// According to schedule the number of the next payment
    #[serde(rename = "NextPaymentNr")]
    pub next_payment_nr: Option<i32>,
    /// According to schedule the count of scheduled payments
    #[serde(rename = "NrOfScheduledPayments")]
    pub nr_of_scheduled_payments: Option<i32>,
    /// The date when the a new schedule was assigned to the borrower
    #[serde(rename = "ReScheduledOn")]
    pub re_scheduled_on: Option<String>,
}
