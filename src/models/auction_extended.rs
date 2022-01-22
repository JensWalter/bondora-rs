
/// AuctionExtended : Auction related data with debts and liabilities

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionExtended {
  /// Borrower's liabilities
  #[serde(rename = "Liabilities")]
  liabilities: Option<Vec<crate::models::Liability>>,
  /// Borrower's debts
  #[serde(rename = "Debts")]
  debts: Option<Vec<crate::models::Debt>>,
  /// Borrower's history
  #[serde(rename = "BorrowerHistory")]
  borrower_history: Option<crate::models::BorrowerHistory>,
  /// Unique loan identificator
  #[serde(rename = "LoanId")]
  loan_id: Option<String>,
  /// Unique auction identificator
  #[serde(rename = "AuctionId")]
  auction_id: Option<String>,
  /// Number of the loan
  #[serde(rename = "LoanNumber")]
  loan_number: Option<i32>,
  /// Customer's Bondora username
  #[serde(rename = "UserName")]
  user_name: Option<String>,
  /// Did the customer have prior credit history in Bondora  <para>0 Customer had at least 3 months of credit history in Bondora</para><para>1 No prior credit history in Bondora</para>
  #[serde(rename = "NewCreditCustomer")]
  new_credit_customer: Option<i32>,
  /// Date when the loan application was started
  #[serde(rename = "LoanApplicationStartedDate")]
  loan_application_started_date: Option<String>,
  /// Date and time when the auction is closed, if it's not funded 100% before that.  Auction will be closed before that, if auction is funded 100%.
  #[serde(rename = "PlannedCloseDate")]
  planned_close_date: Option<String>,
  /// Hour of signing the loan application
  #[serde(rename = "ApplicationSignedHour")]
  application_signed_hour: Option<i32>,
  /// Weekday of signing the loan application
  #[serde(rename = "ApplicationSignedWeekday")]
  application_signed_weekday: Option<i32>,
  /// Verification type
  #[serde(rename = "VerificationType")]
  verification_type: Option<i32>,
  /// Two letter language code
  #[serde(rename = "LanguageCode")]
  language_code: Option<i32>,
  /// Age of the borrower (years)
  #[serde(rename = "Age")]
  age: Option<i32>,
  /// Borrower's date of birth
  #[serde(rename = "DateOfBirth")]
  date_of_birth: Option<String>,
  /// Gender
  #[serde(rename = "Gender")]
  gender: Option<i32>,
  /// Residency of the borrower
  #[serde(rename = "Country")]
  country: Option<String>,
  /// A score that is specifically designed for risk classifying subprime borrowers (defined by Equifax as borrowers that do not have access to bank loans).   A measure of the probability of default one month ahead.  <para>The score is given on a 10-grade scale, from the best score to the worst:</para><para>M1, M2, M3, M4, M5, M6, M7, M8, M9, M10</para>
  #[serde(rename = "CreditScoreEsMicroL")]
  credit_score_es_micro_l: Option<String>,
  /// Generic score for the loan applicants that do not have active past due operations in ASNEF.  A measure of the probability of default one year ahead.   The score is given on a 6-grade scale.  <para>AAA Very low</para><para>AA Low</para><para>A Average</para><para>B Average High</para><para>C High</para><para>D Very High</para>
  #[serde(rename = "CreditScoreEsEquifaxRisk")]
  credit_score_es_equifax_risk: Option<String>,
  /// Credit Scoring model for Finnish Asiakastieto  <para>RL1 Very low risk 01-20</para><para>RL2 Low risk 21-40</para><para>RL3 Average risk 41-60</para><para>RL4 Big risk 61-80</para><para>RL5 Huge risk 81-100</para>
  #[serde(rename = "CreditScoreFiAsiakasTietoRiskGrade")]
  credit_score_fi_asiakas_tieto_risk_grade: Option<String>,
  /// Credit scoring for Estonian loans  <para>1000 No previous payments problems</para><para>900 Payments problems finished 24-36 months ago</para><para>800 Payments problems finished 12-24 months ago</para><para>700 Payments problems finished 6-12 months ago</para><para>600 Payment problems finished &lt;6 months ago</para><para>500 Active payment problems</para>
  #[serde(rename = "CreditScoreEeMini")]
  credit_score_ee_mini: Option<String>,
  /// The amount borrower applied for originally
  #[serde(rename = "AppliedAmount")]
  applied_amount: Option<f64>,
  /// Maximum interest rate accepted in the loan application
  #[serde(rename = "Interest")]
  interest: Option<f64>,
  /// The loan term
  #[serde(rename = "LoanDuration")]
  loan_duration: Option<i32>,
  /// County of the borrower
  #[serde(rename = "County")]
  county: Option<String>,
  /// City of the borrower
  #[serde(rename = "City")]
  city: Option<String>,
  /// Education
  #[serde(rename = "Education")]
  education: Option<i32>,
  /// Employment time with the current employer
  #[serde(rename = "EmploymentDurationCurrentEmployer")]
  employment_duration_current_employer: Option<String>,
  /// Type of home ownership
  #[serde(rename = "HomeOwnershipType")]
  home_ownership_type: Option<i32>,
  /// Total income
  #[serde(rename = "IncomeTotal")]
  income_total: Option<f64>,
  /// Loan monthly payment amount.
  #[serde(rename = "MonthlyPayment")]
  monthly_payment: Option<f64>,
  /// The day of the month the loan payments are scheduled for.  The actual date is adjusted for weekends and bank holidays.  E.g. if 10th is a Sunday then the payment will be made on the 11th in that month.
  #[serde(rename = "MonthlyPaymentDay")]
  monthly_payment_day: Option<i32>,
  /// The version of the Rating model used for issuing the Bondora Rating
  #[serde(rename = "ModelVersion")]
  model_version: Option<i32>,
  /// Expected Loss calculated by the Rating model
  #[serde(rename = "ExpectedLoss")]
  expected_loss: Option<f64>,
  /// Bondora Rating issued by the Rating model
  #[serde(rename = "Rating")]
  rating: Option<String>,
  /// Gives the percentage of outstanding exposure at the time of default that an investor is likely to lose if a loan actually defaults.   This means the proportion of funds lost for the investor after all expected recovery and accounting for the time value of the money recovered.   In general, LGD parameter is intended to be estimated based on the historical recoveries. However, in new markets where limited experience does not allow us more precise loss given default estimates, a LGD of 90% is assumed.
  #[serde(rename = "LossGivenDefault")]
  loss_given_default: Option<f64>,
  /// Probability of Default, refers to a loan’s probability of default within one year horizon.
  #[serde(rename = "ProbabilityOfDefault")]
  probability_of_default: Option<f64>,
  /// Expected return alpha
  #[serde(rename = "ExpectedReturnAlpha")]
  expected_return_alpha: Option<f64>,
  /// Total liabilities
  #[serde(rename = "LiabilitiesTotal")]
  liabilities_total: Option<f64>,
  /// Date when auction was published
  #[serde(rename = "ListedOnUTC")]
  listed_on_utc: Option<String>,
  /// Date and time when the auction was actually closed. Is null, if auction is active.
  #[serde(rename = "ActualCloseDate")]
  actual_close_date: Option<String>,
  /// The amount that auction is fulfilled, taken amount only bids where investors have enough funds.  This is preliminary calculated amount and can change when trying to close auction (auction is closed, when auction is funded 100% or PlannedCloseDate is reached) and specific investor(s) do not have enough funds.
  #[serde(rename = "WinningBidsAmount")]
  winning_bids_amount: Option<f64>,
  /// The amount that is remaining to be funded (AppliedAmount - WinningBidsAmount).
  #[serde(rename = "RemainingAmount")]
  remaining_amount: Option<f64>,
  /// How many bids current user has bidden into the auction
  #[serde(rename = "UserBids")]
  user_bids: Option<i32>,
  /// How much current user has bidden into the auction
  #[serde(rename = "UserBidAmount")]
  user_bid_amount: Option<f64>,
  /// Precentage, how much the auction is fulfilled. Can be more than 100%, if overfunded.
  #[serde(rename = "Fullfilled")]
  fullfilled: Option<f64>,
  /// <para>    1000 No previous payments problems</para>  <para>    900 Payments problems finished 24-36 months ago</para>  <para>    800 Payments problems finished 12-24 months ago</para>  <para>    700 Payments problems finished 6-12 months ago</para>  <para>    600 Payment problems finished &lt;6 months ago</para>  <para>    500 Active payment problems</para>
  #[serde(rename = "CreditScore")]
  credit_score: Option<i32>,
  /// Date when the Rating was calculated for this loan
  #[serde(rename = "ScoringDate")]
  scoring_date: Option<String>,
  /// Use of loan  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
  #[serde(rename = "UseOfLoan")]
  use_of_loan: Option<i32>,
  /// Marital status  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
  #[serde(rename = "MaritalStatus")]
  marital_status: Option<i32>,
  /// Number of children or other dependants  <para>Only Value for new Auctions since 1st of june 2017 is NULL</para>
  #[serde(rename = "NrOfDependants")]
  nr_of_dependants: Option<String>,
  /// Employment status  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
  #[serde(rename = "EmploymentStatus")]
  employment_status: Option<i32>,
  /// Employment position  <para>Only Value for new Auctions since 1st of june 2017 is NULL</para>
  #[serde(rename = "EmploymentPosition")]
  employment_position: Option<String>,
  /// Work experience in total  <para>Only Value for new Auctions since 1st of june 2017 is NULL</para>
  #[serde(rename = "WorkExperience")]
  work_experience: Option<String>,
  /// Occupation area  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
  #[serde(rename = "OccupationArea")]
  occupation_area: Option<i32>,
  /// Salary  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "IncomeFromPrincipalEmployer")]
  income_from_principal_employer: Option<f64>,
  /// Pension  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "IncomeFromPension")]
  income_from_pension: Option<f64>,
  /// Family allowance  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "IncomeFromFamilyAllowance")]
  income_from_family_allowance: Option<f64>,
  /// Social welfare  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "IncomeFromSocialWelfare")]
  income_from_social_welfare: Option<f64>,
  /// Leave pay  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "IncomeFromLeavePay")]
  income_from_leave_pay: Option<f64>,
  /// Child support  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "IncomeFromChildSupport")]
  income_from_child_support: Option<f64>,
  /// Other income  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "IncomeOther")]
  income_other: Option<f64>,
  /// Discretionary Income  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "FreeCash")]
  free_cash: Option<f64>,
  /// Debt to income ratio  <para>Only Value for new Auctions since 1st of june 2017 is 0</para>
  #[serde(rename = "DebtToIncome")]
  debt_to_income: Option<f64>,
  /// Exposure at Default (expressed as a percentage of the original loan amount), indicates outstanding investor exposure at the time of default, including outstanding principal amount plus accrued but unpaid interests.
  #[serde(rename = "EADRate")]
  ead_rate: Option<f64>,
  /// Maturity Factor M of 1.3 is assumed for loans with duration exceeding one year.
  #[serde(rename = "MaturityFactor")]
  maturity_factor: Option<f64>,
  /// Interest rate alpha
  #[serde(rename = "InterestRateAlpha")]
  interest_rate_alpha: Option<f64>
}

