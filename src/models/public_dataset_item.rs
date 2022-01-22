
/// PublicDatasetItem : Represents Public dataset fields

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicDatasetItem {
  /// A unique ID given to a loan
  #[serde(rename = "LoanId")]
  loan_id: Option<String>,
  /// A unique loan number displayed in Bondora's system
  #[serde(rename = "LoanNumber")]
  loan_number: Option<i32>,
  /// Date when the loan application appeared on Primary Market
  #[serde(rename = "ListedOnUTC")]
  listed_on_utc: Option<String>,
  /// Date when the auction bidding started on
  #[serde(rename = "BiddingStartedOn")]
  bidding_started_on: Option<String>,
  /// The amount of investment offers made by Portfolio Managers
  #[serde(rename = "BidsPortfolioManager")]
  bids_portfolio_manager: Option<f64>,
  /// The amount of investment offers made via Api
  #[serde(rename = "BidsApi")]
  bids_api: Option<f64>,
  /// The amount of investment offers made manually
  #[serde(rename = "BidsManual")]
  bids_manual: Option<f64>,
  /// Customer's Bondora username
  #[serde(rename = "UserName")]
  user_name: Option<String>,
  /// <para>Did the customer have prior credit history in Bondora</para>  <para>    false Customer had at least 3 months of credit history in Bondora</para>  <para>    true No prior credit history in Bondora</para>
  #[serde(rename = "NewCreditCustomer")]
  new_credit_customer: Option<bool>,
  /// Date when the loan application was started
  #[serde(rename = "LoanApplicationStartedDate")]
  loan_application_started_date: Option<String>,
  /// Date when the loan was issued
  #[serde(rename = "LoanDate")]
  loan_date: Option<String>,
  /// Date when the loan contract ended
  #[serde(rename = "ContractEndDate")]
  contract_end_date: Option<String>,
  /// First payment date according to initial loan schedule
  #[serde(rename = "FirstPaymentDate")]
  first_payment_date: Option<String>,
  /// Loan maturity date according to the original loan schedule
  #[serde(rename = "MaturityDate_Original")]
  maturity_date_original: Option<String>,
  /// Loan maturity date as of the report generation date
  #[serde(rename = "MaturityDate_Last")]
  maturity_date_last: Option<String>,
  /// Hour of signing the loan application
  #[serde(rename = "ApplicationSignedHour")]
  application_signed_hour: Option<i32>,
  /// Weekday of signing the loan application
  #[serde(rename = "ApplicationSignedWeekday")]
  application_signed_weekday: Option<i32>,
  /// Method used for loan application data verification
  #[serde(rename = "VerificationType")]
  verification_type: Option<i32>,
  /// Customer two letter language code
  #[serde(rename = "LanguageCode")]
  language_code: Option<i32>,
  /// Age of the borrower (years)
  #[serde(rename = "Age")]
  age: Option<i32>,
  /// DateOfBirth of the borrower
  #[serde(rename = "DateOfBirth")]
  date_of_birth: Option<String>,
  /// Borrower gender
  #[serde(rename = "Gender")]
  gender: Option<i32>,
  /// Residency of the borrower
  #[serde(rename = "Country")]
  country: Option<String>,
  /// County of the borrower
  #[serde(rename = "County")]
  county: Option<String>,
  /// City of the borrower
  #[serde(rename = "City")]
  city: Option<String>,
  /// Amount applied
  #[serde(rename = "AppliedAmount")]
  applied_amount: Option<f64>,
  /// Amount the borrower received
  #[serde(rename = "Amount")]
  amount: Option<f64>,
  /// Interest rate
  #[serde(rename = "Interest")]
  interest: Option<f64>,
  /// The loan term
  #[serde(rename = "LoanDuration")]
  loan_duration: Option<i32>,
  /// Estimated amount the borrower has to pay every month
  #[serde(rename = "MonthlyPayment")]
  monthly_payment: Option<i32>,
  /// Use of loan
  #[serde(rename = "UseOfLoan")]
  use_of_loan: Option<i32>,
  /// Education
  #[serde(rename = "Education")]
  education: Option<i32>,
  /// Marital status
  #[serde(rename = "MaritalStatus")]
  marital_status: Option<i32>,
  /// Number of children or other dependants
  #[serde(rename = "NrOfDependants")]
  nr_of_dependants: Option<String>,
  /// Employment status
  #[serde(rename = "EmploymentStatus")]
  employment_status: Option<i32>,
  /// Employment time with the current employer
  #[serde(rename = "EmploymentDurationCurrentEmployer")]
  employment_duration_current_employer: Option<String>,
  /// Employment position with the current employer
  #[serde(rename = "EmploymentPosition")]
  employment_position: Option<String>,
  /// Work experience in total
  #[serde(rename = "WorkExperience")]
  work_experience: Option<String>,
  /// Occupation area
  #[serde(rename = "OccupationArea")]
  occupation_area: Option<i32>,
  /// Home ownership type
  #[serde(rename = "HomeOwnershipType")]
  home_ownership_type: Option<i32>,
  /// Salary
  #[serde(rename = "IncomeFromPrincipalEmployer")]
  income_from_principal_employer: Option<f64>,
  /// Pension
  #[serde(rename = "IncomeFromPension")]
  income_from_pension: Option<f64>,
  /// Family allowance
  #[serde(rename = "IncomeFromFamilyAllowance")]
  income_from_family_allowance: Option<f64>,
  /// Social welfare
  #[serde(rename = "IncomeFromSocialWelfare")]
  income_from_social_welfare: Option<f64>,
  /// Leave pay
  #[serde(rename = "IncomeFromLeavePay")]
  income_from_leave_pay: Option<f64>,
  /// Child support
  #[serde(rename = "IncomeFromChildSupport")]
  income_from_child_support: Option<f64>,
  /// Other income
  #[serde(rename = "IncomeOther")]
  income_other: Option<f64>,
  /// Total income
  #[serde(rename = "IncomeTotal")]
  income_total: Option<f64>,
  /// Borrower's number of existing liabilities
  #[serde(rename = "ExistingLiabilities")]
  existing_liabilities: Option<i32>,
  /// The total amount of liabilities after refinancing
  #[serde(rename = "RefinanceLiabilities")]
  refinance_liabilities: Option<i32>,
  /// Total monthly liabilities
  #[serde(rename = "LiabilitiesTotal")]
  liabilities_total: Option<f64>,
  /// Debt to income ratio
  #[serde(rename = "DebtToIncome")]
  debt_to_income: Option<f64>,
  /// Discretionary income after monthly liabilities
  #[serde(rename = "FreeCash")]
  free_cash: Option<f64>,
  /// The day of the month the loan payments are scheduled for The actual date is adjusted for weekends and bank holidays (e.g. if 10th is Sunday then the payment will be made on the 11th in that month)
  #[serde(rename = "MonthlyPaymentDay")]
  monthly_payment_day: Option<i32>,
  /// Whether the first payment date has been reached according to the active schedule
  #[serde(rename = "ActiveScheduleFirstPaymentReached")]
  active_schedule_first_payment_reached: Option<bool>,
  /// According to active schedule the amount of principal the investment should have received
  #[serde(rename = "PlannedPrincipalTillDate")]
  planned_principal_till_date: Option<f64>,
  /// According to active schedule the amount of interest the investment should have received
  #[serde(rename = "PlannedInterestTillDate")]
  planned_interest_till_date: Option<f64>,
  /// The date of the current last payment received from the borrower
  #[serde(rename = "LastPaymentOn")]
  last_payment_on: Option<String>,
  /// How long the loan has been in Principal Debt
  #[serde(rename = "CurrentDebtDaysPrimary")]
  current_debt_days_primary: Option<i32>,
  /// The date when Principal Debt occurred
  #[serde(rename = "DebtOccuredOn")]
  debt_occured_on: Option<String>,
  /// How long the loan has been in Interest Debt
  #[serde(rename = "CurrentDebtDaysSecondary")]
  current_debt_days_secondary: Option<i32>,
  /// The date when Interest Debt occurred
  #[serde(rename = "DebtOccuredOnForSecondary")]
  debt_occured_on_for_secondary: Option<String>,
  /// Expected Loss calculated by the current Rating model
  #[serde(rename = "ExpectedLoss")]
  expected_loss: Option<f64>,
  /// Gives the percentage of outstanding exposure at the time of default that an investor is likely to lose if a loan actually defaults.  This means the proportion of funds lost for the investor after all expected recovery and accounting for the time value of the money recovered.  In general, LGD parameter is intended to be estimated based on the historical recoveries.  However, in new markets where limited experience does not allow us more precise loss given default estimates, a LGD of 90% is assumed.
  #[serde(rename = "LossGivenDefault")]
  loss_given_default: Option<f64>,
  /// Expected Return calculated by the current Rating model
  #[serde(rename = "ExpectedReturn")]
  expected_return: Option<f64>,
  /// Probability of Default, refers to a loan’s probability of default within one year horizon.
  #[serde(rename = "ProbabilityOfDefault")]
  probability_of_default: Option<f64>,
  /// The date when loan went into defaulted state and collection process was started
  #[serde(rename = "DefaultDate")]
  default_date: Option<String>,
  /// According to the current schedule, principal that is overdue
  #[serde(rename = "PrincipalOverdueBySchedule")]
  principal_overdue_by_schedule: Option<f64>,
  /// The amount of principal that was planned to be received after the default occurred
  #[serde(rename = "PlannedPrincipalPostDefault")]
  planned_principal_post_default: Option<f64>,
  /// The amount of interest that was planned to be received after the default occurred
  #[serde(rename = "PlannedInterestPostDefault")]
  planned_interest_post_default: Option<f64>,
  /// Exposure at default, outstanding principal at default
  #[serde(rename = "EAD1")]
  ead1: Option<f64>,
  /// Exposure at default, loan amount less all payments prior to default
  #[serde(rename = "EAD2")]
  ead2: Option<f64>,
  /// Principal recovered due to collection process from in debt loans
  #[serde(rename = "PrincipalRecovery")]
  principal_recovery: Option<f64>,
  /// Interest recovered due to collection process from in debt loans
  #[serde(rename = "InterestRecovery")]
  interest_recovery: Option<f64>,
  /// Current stage according to the recovery model 1 Collection 2 Recovery 3 Write Off
  #[serde(rename = "RecoveryStage")]
  recovery_stage: Option<i32>,
  /// How long the current recovery stage has been active
  #[serde(rename = "StageActiveSince")]
  stage_active_since: Option<String>,
  /// The version of the Rating model used for issuing the Bondora Rating
  #[serde(rename = "ModelVersion")]
  model_version: Option<i32>,
  /// Bondora Rating issued by the Rating model
  #[serde(rename = "Rating")]
  rating: Option<String>,
  /// Expected loss calculated by the specified version of Rating model
  #[serde(rename = "EL_V0")]
  el_v0: Option<f64>,
  /// Bondora Rating issued by version 0 of the Rating model
  #[serde(rename = "Rating_V0")]
  rating_v0: Option<String>,
  /// Expected loss calculated by the specified version of Rating model
  #[serde(rename = "EL_V1")]
  el_v1: Option<f64>,
  /// Bondora Rating issued by version 1 of the Rating model
  #[serde(rename = "Rating_V1")]
  rating_v1: Option<String>,
  /// Expected loss calculated by the specified version of Rating model
  #[serde(rename = "EL_V2")]
  el_v2: Option<f64>,
  /// Bondora Rating issued by version 2 of the Rating model
  #[serde(rename = "Rating_V2")]
  rating_v2: Option<String>,
  /// If Loan was cancelled
  #[serde(rename = "LoanCancelled")]
  loan_cancelled: Option<bool>,
  /// The current status of the loan application
  #[serde(rename = "Status")]
  status: Option<String>,
  /// The original maturity date of the loan has been increased by more than 60 days
  #[serde(rename = "Restructured")]
  restructured: Option<bool>,
  /// When a loan is in Principal Debt then it will be categorized by Principal Debt days
  #[serde(rename = "ActiveLateCategory")]
  active_late_category: Option<String>,
  /// Displays the last longest period of days when the loan was in Principal Debt
  #[serde(rename = "WorseLateCategory")]
  worse_late_category: Option<String>,
  /// A score that is specifically designed for risk classifying subprime borrowers (defined by Equifax as borrowers that do not have access to bank loans).  A measure of the probability of default one month ahead.  <para>The score is given on a 10-grade scale, from the best score to the worst:</para><para>M1, M2, M3, M4, M5, M6, M7, M8, M9, M10</para>
  #[serde(rename = "CreditScoreEsMicroL")]
  credit_score_es_micro_l: Option<String>,
  /// Generic score for the loan applicants that do not have active past due operations in ASNEF.  A measure of the probability of default one year ahead.  The score is given on a 6-grade scale.  <para>AAA Very low</para><para>AA Low</para><para>A Average</para><para>B Average High</para><para>C High</para><para>D Very High</para>
  #[serde(rename = "CreditScoreEsEquifaxRisk")]
  credit_score_es_equifax_risk: Option<String>,
  /// Credit Scoring model for Finnish Asiakastieto  <para>RL1 Very low risk 01-20</para><para>RL2 Low risk 21-40</para><para>RL3 Average risk 41-60</para><para>RL4 Big risk 61-80</para><para>RL5 Huge risk 81-100</para>
  #[serde(rename = "CreditScoreFiAsiakasTietoRiskGrade")]
  credit_score_fi_asiakas_tieto_risk_grade: Option<String>,
  /// Credit scoring for Estonian loans  <para>1000 No previous payments problems</para><para>900 Payments problems finished 24-36 months ago</para><para>800 Payments problems finished 12-24 months ago</para><para>700 Payments problems finished 6-12 months ago</para><para>600 Payment problems finished &lt;6 months ago</para><para>500 Active payment problems</para>
  #[serde(rename = "CreditScoreEeMini")]
  credit_score_ee_mini: Option<String>,
  /// Note owner received loan transfers principal amount
  #[serde(rename = "PrincipalPaymentsMade")]
  principal_payments_made: Option<f64>,
  /// Note owner received loan transfers earned interest, penalties total amount
  #[serde(rename = "InterestAndPenaltyPaymentsMade")]
  interest_and_penalty_payments_made: Option<f64>,
  /// Principal that was written off on the investment
  #[serde(rename = "PrincipalWriteOffs")]
  principal_write_offs: Option<f64>,
  /// Interest that was written off on the investment
  #[serde(rename = "InterestAndPenaltyWriteOffs")]
  interest_and_penalty_write_offs: Option<f64>,
  /// Service cost related to the recovery of the debt based on the principal of the investment
  #[serde(rename = "PrincipalDebtServicingCost")]
  principal_debt_servicing_cost: Option<f64>,
  /// Service cost related to the recovery of the debt based on the interest and penalties of the investment
  #[serde(rename = "InterestAndPenaltyDebtServicingCost")]
  interest_and_penalty_debt_servicing_cost: Option<f64>,
  /// Principal that still needs to be paid by the borrower
  #[serde(rename = "PrincipalBalance")]
  principal_balance: Option<f64>,
  /// Unpaid interest and penalties
  #[serde(rename = "InterestAndPenaltyBalance")]
  interest_and_penalty_balance: Option<f64>,
  /// Number of previous loans
  #[serde(rename = "NoOfPreviousLoansBeforeLoan")]
  no_of_previous_loans_before_loan: Option<i32>,
  /// Value of previous loans
  #[serde(rename = "AmountOfPreviousLoansBeforeLoan")]
  amount_of_previous_loans_before_loan: Option<f64>,
  /// How much the borrower had repaid before the loan
  #[serde(rename = "PreviousRepaymentsBeforeLoan")]
  previous_repayments_before_loan: Option<f64>,
  /// Previous early repaid amount before this loan
  #[serde(rename = "PreviousEarlyRepaymentsBeforeLoan")]
  previous_early_repayments_before_loan: Option<f64>,
  /// Previous early repaid loans before this loan
  #[serde(rename = "PreviousEarlyRepaymentsCountBeforeLoan")]
  previous_early_repayments_count_before_loan: Option<i32>,
  /// Date of the beginning of Grace period
  #[serde(rename = "GracePeriodStart")]
  grace_period_start: Option<String>,
  /// Date of the end of Grace period
  #[serde(rename = "GracePeriodEnd")]
  grace_period_end: Option<String>,
  /// According to schedule the next date for borrower to make their payment
  #[serde(rename = "NextPaymentDate")]
  next_payment_date: Option<String>,
  /// According to schedule the number of the next payment
  #[serde(rename = "NextPaymentNr")]
  next_payment_nr: Option<i32>,
  /// According to schedule the count of scheduled payments
  #[serde(rename = "NrOfScheduledPayments")]
  nr_of_scheduled_payments: Option<i32>,
  /// The date when the a new schedule was assigned to the borrower
  #[serde(rename = "ReScheduledOn")]
  re_scheduled_on: Option<String>
}

impl PublicDatasetItem {
  /// Represents Public dataset fields
  pub fn new() -> PublicDatasetItem {
    PublicDatasetItem {
      loan_id: None,
      loan_number: None,
      listed_on_utc: None,
      bidding_started_on: None,
      bids_portfolio_manager: None,
      bids_api: None,
      bids_manual: None,
      user_name: None,
      new_credit_customer: None,
      loan_application_started_date: None,
      loan_date: None,
      contract_end_date: None,
      first_payment_date: None,
      maturity_date_original: None,
      maturity_date_last: None,
      application_signed_hour: None,
      application_signed_weekday: None,
      verification_type: None,
      language_code: None,
      age: None,
      date_of_birth: None,
      gender: None,
      country: None,
      county: None,
      city: None,
      applied_amount: None,
      amount: None,
      interest: None,
      loan_duration: None,
      monthly_payment: None,
      use_of_loan: None,
      education: None,
      marital_status: None,
      nr_of_dependants: None,
      employment_status: None,
      employment_duration_current_employer: None,
      employment_position: None,
      work_experience: None,
      occupation_area: None,
      home_ownership_type: None,
      income_from_principal_employer: None,
      income_from_pension: None,
      income_from_family_allowance: None,
      income_from_social_welfare: None,
      income_from_leave_pay: None,
      income_from_child_support: None,
      income_other: None,
      income_total: None,
      existing_liabilities: None,
      refinance_liabilities: None,
      liabilities_total: None,
      debt_to_income: None,
      free_cash: None,
      monthly_payment_day: None,
      active_schedule_first_payment_reached: None,
      planned_principal_till_date: None,
      planned_interest_till_date: None,
      last_payment_on: None,
      current_debt_days_primary: None,
      debt_occured_on: None,
      current_debt_days_secondary: None,
      debt_occured_on_for_secondary: None,
      expected_loss: None,
      loss_given_default: None,
      expected_return: None,
      probability_of_default: None,
      default_date: None,
      principal_overdue_by_schedule: None,
      planned_principal_post_default: None,
      planned_interest_post_default: None,
      ead1: None,
      ead2: None,
      principal_recovery: None,
      interest_recovery: None,
      recovery_stage: None,
      stage_active_since: None,
      model_version: None,
      rating: None,
      el_v0: None,
      rating_v0: None,
      el_v1: None,
      rating_v1: None,
      el_v2: None,
      rating_v2: None,
      loan_cancelled: None,
      status: None,
      restructured: None,
      active_late_category: None,
      worse_late_category: None,
      credit_score_es_micro_l: None,
      credit_score_es_equifax_risk: None,
      credit_score_fi_asiakas_tieto_risk_grade: None,
      credit_score_ee_mini: None,
      principal_payments_made: None,
      interest_and_penalty_payments_made: None,
      principal_write_offs: None,
      interest_and_penalty_write_offs: None,
      principal_debt_servicing_cost: None,
      interest_and_penalty_debt_servicing_cost: None,
      principal_balance: None,
      interest_and_penalty_balance: None,
      no_of_previous_loans_before_loan: None,
      amount_of_previous_loans_before_loan: None,
      previous_repayments_before_loan: None,
      previous_early_repayments_before_loan: None,
      previous_early_repayments_count_before_loan: None,
      grace_period_start: None,
      grace_period_end: None,
      next_payment_date: None,
      next_payment_nr: None,
      nr_of_scheduled_payments: None,
      re_scheduled_on: None
    }
  }