impl AuctionExtended {
  /// Auction related data with debts and liabilities
  pub fn new() -> AuctionExtended {
    AuctionExtended {
      liabilities: None,
      debts: None,
      borrower_history: None,
      loan_id: None,
      auction_id: None,
      loan_number: None,
      user_name: None,
      new_credit_customer: None,
      loan_application_started_date: None,
      planned_close_date: None,
      application_signed_hour: None,
      application_signed_weekday: None,
      verification_type: None,
      language_code: None,
      age: None,
      date_of_birth: None,
      gender: None,
      country: None,
      credit_score_es_micro_l: None,
      credit_score_es_equifax_risk: None,
      credit_score_fi_asiakas_tieto_risk_grade: None,
      credit_score_ee_mini: None,
      applied_amount: None,
      interest: None,
      loan_duration: None,
      county: None,
      city: None,
      education: None,
      employment_duration_current_employer: None,
      home_ownership_type: None,
      income_total: None,
      monthly_payment: None,
      monthly_payment_day: None,
      model_version: None,
      expected_loss: None,
      rating: None,
      loss_given_default: None,
      probability_of_default: None,
      expected_return_alpha: None,
      liabilities_total: None,
      listed_on_utc: None,
      actual_close_date: None,
      winning_bids_amount: None,
      remaining_amount: None,
      user_bids: None,
      user_bid_amount: None,
      fullfilled: None,
      credit_score: None,
      scoring_date: None,
      use_of_loan: None,
      marital_status: None,
      nr_of_dependants: None,
      employment_status: None,
      employment_position: None,
      work_experience: None,
      occupation_area: None,
      income_from_principal_employer: None,
      income_from_pension: None,
      income_from_family_allowance: None,
      income_from_social_welfare: None,
      income_from_leave_pay: None,
      income_from_child_support: None,
      income_other: None,
      free_cash: None,
      debt_to_income: None,
      ead_rate: None,
      maturity_factor: None,
      interest_rate_alpha: None
    }
  }