  pub fn set_loan_id(&mut self, loan_id: String) {
    self.loan_id = Some(loan_id);
  }

  pub fn with_loan_id(mut self, loan_id: String) -> PublicDatasetItem {
    self.loan_id = Some(loan_id);
    self
  }

  pub fn loan_id(&self) -> Option<&String> {
    self.loan_id.as_ref()
  }

  pub fn reset_loan_id(&mut self) {
    self.loan_id = None;
  }

  pub fn set_loan_number(&mut self, loan_number: i32) {
    self.loan_number = Some(loan_number);
  }

  pub fn with_loan_number(mut self, loan_number: i32) -> PublicDatasetItem {
    self.loan_number = Some(loan_number);
    self
  }

  pub fn loan_number(&self) -> Option<&i32> {
    self.loan_number.as_ref()
  }

  pub fn reset_loan_number(&mut self) {
    self.loan_number = None;
  }

  pub fn set_listed_on_utc(&mut self, listed_on_utc: String) {
    self.listed_on_utc = Some(listed_on_utc);
  }

  pub fn with_listed_on_utc(mut self, listed_on_utc: String) -> PublicDatasetItem {
    self.listed_on_utc = Some(listed_on_utc);
    self
  }

  pub fn listed_on_utc(&self) -> Option<&String> {
    self.listed_on_utc.as_ref()
  }

  pub fn reset_listed_on_utc(&mut self) {
    self.listed_on_utc = None;
  }

  pub fn set_bidding_started_on(&mut self, bidding_started_on: String) {
    self.bidding_started_on = Some(bidding_started_on);
  }

  pub fn with_bidding_started_on(mut self, bidding_started_on: String) -> PublicDatasetItem {
    self.bidding_started_on = Some(bidding_started_on);
    self
  }

  pub fn bidding_started_on(&self) -> Option<&String> {
    self.bidding_started_on.as_ref()
  }

  pub fn reset_bidding_started_on(&mut self) {
    self.bidding_started_on = None;
  }

  pub fn set_bids_portfolio_manager(&mut self, bids_portfolio_manager: f64) {
    self.bids_portfolio_manager = Some(bids_portfolio_manager);
  }

  pub fn with_bids_portfolio_manager(mut self, bids_portfolio_manager: f64) -> PublicDatasetItem {
    self.bids_portfolio_manager = Some(bids_portfolio_manager);
    self
  }

  pub fn bids_portfolio_manager(&self) -> Option<&f64> {
    self.bids_portfolio_manager.as_ref()
  }

  pub fn reset_bids_portfolio_manager(&mut self) {
    self.bids_portfolio_manager = None;
  }

  pub fn set_bids_api(&mut self, bids_api: f64) {
    self.bids_api = Some(bids_api);
  }

  pub fn with_bids_api(mut self, bids_api: f64) -> PublicDatasetItem {
    self.bids_api = Some(bids_api);
    self
  }

  pub fn bids_api(&self) -> Option<&f64> {
    self.bids_api.as_ref()
  }

  pub fn reset_bids_api(&mut self) {
    self.bids_api = None;
  }

  pub fn set_bids_manual(&mut self, bids_manual: f64) {
    self.bids_manual = Some(bids_manual);
  }

  pub fn with_bids_manual(mut self, bids_manual: f64) -> PublicDatasetItem {
    self.bids_manual = Some(bids_manual);
    self
  }

  pub fn bids_manual(&self) -> Option<&f64> {
    self.bids_manual.as_ref()
  }

  pub fn reset_bids_manual(&mut self) {
    self.bids_manual = None;
  }

  pub fn set_user_name(&mut self, user_name: String) {
    self.user_name = Some(user_name);
  }

  pub fn with_user_name(mut self, user_name: String) -> PublicDatasetItem {
    self.user_name = Some(user_name);
    self
  }

  pub fn user_name(&self) -> Option<&String> {
    self.user_name.as_ref()
  }

  pub fn reset_user_name(&mut self) {
    self.user_name = None;
  }

  pub fn set_new_credit_customer(&mut self, new_credit_customer: bool) {
    self.new_credit_customer = Some(new_credit_customer);
  }

  pub fn with_new_credit_customer(mut self, new_credit_customer: bool) -> PublicDatasetItem {
    self.new_credit_customer = Some(new_credit_customer);
    self
  }

  pub fn new_credit_customer(&self) -> Option<&bool> {
    self.new_credit_customer.as_ref()
  }

  pub fn reset_new_credit_customer(&mut self) {
    self.new_credit_customer = None;
  }

  pub fn set_loan_application_started_date(&mut self, loan_application_started_date: String) {
    self.loan_application_started_date = Some(loan_application_started_date);
  }

  pub fn with_loan_application_started_date(mut self, loan_application_started_date: String) -> PublicDatasetItem {
    self.loan_application_started_date = Some(loan_application_started_date);
    self
  }

  pub fn loan_application_started_date(&self) -> Option<&String> {
    self.loan_application_started_date.as_ref()
  }

  pub fn reset_loan_application_started_date(&mut self) {
    self.loan_application_started_date = None;
  }

  pub fn set_loan_date(&mut self, loan_date: String) {
    self.loan_date = Some(loan_date);
  }

  pub fn with_loan_date(mut self, loan_date: String) -> PublicDatasetItem {
    self.loan_date = Some(loan_date);
    self
  }

  pub fn loan_date(&self) -> Option<&String> {
    self.loan_date.as_ref()
  }

  pub fn reset_loan_date(&mut self) {
    self.loan_date = None;
  }

  pub fn set_contract_end_date(&mut self, contract_end_date: String) {
    self.contract_end_date = Some(contract_end_date);
  }

  pub fn with_contract_end_date(mut self, contract_end_date: String) -> PublicDatasetItem {
    self.contract_end_date = Some(contract_end_date);
    self
  }

  pub fn contract_end_date(&self) -> Option<&String> {
    self.contract_end_date.as_ref()
  }

  pub fn reset_contract_end_date(&mut self) {
    self.contract_end_date = None;
  }

  pub fn set_first_payment_date(&mut self, first_payment_date: String) {
    self.first_payment_date = Some(first_payment_date);
  }

  pub fn with_first_payment_date(mut self, first_payment_date: String) -> PublicDatasetItem {
    self.first_payment_date = Some(first_payment_date);
    self
  }

  pub fn first_payment_date(&self) -> Option<&String> {
    self.first_payment_date.as_ref()
  }

  pub fn reset_first_payment_date(&mut self) {
    self.first_payment_date = None;
  }

  pub fn set_maturity_date_original(&mut self, maturity_date_original: String) {
    self.maturity_date_original = Some(maturity_date_original);
  }

  pub fn with_maturity_date_original(mut self, maturity_date_original: String) -> PublicDatasetItem {
    self.maturity_date_original = Some(maturity_date_original);
    self
  }

  pub fn maturity_date_original(&self) -> Option<&String> {
    self.maturity_date_original.as_ref()
  }

  pub fn reset_maturity_date_original(&mut self) {
    self.maturity_date_original = None;
  }

  pub fn set_maturity_date_last(&mut self, maturity_date_last: String) {
    self.maturity_date_last = Some(maturity_date_last);
  }

  pub fn with_maturity_date_last(mut self, maturity_date_last: String) -> PublicDatasetItem {
    self.maturity_date_last = Some(maturity_date_last);
    self
  }

  pub fn maturity_date_last(&self) -> Option<&String> {
    self.maturity_date_last.as_ref()
  }

  pub fn reset_maturity_date_last(&mut self) {
    self.maturity_date_last = None;
  }

  pub fn set_application_signed_hour(&mut self, application_signed_hour: i32) {
    self.application_signed_hour = Some(application_signed_hour);
  }

  pub fn with_application_signed_hour(mut self, application_signed_hour: i32) -> PublicDatasetItem {
    self.application_signed_hour = Some(application_signed_hour);
    self
  }

  pub fn application_signed_hour(&self) -> Option<&i32> {
    self.application_signed_hour.as_ref()
  }

  pub fn reset_application_signed_hour(&mut self) {
    self.application_signed_hour = None;
  }

  pub fn set_application_signed_weekday(&mut self, application_signed_weekday: i32) {
    self.application_signed_weekday = Some(application_signed_weekday);
  }

  pub fn with_application_signed_weekday(mut self, application_signed_weekday: i32) -> PublicDatasetItem {
    self.application_signed_weekday = Some(application_signed_weekday);
    self
  }

  pub fn application_signed_weekday(&self) -> Option<&i32> {
    self.application_signed_weekday.as_ref()
  }

  pub fn reset_application_signed_weekday(&mut self) {
    self.application_signed_weekday = None;
  }

  pub fn set_verification_type(&mut self, verification_type: i32) {
    self.verification_type = Some(verification_type);
  }

  pub fn with_verification_type(mut self, verification_type: i32) -> PublicDatasetItem {
    self.verification_type = Some(verification_type);
    self
  }

  pub fn verification_type(&self) -> Option<&i32> {
    self.verification_type.as_ref()
  }

  pub fn reset_verification_type(&mut self) {
    self.verification_type = None;
  }

  pub fn set_language_code(&mut self, language_code: i32) {
    self.language_code = Some(language_code);
  }

  pub fn with_language_code(mut self, language_code: i32) -> PublicDatasetItem {
    self.language_code = Some(language_code);
    self
  }

  pub fn language_code(&self) -> Option<&i32> {
    self.language_code.as_ref()
  }

  pub fn reset_language_code(&mut self) {
    self.language_code = None;
  }

  pub fn set_age(&mut self, age: i32) {
    self.age = Some(age);
  }

  pub fn with_age(mut self, age: i32) -> PublicDatasetItem {
    self.age = Some(age);
    self
  }

  pub fn age(&self) -> Option<&i32> {
    self.age.as_ref()
  }

  pub fn reset_age(&mut self) {
    self.age = None;
  }

  pub fn set_date_of_birth(&mut self, date_of_birth: String) {
    self.date_of_birth = Some(date_of_birth);
  }

  pub fn with_date_of_birth(mut self, date_of_birth: String) -> PublicDatasetItem {
    self.date_of_birth = Some(date_of_birth);
    self
  }

  pub fn date_of_birth(&self) -> Option<&String> {
    self.date_of_birth.as_ref()
  }

  pub fn reset_date_of_birth(&mut self) {
    self.date_of_birth = None;
  }

  pub fn set_gender(&mut self, gender: i32) {
    self.gender = Some(gender);
  }

  pub fn with_gender(mut self, gender: i32) -> PublicDatasetItem {
    self.gender = Some(gender);
    self
  }

  pub fn gender(&self) -> Option<&i32> {
    self.gender.as_ref()
  }

  pub fn reset_gender(&mut self) {
    self.gender = None;
  }

  pub fn set_country(&mut self, country: String) {
    self.country = Some(country);
  }

  pub fn with_country(mut self, country: String) -> PublicDatasetItem {
    self.country = Some(country);
    self
  }

  pub fn country(&self) -> Option<&String> {
    self.country.as_ref()
  }

  pub fn reset_country(&mut self) {
    self.country = None;
  }

  pub fn set_county(&mut self, county: String) {
    self.county = Some(county);
  }

  pub fn with_county(mut self, county: String) -> PublicDatasetItem {
    self.county = Some(county);
    self
  }

  pub fn county(&self) -> Option<&String> {
    self.county.as_ref()
  }

  pub fn reset_county(&mut self) {
    self.county = None;
  }

  pub fn set_city(&mut self, city: String) {
    self.city = Some(city);
  }

  pub fn with_city(mut self, city: String) -> PublicDatasetItem {
    self.city = Some(city);
    self
  }

  pub fn city(&self) -> Option<&String> {
    self.city.as_ref()
  }

  pub fn reset_city(&mut self) {
    self.city = None;
  }

  pub fn set_applied_amount(&mut self, applied_amount: f64) {
    self.applied_amount = Some(applied_amount);
  }

  pub fn with_applied_amount(mut self, applied_amount: f64) -> PublicDatasetItem {
    self.applied_amount = Some(applied_amount);
    self
  }

  pub fn applied_amount(&self) -> Option<&f64> {
    self.applied_amount.as_ref()
  }

  pub fn reset_applied_amount(&mut self) {
    self.applied_amount = None;
  }