  pub fn set_liabilities(&mut self, liabilities: Vec<crate::models::Liability>) {
    self.liabilities = Some(liabilities);
  }

  pub fn with_liabilities(mut self, liabilities: Vec<crate::models::Liability>) -> AuctionExtended {
    self.liabilities = Some(liabilities);
    self
  }

  pub fn liabilities(&self) -> Option<&Vec<crate::models::Liability>> {
    self.liabilities.as_ref()
  }

  pub fn reset_liabilities(&mut self) {
    self.liabilities = None;
  }

  pub fn set_debts(&mut self, debts: Vec<crate::models::Debt>) {
    self.debts = Some(debts);
  }

  pub fn with_debts(mut self, debts: Vec<crate::models::Debt>) -> AuctionExtended {
    self.debts = Some(debts);
    self
  }

  pub fn debts(&self) -> Option<&Vec<crate::models::Debt>> {
    self.debts.as_ref()
  }

  pub fn reset_debts(&mut self) {
    self.debts = None;
  }

  pub fn set_borrower_history(&mut self, borrower_history: crate::models::BorrowerHistory) {
    self.borrower_history = Some(borrower_history);
  }

  pub fn with_borrower_history(mut self, borrower_history: crate::models::BorrowerHistory) -> AuctionExtended {
    self.borrower_history = Some(borrower_history);
    self
  }