  pub fn set_amount(&mut self, amount: f64) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: f64) -> PublicDatasetItem {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&f64> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_interest(&mut self, interest: f64) {
    self.interest = Some(interest);
  }

  pub fn with_interest(mut self, interest: f64) -> PublicDatasetItem {
    self.interest = Some(interest);
    self
  }

  pub fn interest(&self) -> Option<&f64> {
    self.interest.as_ref()
  }

  pub fn reset_interest(&mut self) {
    self.interest = None;
  }

  pub fn set_loan_duration(&mut self, loan_duration: i32) {
    self.loan_duration = Some(loan_duration);
  }

  pub fn with_loan_duration(mut self, loan_duration: i32) -> PublicDatasetItem {
    self.loan_duration = Some(loan_duration);
    self
  }

  pub fn loan_duration(&self) -> Option<&i32> {
    self.loan_duration.as_ref()
  }

  pub fn reset_loan_duration(&mut self) {
    self.loan_duration = None;
  }

  pub fn set_monthly_payment(&mut self, monthly_payment: i32) {
    self.monthly_payment = Some(monthly_payment);
  }

  pub fn with_monthly_payment(mut self, monthly_payment: i32) -> PublicDatasetItem {
    self.monthly_payment = Some(monthly_payment);
    self
  }

  pub fn monthly_payment(&self) -> Option<&i32> {
    self.monthly_payment.as_ref()
  }

  pub fn reset_monthly_payment(&mut self) {
    self.monthly_payment = None;
  }

  pub fn set_use_of_loan(&mut self, use_of_loan: i32) {
    self.use_of_loan = Some(use_of_loan);
  }

  pub fn with_use_of_loan(mut self, use_of_loan: i32) -> PublicDatasetItem {
    self.use_of_loan = Some(use_of_loan);
    self
  }

  pub fn use_of_loan(&self) -> Option<&i32> {
    self.use_of_loan.as_ref()
  }

  pub fn reset_use_of_loan(&mut self) {
    self.use_of_loan = None;
  }

  pub fn set_education(&mut self, education: i32) {
    self.education = Some(education);
  }

  pub fn with_education(mut self, education: i32) -> PublicDatasetItem {
    self.education = Some(education);
    self
  }

  pub fn education(&self) -> Option<&i32> {
    self.education.as_ref()
  }

  pub fn reset_education(&mut self) {
    self.education = None;
  }

  pub fn set_marital_status(&mut self, marital_status: i32) {
    self.marital_status = Some(marital_status);
  }

  pub fn with_marital_status(mut self, marital_status: i32) -> PublicDatasetItem {
    self.marital_status = Some(marital_status);
    self
  }

  pub fn marital_status(&self) -> Option<&i32> {
    self.marital_status.as_ref()
  }

  pub fn reset_marital_status(&mut self) {
    self.marital_status = None;
  }

  pub fn set_nr_of_dependants(&mut self, nr_of_dependants: String) {
    self.nr_of_dependants = Some(nr_of_dependants);
  }

  pub fn with_nr_of_dependants(mut self, nr_of_dependants: String) -> PublicDatasetItem {
    self.nr_of_dependants = Some(nr_of_dependants);
    self
  }

  pub fn nr_of_dependants(&self) -> Option<&String> {
    self.nr_of_dependants.as_ref()
  }

  pub fn reset_nr_of_dependants(&mut self) {
    self.nr_of_dependants = None;
  }

  pub fn set_employment_status(&mut self, employment_status: i32) {
    self.employment_status = Some(employment_status);
  }

  pub fn with_employment_status(mut self, employment_status: i32) -> PublicDatasetItem {
    self.employment_status = Some(employment_status);
    self
  }

  pub fn employment_status(&self) -> Option<&i32> {
    self.employment_status.as_ref()
  }

  pub fn reset_employment_status(&mut self) {
    self.employment_status = None;
  }

  pub fn set_employment_duration_current_employer(&mut self, employment_duration_current_employer: String) {
    self.employment_duration_current_employer = Some(employment_duration_current_employer);
  }

  pub fn with_employment_duration_current_employer(mut self, employment_duration_current_employer: String) -> PublicDatasetItem {
    self.employment_duration_current_employer = Some(employment_duration_current_employer);
    self
  }

  pub fn employment_duration_current_employer(&self) -> Option<&String> {
    self.employment_duration_current_employer.as_ref()
  }

  pub fn reset_employment_duration_current_employer(&mut self) {
    self.employment_duration_current_employer = None;
  }

  pub fn set_employment_position(&mut self, employment_position: String) {
    self.employment_position = Some(employment_position);
  }

  pub fn with_employment_position(mut self, employment_position: String) -> PublicDatasetItem {
    self.employment_position = Some(employment_position);
    self
  }

  pub fn employment_position(&self) -> Option<&String> {
    self.employment_position.as_ref()
  }

  pub fn reset_employment_position(&mut self) {
    self.employment_position = None;
  }

  pub fn set_work_experience(&mut self, work_experience: String) {
    self.work_experience = Some(work_experience);
  }

  pub fn with_work_experience(mut self, work_experience: String) -> PublicDatasetItem {
    self.work_experience = Some(work_experience);
    self
  }

  pub fn work_experience(&self) -> Option<&String> {
    self.work_experience.as_ref()
  }

  pub fn reset_work_experience(&mut self) {
    self.work_experience = None;
  }

  pub fn set_occupation_area(&mut self, occupation_area: i32) {
    self.occupation_area = Some(occupation_area);
  }

  pub fn with_occupation_area(mut self, occupation_area: i32) -> PublicDatasetItem {
    self.occupation_area = Some(occupation_area);
    self
  }

  pub fn occupation_area(&self) -> Option<&i32> {
    self.occupation_area.as_ref()
  }

  pub fn reset_occupation_area(&mut self) {
    self.occupation_area = None;
  }

  pub fn set_home_ownership_type(&mut self, home_ownership_type: i32) {
    self.home_ownership_type = Some(home_ownership_type);
  }

  pub fn with_home_ownership_type(mut self, home_ownership_type: i32) -> PublicDatasetItem {
    self.home_ownership_type = Some(home_ownership_type);
    self
  }

  pub fn home_ownership_type(&self) -> Option<&i32> {
    self.home_ownership_type.as_ref()
  }

  pub fn reset_home_ownership_type(&mut self) {
    self.home_ownership_type = None;
  }

  pub fn set_income_from_principal_employer(&mut self, income_from_principal_employer: f64) {
    self.income_from_principal_employer = Some(income_from_principal_employer);
  }

  pub fn with_income_from_principal_employer(mut self, income_from_principal_employer: f64) -> PublicDatasetItem {
    self.income_from_principal_employer = Some(income_from_principal_employer);
    self
  }

  pub fn income_from_principal_employer(&self) -> Option<&f64> {
    self.income_from_principal_employer.as_ref()
  }

  pub fn reset_income_from_principal_employer(&mut self) {
    self.income_from_principal_employer = None;
  }

  pub fn set_income_from_pension(&mut self, income_from_pension: f64) {
    self.income_from_pension = Some(income_from_pension);
  }

  pub fn with_income_from_pension(mut self, income_from_pension: f64) -> PublicDatasetItem {
    self.income_from_pension = Some(income_from_pension);
    self
  }

  pub fn income_from_pension(&self) -> Option<&f64> {
    self.income_from_pension.as_ref()
  }

  pub fn reset_income_from_pension(&mut self) {
    self.income_from_pension = None;
  }

  pub fn set_income_from_family_allowance(&mut self, income_from_family_allowance: f64) {
    self.income_from_family_allowance = Some(income_from_family_allowance);
  }

  pub fn with_income_from_family_allowance(mut self, income_from_family_allowance: f64) -> PublicDatasetItem {
    self.income_from_family_allowance = Some(income_from_family_allowance);
    self
  }

  pub fn income_from_family_allowance(&self) -> Option<&f64> {
    self.income_from_family_allowance.as_ref()
  }

  pub fn reset_income_from_family_allowance(&mut self) {
    self.income_from_family_allowance = None;
  }

  pub fn set_income_from_social_welfare(&mut self, income_from_social_welfare: f64) {
    self.income_from_social_welfare = Some(income_from_social_welfare);
  }

  pub fn with_income_from_social_welfare(mut self, income_from_social_welfare: f64) -> PublicDatasetItem {
    self.income_from_social_welfare = Some(income_from_social_welfare);
    self
  }

  pub fn income_from_social_welfare(&self) -> Option<&f64> {
    self.income_from_social_welfare.as_ref()
  }

  pub fn reset_income_from_social_welfare(&mut self) {
    self.income_from_social_welfare = None;
  }

  pub fn set_income_from_leave_pay(&mut self, income_from_leave_pay: f64) {
    self.income_from_leave_pay = Some(income_from_leave_pay);
  }

  pub fn with_income_from_leave_pay(mut self, income_from_leave_pay: f64) -> PublicDatasetItem {
    self.income_from_leave_pay = Some(income_from_leave_pay);
    self
  }

  pub fn income_from_leave_pay(&self) -> Option<&f64> {
    self.income_from_leave_pay.as_ref()
  }

  pub fn reset_income_from_leave_pay(&mut self) {
    self.income_from_leave_pay = None;
  }

  pub fn set_income_from_child_support(&mut self, income_from_child_support: f64) {
    self.income_from_child_support = Some(income_from_child_support);
  }

  pub fn with_income_from_child_support(mut self, income_from_child_support: f64) -> PublicDatasetItem {
    self.income_from_child_support = Some(income_from_child_support);
    self
  }

  pub fn income_from_child_support(&self) -> Option<&f64> {
    self.income_from_child_support.as_ref()
  }

  pub fn reset_income_from_child_support(&mut self) {
    self.income_from_child_support = None;
  }

  pub fn set_income_other(&mut self, income_other: f64) {
    self.income_other = Some(income_other);
  }

  pub fn with_income_other(mut self, income_other: f64) -> PublicDatasetItem {
    self.income_other = Some(income_other);
    self
  }

  pub fn income_other(&self) -> Option<&f64> {
    self.income_other.as_ref()
  }

  pub fn reset_income_other(&mut self) {
    self.income_other = None;
  }

  pub fn set_income_total(&mut self, income_total: f64) {
    self.income_total = Some(income_total);
  }

  pub fn with_income_total(mut self, income_total: f64) -> PublicDatasetItem {
    self.income_total = Some(income_total);
    self
  }

  pub fn income_total(&self) -> Option<&f64> {
    self.income_total.as_ref()
  }

  pub fn reset_income_total(&mut self) {
    self.income_total = None;
  }

  pub fn set_existing_liabilities(&mut self, existing_liabilities: i32) {
    self.existing_liabilities = Some(existing_liabilities);
  }

  pub fn with_existing_liabilities(mut self, existing_liabilities: i32) -> PublicDatasetItem {
    self.existing_liabilities = Some(existing_liabilities);
    self
  }

  pub fn existing_liabilities(&self) -> Option<&i32> {
    self.existing_liabilities.as_ref()
  }

  pub fn reset_existing_liabilities(&mut self) {
    self.existing_liabilities = None;
  }

  pub fn set_refinance_liabilities(&mut self, refinance_liabilities: i32) {
    self.refinance_liabilities = Some(refinance_liabilities);
  }

  pub fn with_refinance_liabilities(mut self, refinance_liabilities: i32) -> PublicDatasetItem {
    self.refinance_liabilities = Some(refinance_liabilities);
    self
  }

  pub fn refinance_liabilities(&self) -> Option<&i32> {
    self.refinance_liabilities.as_ref()
  }

  pub fn reset_refinance_liabilities(&mut self) {
    self.refinance_liabilities = None;
  }

  pub fn set_liabilities_total(&mut self, liabilities_total: f64) {
    self.liabilities_total = Some(liabilities_total);
  }

  pub fn with_liabilities_total(mut self, liabilities_total: f64) -> PublicDatasetItem {
    self.liabilities_total = Some(liabilities_total);
    self
  }

  pub fn liabilities_total(&self) -> Option<&f64> {
    self.liabilities_total.as_ref()
  }

  pub fn reset_liabilities_total(&mut self) {
    self.liabilities_total = None;
  }

  pub fn set_debt_to_income(&mut self, debt_to_income: f64) {
    self.debt_to_income = Some(debt_to_income);
  }

  pub fn with_debt_to_income(mut self, debt_to_income: f64) -> PublicDatasetItem {
    self.debt_to_income = Some(debt_to_income);
    self
  }

  pub fn debt_to_income(&self) -> Option<&f64> {
    self.debt_to_income.as_ref()
  }

  pub fn reset_debt_to_income(&mut self) {
    self.debt_to_income = None;
  }

  pub fn set_free_cash(&mut self, free_cash: f64) {
    self.free_cash = Some(free_cash);
  }

  pub fn with_free_cash(mut self, free_cash: f64) -> PublicDatasetItem {
    self.free_cash = Some(free_cash);
    self
  }

  pub fn free_cash(&self) -> Option<&f64> {
    self.free_cash.as_ref()
  }

  pub fn reset_free_cash(&mut self) {
    self.free_cash = None;
  }

  pub fn set_monthly_payment_day(&mut self, monthly_payment_day: i32) {
    self.monthly_payment_day = Some(monthly_payment_day);
  }

  pub fn with_monthly_payment_day(mut self, monthly_payment_day: i32) -> PublicDatasetItem {
    self.monthly_payment_day = Some(monthly_payment_day);
    self
  }

  pub fn monthly_payment_day(&self) -> Option<&i32> {
    self.monthly_payment_day.as_ref()
  }

  pub fn reset_monthly_payment_day(&mut self) {
    self.monthly_payment_day = None;
  }

  pub fn set_active_schedule_first_payment_reached(&mut self, active_schedule_first_payment_reached: bool) {
    self.active_schedule_first_payment_reached = Some(active_schedule_first_payment_reached);
  }

  pub fn with_active_schedule_first_payment_reached(mut self, active_schedule_first_payment_reached: bool) -> PublicDatasetItem {
    self.active_schedule_first_payment_reached = Some(active_schedule_first_payment_reached);
    self
  }

  pub fn active_schedule_first_payment_reached(&self) -> Option<&bool> {
    self.active_schedule_first_payment_reached.as_ref()
  }

  pub fn reset_active_schedule_first_payment_reached(&mut self) {
    self.active_schedule_first_payment_reached = None;
  }

  pub fn set_planned_principal_till_date(&mut self, planned_principal_till_date: f64) {
    self.planned_principal_till_date = Some(planned_principal_till_date);
  }

  pub fn with_planned_principal_till_date(mut self, planned_principal_till_date: f64) -> PublicDatasetItem {
    self.planned_principal_till_date = Some(planned_principal_till_date);
    self
  }

  pub fn planned_principal_till_date(&self) -> Option<&f64> {
    self.planned_principal_till_date.as_ref()
  }

  pub fn reset_planned_principal_till_date(&mut self) {
    self.planned_principal_till_date = None;
  }

  pub fn set_planned_interest_till_date(&mut self, planned_interest_till_date: f64) {
    self.planned_interest_till_date = Some(planned_interest_till_date);
  }

  pub fn with_planned_interest_till_date(mut self, planned_interest_till_date: f64) -> PublicDatasetItem {
    self.planned_interest_till_date = Some(planned_interest_till_date);
    self
  }

  pub fn planned_interest_till_date(&self) -> Option<&f64> {
    self.planned_interest_till_date.as_ref()
  }

  pub fn reset_planned_interest_till_date(&mut self) {
    self.planned_interest_till_date = None;
  }

  pub fn set_last_payment_on(&mut self, last_payment_on: String) {
    self.last_payment_on = Some(last_payment_on);
  }

  pub fn with_last_payment_on(mut self, last_payment_on: String) -> PublicDatasetItem {
    self.last_payment_on = Some(last_payment_on);
    self
  }

  pub fn last_payment_on(&self) -> Option<&String> {
    self.last_payment_on.as_ref()
  }

  pub fn reset_last_payment_on(&mut self) {
    self.last_payment_on = None;
  }

  pub fn set_current_debt_days_primary(&mut self, current_debt_days_primary: i32) {
    self.current_debt_days_primary = Some(current_debt_days_primary);
  }

  pub fn with_current_debt_days_primary(mut self, current_debt_days_primary: i32) -> PublicDatasetItem {
    self.current_debt_days_primary = Some(current_debt_days_primary);
    self
  }

  pub fn current_debt_days_primary(&self) -> Option<&i32> {
    self.current_debt_days_primary.as_ref()
  }

  pub fn reset_current_debt_days_primary(&mut self) {
    self.current_debt_days_primary = None;
  }

  pub fn set_debt_occured_on(&mut self, debt_occured_on: String) {
    self.debt_occured_on = Some(debt_occured_on);
  }

  pub fn with_debt_occured_on(mut self, debt_occured_on: String) -> PublicDatasetItem {
    self.debt_occured_on = Some(debt_occured_on);
    self
  }

  pub fn debt_occured_on(&self) -> Option<&String> {
    self.debt_occured_on.as_ref()
  }

  pub fn reset_debt_occured_on(&mut self) {
    self.debt_occured_on = None;
  }

  pub fn set_current_debt_days_secondary(&mut self, current_debt_days_secondary: i32) {
    self.current_debt_days_secondary = Some(current_debt_days_secondary);
  }

  pub fn with_current_debt_days_secondary(mut self, current_debt_days_secondary: i32) -> PublicDatasetItem {
    self.current_debt_days_secondary = Some(current_debt_days_secondary);
    self
  }

  pub fn current_debt_days_secondary(&self) -> Option<&i32> {
    self.current_debt_days_secondary.as_ref()
  }

  pub fn reset_current_debt_days_secondary(&mut self) {
    self.current_debt_days_secondary = None;
  }

  pub fn set_debt_occured_on_for_secondary(&mut self, debt_occured_on_for_secondary: String) {
    self.debt_occured_on_for_secondary = Some(debt_occured_on_for_secondary);
  }

  pub fn with_debt_occured_on_for_secondary(mut self, debt_occured_on_for_secondary: String) -> PublicDatasetItem {
    self.debt_occured_on_for_secondary = Some(debt_occured_on_for_secondary);
    self
  }

  pub fn debt_occured_on_for_secondary(&self) -> Option<&String> {
    self.debt_occured_on_for_secondary.as_ref()
  }

  pub fn reset_debt_occured_on_for_secondary(&mut self) {
    self.debt_occured_on_for_secondary = None;
  }

  pub fn set_expected_loss(&mut self, expected_loss: f64) {
    self.expected_loss = Some(expected_loss);
  }

  pub fn with_expected_loss(mut self, expected_loss: f64) -> PublicDatasetItem {
    self.expected_loss = Some(expected_loss);
    self
  }

  pub fn expected_loss(&self) -> Option<&f64> {
    self.expected_loss.as_ref()
  }

  pub fn reset_expected_loss(&mut self) {
    self.expected_loss = None;
  }

  pub fn set_loss_given_default(&mut self, loss_given_default: f64) {
    self.loss_given_default = Some(loss_given_default);
  }

  pub fn with_loss_given_default(mut self, loss_given_default: f64) -> PublicDatasetItem {
    self.loss_given_default = Some(loss_given_default);
    self
  }

  pub fn loss_given_default(&self) -> Option<&f64> {
    self.loss_given_default.as_ref()
  }

  pub fn reset_loss_given_default(&mut self) {
    self.loss_given_default = None;
  }

  pub fn set_expected_return(&mut self, expected_return: f64) {
    self.expected_return = Some(expected_return);
  }

  pub fn with_expected_return(mut self, expected_return: f64) -> PublicDatasetItem {
    self.expected_return = Some(expected_return);
    self
  }

  pub fn expected_return(&self) -> Option<&f64> {
    self.expected_return.as_ref()
  }

  pub fn reset_expected_return(&mut self) {
    self.expected_return = None;
  }

  pub fn set_probability_of_default(&mut self, probability_of_default: f64) {
    self.probability_of_default = Some(probability_of_default);
  }

  pub fn with_probability_of_default(mut self, probability_of_default: f64) -> PublicDatasetItem {
    self.probability_of_default = Some(probability_of_default);
    self
  }

  pub fn probability_of_default(&self) -> Option<&f64> {
    self.probability_of_default.as_ref()
  }

  pub fn reset_probability_of_default(&mut self) {
    self.probability_of_default = None;
  }

  pub fn set_default_date(&mut self, default_date: String) {
    self.default_date = Some(default_date);
  }

  pub fn with_default_date(mut self, default_date: String) -> PublicDatasetItem {
    self.default_date = Some(default_date);
    self
  }

  pub fn default_date(&self) -> Option<&String> {
    self.default_date.as_ref()
  }

  pub fn reset_default_date(&mut self) {
    self.default_date = None;
  }

  pub fn set_principal_overdue_by_schedule(&mut self, principal_overdue_by_schedule: f64) {
    self.principal_overdue_by_schedule = Some(principal_overdue_by_schedule);
  }

  pub fn with_principal_overdue_by_schedule(mut self, principal_overdue_by_schedule: f64) -> PublicDatasetItem {
    self.principal_overdue_by_schedule = Some(principal_overdue_by_schedule);
    self
  }

  pub fn principal_overdue_by_schedule(&self) -> Option<&f64> {
    self.principal_overdue_by_schedule.as_ref()
  }

  pub fn reset_principal_overdue_by_schedule(&mut self) {
    self.principal_overdue_by_schedule = None;
  }

  pub fn set_planned_principal_post_default(&mut self, planned_principal_post_default: f64) {
    self.planned_principal_post_default = Some(planned_principal_post_default);
  }

  pub fn with_planned_principal_post_default(mut self, planned_principal_post_default: f64) -> PublicDatasetItem {
    self.planned_principal_post_default = Some(planned_principal_post_default);
    self
  }

  pub fn planned_principal_post_default(&self) -> Option<&f64> {
    self.planned_principal_post_default.as_ref()
  }

  pub fn reset_planned_principal_post_default(&mut self) {
    self.planned_principal_post_default = None;
  }

  pub fn set_planned_interest_post_default(&mut self, planned_interest_post_default: f64) {
    self.planned_interest_post_default = Some(planned_interest_post_default);
  }

  pub fn with_planned_interest_post_default(mut self, planned_interest_post_default: f64) -> PublicDatasetItem {
    self.planned_interest_post_default = Some(planned_interest_post_default);
    self
  }

  pub fn planned_interest_post_default(&self) -> Option<&f64> {
    self.planned_interest_post_default.as_ref()
  }

  pub fn reset_planned_interest_post_default(&mut self) {
    self.planned_interest_post_default = None;
  }

  pub fn set_ead1(&mut self, ead1: f64) {
    self.ead1 = Some(ead1);
  }

  pub fn with_ead1(mut self, ead1: f64) -> PublicDatasetItem {
    self.ead1 = Some(ead1);
    self
  }

  pub fn ead1(&self) -> Option<&f64> {
    self.ead1.as_ref()
  }

  pub fn reset_ead1(&mut self) {
    self.ead1 = None;
  }

  pub fn set_ead2(&mut self, ead2: f64) {
    self.ead2 = Some(ead2);
  }

  pub fn with_ead2(mut self, ead2: f64) -> PublicDatasetItem {
    self.ead2 = Some(ead2);
    self
  }

  pub fn ead2(&self) -> Option<&f64> {
    self.ead2.as_ref()
  }

  pub fn reset_ead2(&mut self) {
    self.ead2 = None;
  }

  pub fn set_principal_recovery(&mut self, principal_recovery: f64) {
    self.principal_recovery = Some(principal_recovery);
  }

  pub fn with_principal_recovery(mut self, principal_recovery: f64) -> PublicDatasetItem {
    self.principal_recovery = Some(principal_recovery);
    self
  }

  pub fn principal_recovery(&self) -> Option<&f64> {
    self.principal_recovery.as_ref()
  }

  pub fn reset_principal_recovery(&mut self) {
    self.principal_recovery = None;
  }

  pub fn set_interest_recovery(&mut self, interest_recovery: f64) {
    self.interest_recovery = Some(interest_recovery);
  }

  pub fn with_interest_recovery(mut self, interest_recovery: f64) -> PublicDatasetItem {
    self.interest_recovery = Some(interest_recovery);
    self
  }

  pub fn interest_recovery(&self) -> Option<&f64> {
    self.interest_recovery.as_ref()
  }

  pub fn reset_interest_recovery(&mut self) {
    self.interest_recovery = None;
  }

  pub fn set_recovery_stage(&mut self, recovery_stage: i32) {
    self.recovery_stage = Some(recovery_stage);
  }

  pub fn with_recovery_stage(mut self, recovery_stage: i32) -> PublicDatasetItem {
    self.recovery_stage = Some(recovery_stage);
    self
  }

  pub fn recovery_stage(&self) -> Option<&i32> {
    self.recovery_stage.as_ref()
  }

  pub fn reset_recovery_stage(&mut self) {
    self.recovery_stage = None;
  }

  pub fn set_stage_active_since(&mut self, stage_active_since: String) {
    self.stage_active_since = Some(stage_active_since);
  }

  pub fn with_stage_active_since(mut self, stage_active_since: String) -> PublicDatasetItem {
    self.stage_active_since = Some(stage_active_since);
    self
  }

  pub fn stage_active_since(&self) -> Option<&String> {
    self.stage_active_since.as_ref()
  }

  pub fn reset_stage_active_since(&mut self) {
    self.stage_active_since = None;
  }

  pub fn set_model_version(&mut self, model_version: i32) {
    self.model_version = Some(model_version);
  }

  pub fn with_model_version(mut self, model_version: i32) -> PublicDatasetItem {
    self.model_version = Some(model_version);
    self
  }

  pub fn model_version(&self) -> Option<&i32> {
    self.model_version.as_ref()
  }

  pub fn reset_model_version(&mut self) {
    self.model_version = None;
  }

  pub fn set_rating(&mut self, rating: String) {
    self.rating = Some(rating);
  }

  pub fn with_rating(mut self, rating: String) -> PublicDatasetItem {
    self.rating = Some(rating);
    self
  }

  pub fn rating(&self) -> Option<&String> {
    self.rating.as_ref()
  }

  pub fn reset_rating(&mut self) {
    self.rating = None;
  }

  pub fn set_el_v0(&mut self, el_v0: f64) {
    self.el_v0 = Some(el_v0);
  }

  pub fn with_el_v0(mut self, el_v0: f64) -> PublicDatasetItem {
    self.el_v0 = Some(el_v0);
    self
  }

  pub fn el_v0(&self) -> Option<&f64> {
    self.el_v0.as_ref()
  }

  pub fn reset_el_v0(&mut self) {
    self.el_v0 = None;
  }

  pub fn set_rating_v0(&mut self, rating_v0: String) {
    self.rating_v0 = Some(rating_v0);
  }

  pub fn with_rating_v0(mut self, rating_v0: String) -> PublicDatasetItem {
    self.rating_v0 = Some(rating_v0);
    self
  }

  pub fn rating_v0(&self) -> Option<&String> {
    self.rating_v0.as_ref()
  }

  pub fn reset_rating_v0(&mut self) {
    self.rating_v0 = None;
  }

  pub fn set_el_v1(&mut self, el_v1: f64) {
    self.el_v1 = Some(el_v1);
  }

  pub fn with_el_v1(mut self, el_v1: f64) -> PublicDatasetItem {
    self.el_v1 = Some(el_v1);
    self
  }

  pub fn el_v1(&self) -> Option<&f64> {
    self.el_v1.as_ref()
  }

  pub fn reset_el_v1(&mut self) {
    self.el_v1 = None;
  }

  pub fn set_rating_v1(&mut self, rating_v1: String) {
    self.rating_v1 = Some(rating_v1);
  }

  pub fn with_rating_v1(mut self, rating_v1: String) -> PublicDatasetItem {
    self.rating_v1 = Some(rating_v1);
    self
  }

  pub fn rating_v1(&self) -> Option<&String> {
    self.rating_v1.as_ref()
  }

  pub fn reset_rating_v1(&mut self) {
    self.rating_v1 = None;
  }

  pub fn set_el_v2(&mut self, el_v2: f64) {
    self.el_v2 = Some(el_v2);
  }

  pub fn with_el_v2(mut self, el_v2: f64) -> PublicDatasetItem {
    self.el_v2 = Some(el_v2);
    self
  }

  pub fn el_v2(&self) -> Option<&f64> {
    self.el_v2.as_ref()
  }

  pub fn reset_el_v2(&mut self) {
    self.el_v2 = None;
  }

  pub fn set_rating_v2(&mut self, rating_v2: String) {
    self.rating_v2 = Some(rating_v2);
  }

  pub fn with_rating_v2(mut self, rating_v2: String) -> PublicDatasetItem {
    self.rating_v2 = Some(rating_v2);
    self
  }

  pub fn rating_v2(&self) -> Option<&String> {
    self.rating_v2.as_ref()
  }

  pub fn reset_rating_v2(&mut self) {
    self.rating_v2 = None;
  }

  pub fn set_loan_cancelled(&mut self, loan_cancelled: bool) {
    self.loan_cancelled = Some(loan_cancelled);
  }

  pub fn with_loan_cancelled(mut self, loan_cancelled: bool) -> PublicDatasetItem {
    self.loan_cancelled = Some(loan_cancelled);
    self
  }

  pub fn loan_cancelled(&self) -> Option<&bool> {
    self.loan_cancelled.as_ref()
  }

  pub fn reset_loan_cancelled(&mut self) {
    self.loan_cancelled = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: String) -> PublicDatasetItem {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&String> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

  pub fn set_restructured(&mut self, restructured: bool) {
    self.restructured = Some(restructured);
  }

  pub fn with_restructured(mut self, restructured: bool) -> PublicDatasetItem {
    self.restructured = Some(restructured);
    self
  }

  pub fn restructured(&self) -> Option<&bool> {
    self.restructured.as_ref()
  }

  pub fn reset_restructured(&mut self) {
    self.restructured = None;
  }

  pub fn set_active_late_category(&mut self, active_late_category: String) {
    self.active_late_category = Some(active_late_category);
  }

  pub fn with_active_late_category(mut self, active_late_category: String) -> PublicDatasetItem {
    self.active_late_category = Some(active_late_category);
    self
  }

  pub fn active_late_category(&self) -> Option<&String> {
    self.active_late_category.as_ref()
  }

  pub fn reset_active_late_category(&mut self) {
    self.active_late_category = None;
  }

  pub fn set_worse_late_category(&mut self, worse_late_category: String) {
    self.worse_late_category = Some(worse_late_category);
  }

  pub fn with_worse_late_category(mut self, worse_late_category: String) -> PublicDatasetItem {
    self.worse_late_category = Some(worse_late_category);
    self
  }

  pub fn worse_late_category(&self) -> Option<&String> {
    self.worse_late_category.as_ref()
  }

  pub fn reset_worse_late_category(&mut self) {
    self.worse_late_category = None;
  }

  pub fn set_credit_score_es_micro_l(&mut self, credit_score_es_micro_l: String) {
    self.credit_score_es_micro_l = Some(credit_score_es_micro_l);
  }

  pub fn with_credit_score_es_micro_l(mut self, credit_score_es_micro_l: String) -> PublicDatasetItem {
    self.credit_score_es_micro_l = Some(credit_score_es_micro_l);
    self
  }

  pub fn credit_score_es_micro_l(&self) -> Option<&String> {
    self.credit_score_es_micro_l.as_ref()
  }

  pub fn reset_credit_score_es_micro_l(&mut self) {
    self.credit_score_es_micro_l = None;
  }

  pub fn set_credit_score_es_equifax_risk(&mut self, credit_score_es_equifax_risk: String) {
    self.credit_score_es_equifax_risk = Some(credit_score_es_equifax_risk);
  }

  pub fn with_credit_score_es_equifax_risk(mut self, credit_score_es_equifax_risk: String) -> PublicDatasetItem {
    self.credit_score_es_equifax_risk = Some(credit_score_es_equifax_risk);
    self
  }

  pub fn credit_score_es_equifax_risk(&self) -> Option<&String> {
    self.credit_score_es_equifax_risk.as_ref()
  }

  pub fn reset_credit_score_es_equifax_risk(&mut self) {
    self.credit_score_es_equifax_risk = None;
  }

  pub fn set_credit_score_fi_asiakas_tieto_risk_grade(&mut self, credit_score_fi_asiakas_tieto_risk_grade: String) {
    self.credit_score_fi_asiakas_tieto_risk_grade = Some(credit_score_fi_asiakas_tieto_risk_grade);
  }

  pub fn with_credit_score_fi_asiakas_tieto_risk_grade(mut self, credit_score_fi_asiakas_tieto_risk_grade: String) -> PublicDatasetItem {
    self.credit_score_fi_asiakas_tieto_risk_grade = Some(credit_score_fi_asiakas_tieto_risk_grade);
    self
  }

  pub fn credit_score_fi_asiakas_tieto_risk_grade(&self) -> Option<&String> {
    self.credit_score_fi_asiakas_tieto_risk_grade.as_ref()
  }

  pub fn reset_credit_score_fi_asiakas_tieto_risk_grade(&mut self) {
    self.credit_score_fi_asiakas_tieto_risk_grade = None;
  }

  pub fn set_credit_score_ee_mini(&mut self, credit_score_ee_mini: String) {
    self.credit_score_ee_mini = Some(credit_score_ee_mini);
  }

  pub fn with_credit_score_ee_mini(mut self, credit_score_ee_mini: String) -> PublicDatasetItem {
    self.credit_score_ee_mini = Some(credit_score_ee_mini);
    self
  }

  pub fn credit_score_ee_mini(&self) -> Option<&String> {
    self.credit_score_ee_mini.as_ref()
  }

  pub fn reset_credit_score_ee_mini(&mut self) {
    self.credit_score_ee_mini = None;
  }

  pub fn set_principal_payments_made(&mut self, principal_payments_made: f64) {
    self.principal_payments_made = Some(principal_payments_made);
  }

  pub fn with_principal_payments_made(mut self, principal_payments_made: f64) -> PublicDatasetItem {
    self.principal_payments_made = Some(principal_payments_made);
    self
  }

  pub fn principal_payments_made(&self) -> Option<&f64> {
    self.principal_payments_made.as_ref()
  }

  pub fn reset_principal_payments_made(&mut self) {
    self.principal_payments_made = None;
  }

  pub fn set_interest_and_penalty_payments_made(&mut self, interest_and_penalty_payments_made: f64) {
    self.interest_and_penalty_payments_made = Some(interest_and_penalty_payments_made);
  }

  pub fn with_interest_and_penalty_payments_made(mut self, interest_and_penalty_payments_made: f64) -> PublicDatasetItem {
    self.interest_and_penalty_payments_made = Some(interest_and_penalty_payments_made);
    self
  }

  pub fn interest_and_penalty_payments_made(&self) -> Option<&f64> {
    self.interest_and_penalty_payments_made.as_ref()
  }

  pub fn reset_interest_and_penalty_payments_made(&mut self) {
    self.interest_and_penalty_payments_made = None;
  }

  pub fn set_principal_write_offs(&mut self, principal_write_offs: f64) {
    self.principal_write_offs = Some(principal_write_offs);
  }

  pub fn with_principal_write_offs(mut self, principal_write_offs: f64) -> PublicDatasetItem {
    self.principal_write_offs = Some(principal_write_offs);
    self
  }

  pub fn principal_write_offs(&self) -> Option<&f64> {
    self.principal_write_offs.as_ref()
  }

  pub fn reset_principal_write_offs(&mut self) {
    self.principal_write_offs = None;
  }

  pub fn set_interest_and_penalty_write_offs(&mut self, interest_and_penalty_write_offs: f64) {
    self.interest_and_penalty_write_offs = Some(interest_and_penalty_write_offs);
  }

  pub fn with_interest_and_penalty_write_offs(mut self, interest_and_penalty_write_offs: f64) -> PublicDatasetItem {
    self.interest_and_penalty_write_offs = Some(interest_and_penalty_write_offs);
    self
  }

  pub fn interest_and_penalty_write_offs(&self) -> Option<&f64> {
    self.interest_and_penalty_write_offs.as_ref()
  }

  pub fn reset_interest_and_penalty_write_offs(&mut self) {
    self.interest_and_penalty_write_offs = None;
  }

  pub fn set_principal_debt_servicing_cost(&mut self, principal_debt_servicing_cost: f64) {
    self.principal_debt_servicing_cost = Some(principal_debt_servicing_cost);
  }

  pub fn with_principal_debt_servicing_cost(mut self, principal_debt_servicing_cost: f64) -> PublicDatasetItem {
    self.principal_debt_servicing_cost = Some(principal_debt_servicing_cost);
    self
  }

  pub fn principal_debt_servicing_cost(&self) -> Option<&f64> {
    self.principal_debt_servicing_cost.as_ref()
  }

  pub fn reset_principal_debt_servicing_cost(&mut self) {
    self.principal_debt_servicing_cost = None;
  }

  pub fn set_interest_and_penalty_debt_servicing_cost(&mut self, interest_and_penalty_debt_servicing_cost: f64) {
    self.interest_and_penalty_debt_servicing_cost = Some(interest_and_penalty_debt_servicing_cost);
  }

  pub fn with_interest_and_penalty_debt_servicing_cost(mut self, interest_and_penalty_debt_servicing_cost: f64) -> PublicDatasetItem {
    self.interest_and_penalty_debt_servicing_cost = Some(interest_and_penalty_debt_servicing_cost);
    self
  }

  pub fn interest_and_penalty_debt_servicing_cost(&self) -> Option<&f64> {
    self.interest_and_penalty_debt_servicing_cost.as_ref()
  }

  pub fn reset_interest_and_penalty_debt_servicing_cost(&mut self) {
    self.interest_and_penalty_debt_servicing_cost = None;
  }

  pub fn set_principal_balance(&mut self, principal_balance: f64) {
    self.principal_balance = Some(principal_balance);
  }

  pub fn with_principal_balance(mut self, principal_balance: f64) -> PublicDatasetItem {
    self.principal_balance = Some(principal_balance);
    self
  }

  pub fn principal_balance(&self) -> Option<&f64> {
    self.principal_balance.as_ref()
  }

  pub fn reset_principal_balance(&mut self) {
    self.principal_balance = None;
  }

  pub fn set_interest_and_penalty_balance(&mut self, interest_and_penalty_balance: f64) {
    self.interest_and_penalty_balance = Some(interest_and_penalty_balance);
  }

  pub fn with_interest_and_penalty_balance(mut self, interest_and_penalty_balance: f64) -> PublicDatasetItem {
    self.interest_and_penalty_balance = Some(interest_and_penalty_balance);
    self
  }

  pub fn interest_and_penalty_balance(&self) -> Option<&f64> {
    self.interest_and_penalty_balance.as_ref()
  }

  pub fn reset_interest_and_penalty_balance(&mut self) {
    self.interest_and_penalty_balance = None;
  }

  pub fn set_no_of_previous_loans_before_loan(&mut self, no_of_previous_loans_before_loan: i32) {
    self.no_of_previous_loans_before_loan = Some(no_of_previous_loans_before_loan);
  }

  pub fn with_no_of_previous_loans_before_loan(mut self, no_of_previous_loans_before_loan: i32) -> PublicDatasetItem {
    self.no_of_previous_loans_before_loan = Some(no_of_previous_loans_before_loan);
    self
  }

  pub fn no_of_previous_loans_before_loan(&self) -> Option<&i32> {
    self.no_of_previous_loans_before_loan.as_ref()
  }

  pub fn reset_no_of_previous_loans_before_loan(&mut self) {
    self.no_of_previous_loans_before_loan = None;
  }

  pub fn set_amount_of_previous_loans_before_loan(&mut self, amount_of_previous_loans_before_loan: f64) {
    self.amount_of_previous_loans_before_loan = Some(amount_of_previous_loans_before_loan);
  }

  pub fn with_amount_of_previous_loans_before_loan(mut self, amount_of_previous_loans_before_loan: f64) -> PublicDatasetItem {
    self.amount_of_previous_loans_before_loan = Some(amount_of_previous_loans_before_loan);
    self
  }

  pub fn amount_of_previous_loans_before_loan(&self) -> Option<&f64> {
    self.amount_of_previous_loans_before_loan.as_ref()
  }

  pub fn reset_amount_of_previous_loans_before_loan(&mut self) {
    self.amount_of_previous_loans_before_loan = None;
  }

  pub fn set_previous_repayments_before_loan(&mut self, previous_repayments_before_loan: f64) {
    self.previous_repayments_before_loan = Some(previous_repayments_before_loan);
  }

  pub fn with_previous_repayments_before_loan(mut self, previous_repayments_before_loan: f64) -> PublicDatasetItem {
    self.previous_repayments_before_loan = Some(previous_repayments_before_loan);
    self
  }

  pub fn previous_repayments_before_loan(&self) -> Option<&f64> {
    self.previous_repayments_before_loan.as_ref()
  }

  pub fn reset_previous_repayments_before_loan(&mut self) {
    self.previous_repayments_before_loan = None;
  }

  pub fn set_previous_early_repayments_before_loan(&mut self, previous_early_repayments_before_loan: f64) {
    self.previous_early_repayments_before_loan = Some(previous_early_repayments_before_loan);
  }

  pub fn with_previous_early_repayments_before_loan(mut self, previous_early_repayments_before_loan: f64) -> PublicDatasetItem {
    self.previous_early_repayments_before_loan = Some(previous_early_repayments_before_loan);
    self
  }

  pub fn previous_early_repayments_before_loan(&self) -> Option<&f64> {
    self.previous_early_repayments_before_loan.as_ref()
  }

  pub fn reset_previous_early_repayments_before_loan(&mut self) {
    self.previous_early_repayments_before_loan = None;
  }

  pub fn set_previous_early_repayments_count_before_loan(&mut self, previous_early_repayments_count_before_loan: i32) {
    self.previous_early_repayments_count_before_loan = Some(previous_early_repayments_count_before_loan);
  }

  pub fn with_previous_early_repayments_count_before_loan(mut self, previous_early_repayments_count_before_loan: i32) -> PublicDatasetItem {
    self.previous_early_repayments_count_before_loan = Some(previous_early_repayments_count_before_loan);
    self
  }

  pub fn previous_early_repayments_count_before_loan(&self) -> Option<&i32> {
    self.previous_early_repayments_count_before_loan.as_ref()
  }

  pub fn reset_previous_early_repayments_count_before_loan(&mut self) {
    self.previous_early_repayments_count_before_loan = None;
  }

  pub fn set_grace_period_start(&mut self, grace_period_start: String) {
    self.grace_period_start = Some(grace_period_start);
  }

  pub fn with_grace_period_start(mut self, grace_period_start: String) -> PublicDatasetItem {
    self.grace_period_start = Some(grace_period_start);
    self
  }

  pub fn grace_period_start(&self) -> Option<&String> {
    self.grace_period_start.as_ref()
  }

  pub fn reset_grace_period_start(&mut self) {
    self.grace_period_start = None;
  }

  pub fn set_grace_period_end(&mut self, grace_period_end: String) {
    self.grace_period_end = Some(grace_period_end);
  }

  pub fn with_grace_period_end(mut self, grace_period_end: String) -> PublicDatasetItem {
    self.grace_period_end = Some(grace_period_end);
    self
  }

  pub fn grace_period_end(&self) -> Option<&String> {
    self.grace_period_end.as_ref()
  }

  pub fn reset_grace_period_end(&mut self) {
    self.grace_period_end = None;
  }

  pub fn set_next_payment_date(&mut self, next_payment_date: String) {
    self.next_payment_date = Some(next_payment_date);
  }

  pub fn with_next_payment_date(mut self, next_payment_date: String) -> PublicDatasetItem {
    self.next_payment_date = Some(next_payment_date);
    self
  }

  pub fn next_payment_date(&self) -> Option<&String> {
    self.next_payment_date.as_ref()
  }

  pub fn reset_next_payment_date(&mut self) {
    self.next_payment_date = None;
  }

  pub fn set_next_payment_nr(&mut self, next_payment_nr: i32) {
    self.next_payment_nr = Some(next_payment_nr);
  }

  pub fn with_next_payment_nr(mut self, next_payment_nr: i32) -> PublicDatasetItem {
    self.next_payment_nr = Some(next_payment_nr);
    self
  }

  pub fn next_payment_nr(&self) -> Option<&i32> {
    self.next_payment_nr.as_ref()
  }

  pub fn reset_next_payment_nr(&mut self) {
    self.next_payment_nr = None;
  }

  pub fn set_nr_of_scheduled_payments(&mut self, nr_of_scheduled_payments: i32) {
    self.nr_of_scheduled_payments = Some(nr_of_scheduled_payments);
  }

  pub fn with_nr_of_scheduled_payments(mut self, nr_of_scheduled_payments: i32) -> PublicDatasetItem {
    self.nr_of_scheduled_payments = Some(nr_of_scheduled_payments);
    self
  }

  pub fn nr_of_scheduled_payments(&self) -> Option<&i32> {
    self.nr_of_scheduled_payments.as_ref()
  }

  pub fn reset_nr_of_scheduled_payments(&mut self) {
    self.nr_of_scheduled_payments = None;
  }

  pub fn set_re_scheduled_on(&mut self, re_scheduled_on: String) {
    self.re_scheduled_on = Some(re_scheduled_on);
  }

  pub fn with_re_scheduled_on(mut self, re_scheduled_on: String) -> PublicDatasetItem {
    self.re_scheduled_on = Some(re_scheduled_on);
    self
  }

  pub fn re_scheduled_on(&self) -> Option<&String> {
    self.re_scheduled_on.as_ref()
  }

  pub fn reset_re_scheduled_on(&mut self) {
    self.re_scheduled_on = None;
  }

}