  pub fn borrower_history(&self) -> Option<&crate::models::BorrowerHistory> {
    self.borrower_history.as_ref()
  }

  pub fn reset_borrower_history(&mut self) {
    self.borrower_history = None;
  }

  pub fn set_loan_id(&mut self, loan_id: String) {
    self.loan_id = Some(loan_id);
  }

  pub fn with_loan_id(mut self, loan_id: String) -> AuctionExtended {
    self.loan_id = Some(loan_id);
    self
  }

  pub fn loan_id(&self) -> Option<&String> {
    self.loan_id.as_ref()
  }

  pub fn reset_loan_id(&mut self) {
    self.loan_id = None;
  }

  pub fn set_auction_id(&mut self, auction_id: String) {
    self.auction_id = Some(auction_id);
  }

  pub fn with_auction_id(mut self, auction_id: String) -> AuctionExtended {
    self.auction_id = Some(auction_id);
    self
  }

  pub fn auction_id(&self) -> Option<&String> {
    self.auction_id.as_ref()
  }

  pub fn reset_auction_id(&mut self) {
    self.auction_id = None;
  }

  pub fn set_loan_number(&mut self, loan_number: i32) {
    self.loan_number = Some(loan_number);
  }

  pub fn with_loan_number(mut self, loan_number: i32) -> AuctionExtended {
    self.loan_number = Some(loan_number);
    self
  }

  pub fn loan_number(&self) -> Option<&i32> {
    self.loan_number.as_ref()
  }

  pub fn reset_loan_number(&mut self) {
    self.loan_number = None;
  }

  pub fn set_user_name(&mut self, user_name: String) {
    self.user_name = Some(user_name);
  }

  pub fn with_user_name(mut self, user_name: String) -> AuctionExtended {
    self.user_name = Some(user_name);
    self
  }

  pub fn user_name(&self) -> Option<&String> {
    self.user_name.as_ref()
  }

  pub fn reset_user_name(&mut self) {
    self.user_name = None;
  }

  pub fn set_new_credit_customer(&mut self, new_credit_customer: i32) {
    self.new_credit_customer = Some(new_credit_customer);
  }

  pub fn with_new_credit_customer(mut self, new_credit_customer: i32) -> AuctionExtended {
    self.new_credit_customer = Some(new_credit_customer);
    self
  }

  pub fn new_credit_customer(&self) -> Option<&i32> {
    self.new_credit_customer.as_ref()
  }

  pub fn reset_new_credit_customer(&mut self) {
    self.new_credit_customer = None;
  }

  pub fn set_loan_application_started_date(&mut self, loan_application_started_date: String) {
    self.loan_application_started_date = Some(loan_application_started_date);
  }

  pub fn with_loan_application_started_date(mut self, loan_application_started_date: String) -> AuctionExtended {
    self.loan_application_started_date = Some(loan_application_started_date);
    self
  }

  pub fn loan_application_started_date(&self) -> Option<&String> {
    self.loan_application_started_date.as_ref()
  }

  pub fn reset_loan_application_started_date(&mut self) {
    self.loan_application_started_date = None;
  }

  pub fn set_planned_close_date(&mut self, planned_close_date: String) {
    self.planned_close_date = Some(planned_close_date);
  }

  pub fn with_planned_close_date(mut self, planned_close_date: String) -> AuctionExtended {
    self.planned_close_date = Some(planned_close_date);
    self
  }

  pub fn planned_close_date(&self) -> Option<&String> {
    self.planned_close_date.as_ref()
  }

  pub fn reset_planned_close_date(&mut self) {
    self.planned_close_date = None;
  }

  pub fn set_application_signed_hour(&mut self, application_signed_hour: i32) {
    self.application_signed_hour = Some(application_signed_hour);
  }

  pub fn with_application_signed_hour(mut self, application_signed_hour: i32) -> AuctionExtended {
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

  pub fn with_application_signed_weekday(mut self, application_signed_weekday: i32) -> AuctionExtended {
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

  pub fn with_verification_type(mut self, verification_type: i32) -> AuctionExtended {
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

  pub fn with_language_code(mut self, language_code: i32) -> AuctionExtended {
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

  pub fn with_age(mut self, age: i32) -> AuctionExtended {
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

  pub fn with_date_of_birth(mut self, date_of_birth: String) -> AuctionExtended {
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

  pub fn with_gender(mut self, gender: i32) -> AuctionExtended {
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

  pub fn with_country(mut self, country: String) -> AuctionExtended {
    self.country = Some(country);
    self
  }

  pub fn country(&self) -> Option<&String> {
    self.country.as_ref()
  }

  pub fn reset_country(&mut self) {
    self.country = None;
  }

  pub fn set_credit_score_es_micro_l(&mut self, credit_score_es_micro_l: String) {
    self.credit_score_es_micro_l = Some(credit_score_es_micro_l);
  }

  pub fn with_credit_score_es_micro_l(mut self, credit_score_es_micro_l: String) -> AuctionExtended {
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

  pub fn with_credit_score_es_equifax_risk(mut self, credit_score_es_equifax_risk: String) -> AuctionExtended {
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

  pub fn with_credit_score_fi_asiakas_tieto_risk_grade(mut self, credit_score_fi_asiakas_tieto_risk_grade: String) -> AuctionExtended {
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

  pub fn with_credit_score_ee_mini(mut self, credit_score_ee_mini: String) -> AuctionExtended {
    self.credit_score_ee_mini = Some(credit_score_ee_mini);
    self
  }

  pub fn credit_score_ee_mini(&self) -> Option<&String> {
    self.credit_score_ee_mini.as_ref()
  }

  pub fn reset_credit_score_ee_mini(&mut self) {
    self.credit_score_ee_mini = None;
  }

  pub fn set_applied_amount(&mut self, applied_amount: f64) {
    self.applied_amount = Some(applied_amount);
  }

  pub fn with_applied_amount(mut self, applied_amount: f64) -> AuctionExtended {
    self.applied_amount = Some(applied_amount);
    self
  }

  pub fn applied_amount(&self) -> Option<&f64> {
    self.applied_amount.as_ref()
  }

  pub fn reset_applied_amount(&mut self) {
    self.applied_amount = None;
  }

  pub fn set_interest(&mut self, interest: f64) {
    self.interest = Some(interest);
  }

  pub fn with_interest(mut self, interest: f64) -> AuctionExtended {
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

  pub fn with_loan_duration(mut self, loan_duration: i32) -> AuctionExtended {
    self.loan_duration = Some(loan_duration);
    self
  }

  pub fn loan_duration(&self) -> Option<&i32> {
    self.loan_duration.as_ref()
  }

  pub fn reset_loan_duration(&mut self) {
    self.loan_duration = None;
  }

  pub fn set_county(&mut self, county: String) {
    self.county = Some(county);
  }

  pub fn with_county(mut self, county: String) -> AuctionExtended {
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

  pub fn with_city(mut self, city: String) -> AuctionExtended {
    self.city = Some(city);
    self
  }

  pub fn city(&self) -> Option<&String> {
    self.city.as_ref()
  }

  pub fn reset_city(&mut self) {
    self.city = None;
  }

  pub fn set_education(&mut self, education: i32) {
    self.education = Some(education);
  }

  pub fn with_education(mut self, education: i32) -> AuctionExtended {
    self.education = Some(education);
    self
  }

  pub fn education(&self) -> Option<&i32> {
    self.education.as_ref()
  }

  pub fn reset_education(&mut self) {
    self.education = None;
  }

  pub fn set_employment_duration_current_employer(&mut self, employment_duration_current_employer: String) {
    self.employment_duration_current_employer = Some(employment_duration_current_employer);
  }

  pub fn with_employment_duration_current_employer(mut self, employment_duration_current_employer: String) -> AuctionExtended {
    self.employment_duration_current_employer = Some(employment_duration_current_employer);
    self
  }

  pub fn employment_duration_current_employer(&self) -> Option<&String> {
    self.employment_duration_current_employer.as_ref()
  }

  pub fn reset_employment_duration_current_employer(&mut self) {
    self.employment_duration_current_employer = None;
  }

  pub fn set_home_ownership_type(&mut self, home_ownership_type: i32) {
    self.home_ownership_type = Some(home_ownership_type);
  }

  pub fn with_home_ownership_type(mut self, home_ownership_type: i32) -> AuctionExtended {
    self.home_ownership_type = Some(home_ownership_type);
    self
  }

  pub fn home_ownership_type(&self) -> Option<&i32> {
    self.home_ownership_type.as_ref()
  }

  pub fn reset_home_ownership_type(&mut self) {
    self.home_ownership_type = None;
  }

  pub fn set_income_total(&mut self, income_total: f64) {
    self.income_total = Some(income_total);
  }

  pub fn with_income_total(mut self, income_total: f64) -> AuctionExtended {
    self.income_total = Some(income_total);
    self
  }

  pub fn income_total(&self) -> Option<&f64> {
    self.income_total.as_ref()
  }

  pub fn reset_income_total(&mut self) {
    self.income_total = None;
  }

  pub fn set_monthly_payment(&mut self, monthly_payment: f64) {
    self.monthly_payment = Some(monthly_payment);
  }

  pub fn with_monthly_payment(mut self, monthly_payment: f64) -> AuctionExtended {
    self.monthly_payment = Some(monthly_payment);
    self
  }

  pub fn monthly_payment(&self) -> Option<&f64> {
    self.monthly_payment.as_ref()
  }

  pub fn reset_monthly_payment(&mut self) {
    self.monthly_payment = None;
  }

  pub fn set_monthly_payment_day(&mut self, monthly_payment_day: i32) {
    self.monthly_payment_day = Some(monthly_payment_day);
  }

  pub fn with_monthly_payment_day(mut self, monthly_payment_day: i32) -> AuctionExtended {
    self.monthly_payment_day = Some(monthly_payment_day);
    self
  }

  pub fn monthly_payment_day(&self) -> Option<&i32> {
    self.monthly_payment_day.as_ref()
  }

  pub fn reset_monthly_payment_day(&mut self) {
    self.monthly_payment_day = None;
  }

  pub fn set_model_version(&mut self, model_version: i32) {
    self.model_version = Some(model_version);
  }

  pub fn with_model_version(mut self, model_version: i32) -> AuctionExtended {
    self.model_version = Some(model_version);
    self
  }

  pub fn model_version(&self) -> Option<&i32> {
    self.model_version.as_ref()
  }

  pub fn reset_model_version(&mut self) {
    self.model_version = None;
  }

  pub fn set_expected_loss(&mut self, expected_loss: f64) {
    self.expected_loss = Some(expected_loss);
  }

  pub fn with_expected_loss(mut self, expected_loss: f64) -> AuctionExtended {
    self.expected_loss = Some(expected_loss);
    self
  }

  pub fn expected_loss(&self) -> Option<&f64> {
    self.expected_loss.as_ref()
  }

  pub fn reset_expected_loss(&mut self) {
    self.expected_loss = None;
  }

  pub fn set_rating(&mut self, rating: String) {
    self.rating = Some(rating);
  }

  pub fn with_rating(mut self, rating: String) -> AuctionExtended {
    self.rating = Some(rating);
    self
  }

  pub fn rating(&self) -> Option<&String> {
    self.rating.as_ref()
  }

  pub fn reset_rating(&mut self) {
    self.rating = None;
  }

  pub fn set_loss_given_default(&mut self, loss_given_default: f64) {
    self.loss_given_default = Some(loss_given_default);
  }

  pub fn with_loss_given_default(mut self, loss_given_default: f64) -> AuctionExtended {
    self.loss_given_default = Some(loss_given_default);
    self
  }

  pub fn loss_given_default(&self) -> Option<&f64> {
    self.loss_given_default.as_ref()
  }

  pub fn reset_loss_given_default(&mut self) {
    self.loss_given_default = None;
  }

  pub fn set_probability_of_default(&mut self, probability_of_default: f64) {
    self.probability_of_default = Some(probability_of_default);
  }

  pub fn with_probability_of_default(mut self, probability_of_default: f64) -> AuctionExtended {
    self.probability_of_default = Some(probability_of_default);
    self
  }

  pub fn probability_of_default(&self) -> Option<&f64> {
    self.probability_of_default.as_ref()
  }

  pub fn reset_probability_of_default(&mut self) {
    self.probability_of_default = None;
  }

  pub fn set_expected_return_alpha(&mut self, expected_return_alpha: f64) {
    self.expected_return_alpha = Some(expected_return_alpha);
  }

  pub fn with_expected_return_alpha(mut self, expected_return_alpha: f64) -> AuctionExtended {
    self.expected_return_alpha = Some(expected_return_alpha);
    self
  }

  pub fn expected_return_alpha(&self) -> Option<&f64> {
    self.expected_return_alpha.as_ref()
  }

  pub fn reset_expected_return_alpha(&mut self) {
    self.expected_return_alpha = None;
  }

  pub fn set_liabilities_total(&mut self, liabilities_total: f64) {
    self.liabilities_total = Some(liabilities_total);
  }

  pub fn with_liabilities_total(mut self, liabilities_total: f64) -> AuctionExtended {
    self.liabilities_total = Some(liabilities_total);
    self
  }

  pub fn liabilities_total(&self) -> Option<&f64> {
    self.liabilities_total.as_ref()
  }

  pub fn reset_liabilities_total(&mut self) {
    self.liabilities_total = None;
  }

  pub fn set_listed_on_utc(&mut self, listed_on_utc: String) {
    self.listed_on_utc = Some(listed_on_utc);
  }

  pub fn with_listed_on_utc(mut self, listed_on_utc: String) -> AuctionExtended {
    self.listed_on_utc = Some(listed_on_utc);
    self
  }

  pub fn listed_on_utc(&self) -> Option<&String> {
    self.listed_on_utc.as_ref()
  }

  pub fn reset_listed_on_utc(&mut self) {
    self.listed_on_utc = None;
  }

  pub fn set_actual_close_date(&mut self, actual_close_date: String) {
    self.actual_close_date = Some(actual_close_date);
  }

  pub fn with_actual_close_date(mut self, actual_close_date: String) -> AuctionExtended {
    self.actual_close_date = Some(actual_close_date);
    self
  }

  pub fn actual_close_date(&self) -> Option<&String> {
    self.actual_close_date.as_ref()
  }

  pub fn reset_actual_close_date(&mut self) {
    self.actual_close_date = None;
  }

  pub fn set_winning_bids_amount(&mut self, winning_bids_amount: f64) {
    self.winning_bids_amount = Some(winning_bids_amount);
  }

  pub fn with_winning_bids_amount(mut self, winning_bids_amount: f64) -> AuctionExtended {
    self.winning_bids_amount = Some(winning_bids_amount);
    self
  }

  pub fn winning_bids_amount(&self) -> Option<&f64> {
    self.winning_bids_amount.as_ref()
  }

  pub fn reset_winning_bids_amount(&mut self) {
    self.winning_bids_amount = None;
  }

  pub fn set_remaining_amount(&mut self, remaining_amount: f64) {
    self.remaining_amount = Some(remaining_amount);
  }

  pub fn with_remaining_amount(mut self, remaining_amount: f64) -> AuctionExtended {
    self.remaining_amount = Some(remaining_amount);
    self
  }

  pub fn remaining_amount(&self) -> Option<&f64> {
    self.remaining_amount.as_ref()
  }

  pub fn reset_remaining_amount(&mut self) {
    self.remaining_amount = None;
  }

  pub fn set_user_bids(&mut self, user_bids: i32) {
    self.user_bids = Some(user_bids);
  }

  pub fn with_user_bids(mut self, user_bids: i32) -> AuctionExtended {
    self.user_bids = Some(user_bids);
    self
  }

  pub fn user_bids(&self) -> Option<&i32> {
    self.user_bids.as_ref()
  }

  pub fn reset_user_bids(&mut self) {
    self.user_bids = None;
  }

  pub fn set_user_bid_amount(&mut self, user_bid_amount: f64) {
    self.user_bid_amount = Some(user_bid_amount);
  }

  pub fn with_user_bid_amount(mut self, user_bid_amount: f64) -> AuctionExtended {
    self.user_bid_amount = Some(user_bid_amount);
    self
  }

  pub fn user_bid_amount(&self) -> Option<&f64> {
    self.user_bid_amount.as_ref()
  }

  pub fn reset_user_bid_amount(&mut self) {
    self.user_bid_amount = None;
  }

  pub fn set_fullfilled(&mut self, fullfilled: f64) {
    self.fullfilled = Some(fullfilled);
  }

  pub fn with_fullfilled(mut self, fullfilled: f64) -> AuctionExtended {
    self.fullfilled = Some(fullfilled);
    self
  }

  pub fn fullfilled(&self) -> Option<&f64> {
    self.fullfilled.as_ref()
  }

  pub fn reset_fullfilled(&mut self) {
    self.fullfilled = None;
  }

  pub fn set_credit_score(&mut self, credit_score: i32) {
    self.credit_score = Some(credit_score);
  }

  pub fn with_credit_score(mut self, credit_score: i32) -> AuctionExtended {
    self.credit_score = Some(credit_score);
    self
  }

  pub fn credit_score(&self) -> Option<&i32> {
    self.credit_score.as_ref()
  }

  pub fn reset_credit_score(&mut self) {
    self.credit_score = None;
  }

  pub fn set_scoring_date(&mut self, scoring_date: String) {
    self.scoring_date = Some(scoring_date);
  }

  pub fn with_scoring_date(mut self, scoring_date: String) -> AuctionExtended {
    self.scoring_date = Some(scoring_date);
    self
  }

  pub fn scoring_date(&self) -> Option<&String> {
    self.scoring_date.as_ref()
  }

  pub fn reset_scoring_date(&mut self) {
    self.scoring_date = None;
  }

  pub fn set_use_of_loan(&mut self, use_of_loan: i32) {
    self.use_of_loan = Some(use_of_loan);
  }

  pub fn with_use_of_loan(mut self, use_of_loan: i32) -> AuctionExtended {
    self.use_of_loan = Some(use_of_loan);
    self
  }

  pub fn use_of_loan(&self) -> Option<&i32> {
    self.use_of_loan.as_ref()
  }

  pub fn reset_use_of_loan(&mut self) {
    self.use_of_loan = None;
  }

  pub fn set_marital_status(&mut self, marital_status: i32) {
    self.marital_status = Some(marital_status);
  }

  pub fn with_marital_status(mut self, marital_status: i32) -> AuctionExtended {
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

  pub fn with_nr_of_dependants(mut self, nr_of_dependants: String) -> AuctionExtended {
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

  pub fn with_employment_status(mut self, employment_status: i32) -> AuctionExtended {
    self.employment_status = Some(employment_status);
    self
  }

  pub fn employment_status(&self) -> Option<&i32> {
    self.employment_status.as_ref()
  }

  pub fn reset_employment_status(&mut self) {
    self.employment_status = None;
  }

  pub fn set_employment_position(&mut self, employment_position: String) {
    self.employment_position = Some(employment_position);
  }

  pub fn with_employment_position(mut self, employment_position: String) -> AuctionExtended {
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

  pub fn with_work_experience(mut self, work_experience: String) -> AuctionExtended {
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

  pub fn with_occupation_area(mut self, occupation_area: i32) -> AuctionExtended {
    self.occupation_area = Some(occupation_area);
    self
  }

  pub fn occupation_area(&self) -> Option<&i32> {
    self.occupation_area.as_ref()
  }

  pub fn reset_occupation_area(&mut self) {
    self.occupation_area = None;
  }

  pub fn set_income_from_principal_employer(&mut self, income_from_principal_employer: f64) {
    self.income_from_principal_employer = Some(income_from_principal_employer);
  }

  pub fn with_income_from_principal_employer(mut self, income_from_principal_employer: f64) -> AuctionExtended {
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

  pub fn with_income_from_pension(mut self, income_from_pension: f64) -> AuctionExtended {
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

  pub fn with_income_from_family_allowance(mut self, income_from_family_allowance: f64) -> AuctionExtended {
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

  pub fn with_income_from_social_welfare(mut self, income_from_social_welfare: f64) -> AuctionExtended {
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

  pub fn with_income_from_leave_pay(mut self, income_from_leave_pay: f64) -> AuctionExtended {
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

  pub fn with_income_from_child_support(mut self, income_from_child_support: f64) -> AuctionExtended {
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

  pub fn with_income_other(mut self, income_other: f64) -> AuctionExtended {
    self.income_other = Some(income_other);
    self
  }

  pub fn income_other(&self) -> Option<&f64> {
    self.income_other.as_ref()
  }

  pub fn reset_income_other(&mut self) {
    self.income_other = None;
  }

  pub fn set_free_cash(&mut self, free_cash: f64) {
    self.free_cash = Some(free_cash);
  }

  pub fn with_free_cash(mut self, free_cash: f64) -> AuctionExtended {
    self.free_cash = Some(free_cash);
    self
  }

  pub fn free_cash(&self) -> Option<&f64> {
    self.free_cash.as_ref()
  }

  pub fn reset_free_cash(&mut self) {
    self.free_cash = None;
  }

  pub fn set_debt_to_income(&mut self, debt_to_income: f64) {
    self.debt_to_income = Some(debt_to_income);
  }

  pub fn with_debt_to_income(mut self, debt_to_income: f64) -> AuctionExtended {
    self.debt_to_income = Some(debt_to_income);
    self
  }

  pub fn debt_to_income(&self) -> Option<&f64> {
    self.debt_to_income.as_ref()
  }

  pub fn reset_debt_to_income(&mut self) {
    self.debt_to_income = None;
  }

  pub fn set_ead_rate(&mut self, ead_rate: f64) {
    self.ead_rate = Some(ead_rate);
  }

  pub fn with_ead_rate(mut self, ead_rate: f64) -> AuctionExtended {
    self.ead_rate = Some(ead_rate);
    self
  }

  pub fn ead_rate(&self) -> Option<&f64> {
    self.ead_rate.as_ref()
  }

  pub fn reset_ead_rate(&mut self) {
    self.ead_rate = None;
  }

  pub fn set_maturity_factor(&mut self, maturity_factor: f64) {
    self.maturity_factor = Some(maturity_factor);
  }

  pub fn with_maturity_factor(mut self, maturity_factor: f64) -> AuctionExtended {
    self.maturity_factor = Some(maturity_factor);
    self
  }

  pub fn maturity_factor(&self) -> Option<&f64> {
    self.maturity_factor.as_ref()
  }

  pub fn reset_maturity_factor(&mut self) {
    self.maturity_factor = None;
  }

  pub fn set_interest_rate_alpha(&mut self, interest_rate_alpha: f64) {
    self.interest_rate_alpha = Some(interest_rate_alpha);
  }

  pub fn with_interest_rate_alpha(mut self, interest_rate_alpha: f64) -> AuctionExtended {
    self.interest_rate_alpha = Some(interest_rate_alpha);
    self
  }

  pub fn interest_rate_alpha(&self) -> Option<&f64> {
    self.interest_rate_alpha.as_ref()
  }

  pub fn reset_interest_rate_alpha(&mut self) {
    self.interest_rate_alpha = None;
  }

}



