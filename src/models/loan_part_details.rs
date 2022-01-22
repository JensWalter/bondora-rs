
/// LoanPartDetails : LoanPart related data with summary, collection process and schedules

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanPartDetails {
  /// Initial interest rate
  #[serde(rename = "InitialInterest")]
  initial_interest: Option<f64>,
  /// Interest rate after the loan was rescheduled
  #[serde(rename = "RescheduledInterest")]
  rescheduled_interest: Option<f64>,
  /// Write off total
  #[serde(rename = "WriteOffTotal")]
  write_off_total: Option<f64>,
  /// Debt servicing cost total
  #[serde(rename = "DebtServicingCostTotal")]
  debt_servicing_cost_total: Option<f64>,
  /// Total principal repaid amount to current note owner
  #[serde(rename = "RepaidPrincipalCurrentOwner")]
  repaid_principal_current_owner: Option<f64>,
  /// Total interest repaid amount to current note owner
  #[serde(rename = "RepaidInterestsCurrentOwner")]
  repaid_interests_current_owner: Option<f64>,
  /// Late charges paid amount to current note owner
  #[serde(rename = "LateChargesPaidCurrentOwner")]
  late_charges_paid_current_owner: Option<f64>,
  /// Total repaid amount to current note owner
  #[serde(rename = "RepaidTotalCurrentOwner")]
  repaid_total_current_owner: Option<f64>,
  /// Total repaid amount
  #[serde(rename = "TotalRepaid")]
  total_repaid: Option<f64>,
  /// Debt managment event collection
  #[serde(rename = "DebtManagmentEvents")]
  debt_managment_events: Option<Vec<crate::models::DebtManagementEvent>>,
  /// Collection of all loan payments
  #[serde(rename = "LoanTransfers")]
  loan_transfers: Option<Vec<crate::models::LoanTransfer>>,
  /// Collection of all loan scheduled payments.   Contains previous period values before rescheduling was made
  #[serde(rename = "ScheduledPayments")]
  scheduled_payments: Option<Vec<crate::models::ScheduledPayment>>,
  /// Loan unique identifier
  #[serde(rename = "LoanId")]
  loan_id: Option<String>,
  /// Loan original lenght
  #[serde(rename = "LoanDuration")]
  loan_duration: Option<i32>,
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
  /// LoanPart unique identifier
  #[serde(rename = "LoanPartId")]
  loan_part_id: Option<String>,
  /// Investment amount
  #[serde(rename = "Amount")]
  amount: Option<f64>,
  /// Auction unique identifier
  #[serde(rename = "AuctionId")]
  auction_id: Option<String>,
  /// Auction name
  #[serde(rename = "AuctionName")]
  auction_name: Option<String>,
  /// Auction number
  #[serde(rename = "AuctionNumber")]
  auction_number: Option<i32>,
  /// Auction bid number
  #[serde(rename = "AuctionBidNumber")]
  auction_bid_number: Option<i32>,
  /// Auction number + Auction bid number combined
  #[serde(rename = "InvestmentNumber")]
  investment_number: Option<String>,
  /// Residency of the borrower
  #[serde(rename = "Country")]
  country: Option<String>,
  /// <para>    1000 No previous payments problems</para>  <para>    900 Payments problems finished 24-36 months ago</para>  <para>    800 Payments problems finished 12-24 months ago</para>  <para>    700 Payments problems finished 6-12 months ago</para>  <para>    600 Payment problems finished &lt;6 months ago</para>  <para>    500 Active payment problems</para>
  #[serde(rename = "CreditScore")]
  credit_score: Option<f64>,
  /// Bondora Rating issued by the Rating model
  #[serde(rename = "Rating")]
  rating: Option<String>,
  /// Current interest rate
  #[serde(rename = "Interest")]
  interest: Option<f64>,
  /// Use of loan
  #[serde(rename = "UseOfLoan")]
  use_of_loan: Option<i32>,
  /// Income verification type
  #[serde(rename = "IncomeVerificationStatus")]
  income_verification_status: Option<i32>,
  /// Loan status code  <para>0 Reserved</para><para>2 Current</para><para>3 Cancelled</para><para>100 Overdue</para><para>5 60+ days overdue</para><para>4 Repaid</para><para>8 Released</para>
  #[serde(rename = "LoanStatusCode")]
  loan_status_code: Option<i32>,
  /// Borrower's username
  #[serde(rename = "UserName")]
  user_name: Option<String>,
  /// Borrower's Gender
  #[serde(rename = "Gender")]
  gender: Option<i32>,
  /// Borrower's date of birth
  #[serde(rename = "DateOfBirth")]
  date_of_birth: Option<String>,
  /// Loan issued date
  #[serde(rename = "SignedDate")]
  signed_date: Option<String>,
  /// Last rescheduling date
  #[serde(rename = "ReScheduledOn")]
  re_scheduled_on: Option<String>,
  /// Date and time when the principal part of the payment is overdue (PrincipalLateAmount is greater than zero).
  #[serde(rename = "DebtOccuredOn")]
  debt_occured_on: Option<String>,
  /// Date and time when loan part payment is overdue (principal, interest or penalty) aka when the dept occured for the loan part (LateAmountTotal is greater than zero).
  #[serde(rename = "DebtOccuredOnForSecondary")]
  debt_occured_on_for_secondary: Option<String>,
  /// Next scheduled payment number
  #[serde(rename = "NextPaymentNr")]
  next_payment_nr: Option<i32>,
  /// Next scheduled payment date
  #[serde(rename = "NextPaymentDate")]
  next_payment_date: Option<String>,
  /// Next scheduled payment amount
  #[serde(rename = "NextPaymentSum")]
  next_payment_sum: Option<f64>,
  /// Total number of scheduled payments
  #[serde(rename = "NrOfScheduledPayments")]
  nr_of_scheduled_payments: Option<i32>,
  /// Last payment date
  #[serde(rename = "LastPaymentDate")]
  last_payment_date: Option<String>,
  /// Total principal repaid amount
  #[serde(rename = "PrincipalRepaid")]
  principal_repaid: Option<f64>,
  /// Total interest repaid amount
  #[serde(rename = "InterestRepaid")]
  interest_repaid: Option<f64>,
  /// Total late charges paid amount
  #[serde(rename = "LateAmountPaid")]
  late_amount_paid: Option<f64>,
  /// Remaining principal amount
  #[serde(rename = "PrincipalRemaining")]
  principal_remaining: Option<f64>,
  /// Principal debt amount
  #[serde(rename = "PrincipalLateAmount")]
  principal_late_amount: Option<f64>,
  /// Interest debt amount
  #[serde(rename = "InterestLateAmount")]
  interest_late_amount: Option<f64>,
  /// Late charges debt amount
  #[serde(rename = "PenaltyLateAmount")]
  penalty_late_amount: Option<f64>,
  /// Late amount total
  #[serde(rename = "LateAmountTotal")]
  late_amount_total: Option<f64>,
  /// Total amount of principal written off
  #[serde(rename = "PrincipalWriteOffAmount")]
  principal_write_off_amount: Option<f64>,
  /// Total amount of interest written off
  #[serde(rename = "InterestWriteOffAmount")]
  interest_write_off_amount: Option<f64>,
  /// Total amount of penalty written off
  #[serde(rename = "PenaltyWriteOffAmount")]
  penalty_write_off_amount: Option<f64>,
  /// Total amount of principal debt servicing cost
  #[serde(rename = "DebtServicingCostMainAmount")]
  debt_servicing_cost_main_amount: Option<f64>,
  /// Total amount of interest debt servicing cost
  #[serde(rename = "DebtServicingCostInterestAmount")]
  debt_servicing_cost_interest_amount: Option<f64>,
  /// Total amount of penalty debt servicing cost
  #[serde(rename = "DebtServicingCostPenaltyAmount")]
  debt_servicing_cost_penalty_amount: Option<f64>
}

impl LoanPartDetails {
  /// LoanPart related data with summary, collection process and schedules
  pub fn new() -> LoanPartDetails {
    LoanPartDetails {
      initial_interest: None,
      rescheduled_interest: None,
      write_off_total: None,
      debt_servicing_cost_total: None,
      repaid_principal_current_owner: None,
      repaid_interests_current_owner: None,
      late_charges_paid_current_owner: None,
      repaid_total_current_owner: None,
      total_repaid: None,
      debt_managment_events: None,
      loan_transfers: None,
      scheduled_payments: None,
      loan_id: None,
      loan_duration: None,
      credit_score_es_micro_l: None,
      credit_score_es_equifax_risk: None,
      credit_score_fi_asiakas_tieto_risk_grade: None,
      credit_score_ee_mini: None,
      loan_part_id: None,
      amount: None,
      auction_id: None,
      auction_name: None,
      auction_number: None,
      auction_bid_number: None,
      investment_number: None,
      country: None,
      credit_score: None,
      rating: None,
      interest: None,
      use_of_loan: None,
      income_verification_status: None,
      loan_status_code: None,
      user_name: None,
      gender: None,
      date_of_birth: None,
      signed_date: None,
      re_scheduled_on: None,
      debt_occured_on: None,
      debt_occured_on_for_secondary: None,
      next_payment_nr: None,
      next_payment_date: None,
      next_payment_sum: None,
      nr_of_scheduled_payments: None,
      last_payment_date: None,
      principal_repaid: None,
      interest_repaid: None,
      late_amount_paid: None,
      principal_remaining: None,
      principal_late_amount: None,
      interest_late_amount: None,
      penalty_late_amount: None,
      late_amount_total: None,
      principal_write_off_amount: None,
      interest_write_off_amount: None,
      penalty_write_off_amount: None,
      debt_servicing_cost_main_amount: None,
      debt_servicing_cost_interest_amount: None,
      debt_servicing_cost_penalty_amount: None
    }
  }

  pub fn set_initial_interest(&mut self, initial_interest: f64) {
    self.initial_interest = Some(initial_interest);
  }

  pub fn with_initial_interest(mut self, initial_interest: f64) -> LoanPartDetails {
    self.initial_interest = Some(initial_interest);
    self
  }

  pub fn initial_interest(&self) -> Option<&f64> {
    self.initial_interest.as_ref()
  }

  pub fn reset_initial_interest(&mut self) {
    self.initial_interest = None;
  }

  pub fn set_rescheduled_interest(&mut self, rescheduled_interest: f64) {
    self.rescheduled_interest = Some(rescheduled_interest);
  }

  pub fn with_rescheduled_interest(mut self, rescheduled_interest: f64) -> LoanPartDetails {
    self.rescheduled_interest = Some(rescheduled_interest);
    self
  }

  pub fn rescheduled_interest(&self) -> Option<&f64> {
    self.rescheduled_interest.as_ref()
  }

  pub fn reset_rescheduled_interest(&mut self) {
    self.rescheduled_interest = None;
  }

  pub fn set_write_off_total(&mut self, write_off_total: f64) {
    self.write_off_total = Some(write_off_total);
  }

  pub fn with_write_off_total(mut self, write_off_total: f64) -> LoanPartDetails {
    self.write_off_total = Some(write_off_total);
    self
  }

  pub fn write_off_total(&self) -> Option<&f64> {
    self.write_off_total.as_ref()
  }

  pub fn reset_write_off_total(&mut self) {
    self.write_off_total = None;
  }

  pub fn set_debt_servicing_cost_total(&mut self, debt_servicing_cost_total: f64) {
    self.debt_servicing_cost_total = Some(debt_servicing_cost_total);
  }

  pub fn with_debt_servicing_cost_total(mut self, debt_servicing_cost_total: f64) -> LoanPartDetails {
    self.debt_servicing_cost_total = Some(debt_servicing_cost_total);
    self
  }

  pub fn debt_servicing_cost_total(&self) -> Option<&f64> {
    self.debt_servicing_cost_total.as_ref()
  }

  pub fn reset_debt_servicing_cost_total(&mut self) {
    self.debt_servicing_cost_total = None;
  }

  pub fn set_repaid_principal_current_owner(&mut self, repaid_principal_current_owner: f64) {
    self.repaid_principal_current_owner = Some(repaid_principal_current_owner);
  }

  pub fn with_repaid_principal_current_owner(mut self, repaid_principal_current_owner: f64) -> LoanPartDetails {
    self.repaid_principal_current_owner = Some(repaid_principal_current_owner);
    self
  }

  pub fn repaid_principal_current_owner(&self) -> Option<&f64> {
    self.repaid_principal_current_owner.as_ref()
  }

  pub fn reset_repaid_principal_current_owner(&mut self) {
    self.repaid_principal_current_owner = None;
  }

  pub fn set_repaid_interests_current_owner(&mut self, repaid_interests_current_owner: f64) {
    self.repaid_interests_current_owner = Some(repaid_interests_current_owner);
  }

  pub fn with_repaid_interests_current_owner(mut self, repaid_interests_current_owner: f64) -> LoanPartDetails {
    self.repaid_interests_current_owner = Some(repaid_interests_current_owner);
    self
  }

  pub fn repaid_interests_current_owner(&self) -> Option<&f64> {
    self.repaid_interests_current_owner.as_ref()
  }

  pub fn reset_repaid_interests_current_owner(&mut self) {
    self.repaid_interests_current_owner = None;
  }

  pub fn set_late_charges_paid_current_owner(&mut self, late_charges_paid_current_owner: f64) {
    self.late_charges_paid_current_owner = Some(late_charges_paid_current_owner);
  }

  pub fn with_late_charges_paid_current_owner(mut self, late_charges_paid_current_owner: f64) -> LoanPartDetails {
    self.late_charges_paid_current_owner = Some(late_charges_paid_current_owner);
    self
  }

  pub fn late_charges_paid_current_owner(&self) -> Option<&f64> {
    self.late_charges_paid_current_owner.as_ref()
  }

  pub fn reset_late_charges_paid_current_owner(&mut self) {
    self.late_charges_paid_current_owner = None;
  }

  pub fn set_repaid_total_current_owner(&mut self, repaid_total_current_owner: f64) {
    self.repaid_total_current_owner = Some(repaid_total_current_owner);
  }

  pub fn with_repaid_total_current_owner(mut self, repaid_total_current_owner: f64) -> LoanPartDetails {
    self.repaid_total_current_owner = Some(repaid_total_current_owner);
    self
  }

  pub fn repaid_total_current_owner(&self) -> Option<&f64> {
    self.repaid_total_current_owner.as_ref()
  }

  pub fn reset_repaid_total_current_owner(&mut self) {
    self.repaid_total_current_owner = None;
  }

  pub fn set_total_repaid(&mut self, total_repaid: f64) {
    self.total_repaid = Some(total_repaid);
  }

  pub fn with_total_repaid(mut self, total_repaid: f64) -> LoanPartDetails {
    self.total_repaid = Some(total_repaid);
    self
  }

  pub fn total_repaid(&self) -> Option<&f64> {
    self.total_repaid.as_ref()
  }

  pub fn reset_total_repaid(&mut self) {
    self.total_repaid = None;
  }

  pub fn set_debt_managment_events(&mut self, debt_managment_events: Vec<crate::models::DebtManagementEvent>) {
    self.debt_managment_events = Some(debt_managment_events);
  }

  pub fn with_debt_managment_events(mut self, debt_managment_events: Vec<crate::models::DebtManagementEvent>) -> LoanPartDetails {
    self.debt_managment_events = Some(debt_managment_events);
    self
  }

  pub fn debt_managment_events(&self) -> Option<&Vec<crate::models::DebtManagementEvent>> {
    self.debt_managment_events.as_ref()
  }

  pub fn reset_debt_managment_events(&mut self) {
    self.debt_managment_events = None;
  }

  pub fn set_loan_transfers(&mut self, loan_transfers: Vec<crate::models::LoanTransfer>) {
    self.loan_transfers = Some(loan_transfers);
  }

  pub fn with_loan_transfers(mut self, loan_transfers: Vec<crate::models::LoanTransfer>) -> LoanPartDetails {
    self.loan_transfers = Some(loan_transfers);
    self
  }

  pub fn loan_transfers(&self) -> Option<&Vec<crate::models::LoanTransfer>> {
    self.loan_transfers.as_ref()
  }

  pub fn reset_loan_transfers(&mut self) {
    self.loan_transfers = None;
  }

  pub fn set_scheduled_payments(&mut self, scheduled_payments: Vec<crate::models::ScheduledPayment>) {
    self.scheduled_payments = Some(scheduled_payments);
  }

  pub fn with_scheduled_payments(mut self, scheduled_payments: Vec<crate::models::ScheduledPayment>) -> LoanPartDetails {
    self.scheduled_payments = Some(scheduled_payments);
    self
  }

  pub fn scheduled_payments(&self) -> Option<&Vec<crate::models::ScheduledPayment>> {
    self.scheduled_payments.as_ref()
  }

  pub fn reset_scheduled_payments(&mut self) {
    self.scheduled_payments = None;
  }

  pub fn set_loan_id(&mut self, loan_id: String) {
    self.loan_id = Some(loan_id);
  }

  pub fn with_loan_id(mut self, loan_id: String) -> LoanPartDetails {
    self.loan_id = Some(loan_id);
    self
  }

  pub fn loan_id(&self) -> Option<&String> {
    self.loan_id.as_ref()
  }

  pub fn reset_loan_id(&mut self) {
    self.loan_id = None;
  }

  pub fn set_loan_duration(&mut self, loan_duration: i32) {
    self.loan_duration = Some(loan_duration);
  }

  pub fn with_loan_duration(mut self, loan_duration: i32) -> LoanPartDetails {
    self.loan_duration = Some(loan_duration);
    self
  }

  pub fn loan_duration(&self) -> Option<&i32> {
    self.loan_duration.as_ref()
  }

  pub fn reset_loan_duration(&mut self) {
    self.loan_duration = None;
  }

  pub fn set_credit_score_es_micro_l(&mut self, credit_score_es_micro_l: String) {
    self.credit_score_es_micro_l = Some(credit_score_es_micro_l);
  }

  pub fn with_credit_score_es_micro_l(mut self, credit_score_es_micro_l: String) -> LoanPartDetails {
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

  pub fn with_credit_score_es_equifax_risk(mut self, credit_score_es_equifax_risk: String) -> LoanPartDetails {
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

  pub fn with_credit_score_fi_asiakas_tieto_risk_grade(mut self, credit_score_fi_asiakas_tieto_risk_grade: String) -> LoanPartDetails {
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

  pub fn with_credit_score_ee_mini(mut self, credit_score_ee_mini: String) -> LoanPartDetails {
    self.credit_score_ee_mini = Some(credit_score_ee_mini);
    self
  }

  pub fn credit_score_ee_mini(&self) -> Option<&String> {
    self.credit_score_ee_mini.as_ref()
  }

  pub fn reset_credit_score_ee_mini(&mut self) {
    self.credit_score_ee_mini = None;
  }

  pub fn set_loan_part_id(&mut self, loan_part_id: String) {
    self.loan_part_id = Some(loan_part_id);
  }

  pub fn with_loan_part_id(mut self, loan_part_id: String) -> LoanPartDetails {
    self.loan_part_id = Some(loan_part_id);
    self
  }

  pub fn loan_part_id(&self) -> Option<&String> {
    self.loan_part_id.as_ref()
  }

  pub fn reset_loan_part_id(&mut self) {
    self.loan_part_id = None;
  }

  pub fn set_amount(&mut self, amount: f64) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: f64) -> LoanPartDetails {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&f64> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_auction_id(&mut self, auction_id: String) {
    self.auction_id = Some(auction_id);
  }

  pub fn with_auction_id(mut self, auction_id: String) -> LoanPartDetails {
    self.auction_id = Some(auction_id);
    self
  }

  pub fn auction_id(&self) -> Option<&String> {
    self.auction_id.as_ref()
  }

  pub fn reset_auction_id(&mut self) {
    self.auction_id = None;
  }

  pub fn set_auction_name(&mut self, auction_name: String) {
    self.auction_name = Some(auction_name);
  }

  pub fn with_auction_name(mut self, auction_name: String) -> LoanPartDetails {
    self.auction_name = Some(auction_name);
    self
  }

  pub fn auction_name(&self) -> Option<&String> {
    self.auction_name.as_ref()
  }

  pub fn reset_auction_name(&mut self) {
    self.auction_name = None;
  }

  pub fn set_auction_number(&mut self, auction_number: i32) {
    self.auction_number = Some(auction_number);
  }

  pub fn with_auction_number(mut self, auction_number: i32) -> LoanPartDetails {
    self.auction_number = Some(auction_number);
    self
  }

  pub fn auction_number(&self) -> Option<&i32> {
    self.auction_number.as_ref()
  }

  pub fn reset_auction_number(&mut self) {
    self.auction_number = None;
  }

  pub fn set_auction_bid_number(&mut self, auction_bid_number: i32) {
    self.auction_bid_number = Some(auction_bid_number);
  }

  pub fn with_auction_bid_number(mut self, auction_bid_number: i32) -> LoanPartDetails {
    self.auction_bid_number = Some(auction_bid_number);
    self
  }

  pub fn auction_bid_number(&self) -> Option<&i32> {
    self.auction_bid_number.as_ref()
  }

  pub fn reset_auction_bid_number(&mut self) {
    self.auction_bid_number = None;
  }

  pub fn set_investment_number(&mut self, investment_number: String) {
    self.investment_number = Some(investment_number);
  }

  pub fn with_investment_number(mut self, investment_number: String) -> LoanPartDetails {
    self.investment_number = Some(investment_number);
    self
  }

  pub fn investment_number(&self) -> Option<&String> {
    self.investment_number.as_ref()
  }

  pub fn reset_investment_number(&mut self) {
    self.investment_number = None;
  }

  pub fn set_country(&mut self, country: String) {
    self.country = Some(country);
  }

  pub fn with_country(mut self, country: String) -> LoanPartDetails {
    self.country = Some(country);
    self
  }

  pub fn country(&self) -> Option<&String> {
    self.country.as_ref()
  }

  pub fn reset_country(&mut self) {
    self.country = None;
  }

  pub fn set_credit_score(&mut self, credit_score: f64) {
    self.credit_score = Some(credit_score);
  }

  pub fn with_credit_score(mut self, credit_score: f64) -> LoanPartDetails {
    self.credit_score = Some(credit_score);
    self
  }

  pub fn credit_score(&self) -> Option<&f64> {
    self.credit_score.as_ref()
  }

  pub fn reset_credit_score(&mut self) {
    self.credit_score = None;
  }

  pub fn set_rating(&mut self, rating: String) {
    self.rating = Some(rating);
  }

  pub fn with_rating(mut self, rating: String) -> LoanPartDetails {
    self.rating = Some(rating);
    self
  }

  pub fn rating(&self) -> Option<&String> {
    self.rating.as_ref()
  }

  pub fn reset_rating(&mut self) {
    self.rating = None;
  }

  pub fn set_interest(&mut self, interest: f64) {
    self.interest = Some(interest);
  }

  pub fn with_interest(mut self, interest: f64) -> LoanPartDetails {
    self.interest = Some(interest);
    self
  }

  pub fn interest(&self) -> Option<&f64> {
    self.interest.as_ref()
  }

  pub fn reset_interest(&mut self) {
    self.interest = None;
  }

  pub fn set_use_of_loan(&mut self, use_of_loan: i32) {
    self.use_of_loan = Some(use_of_loan);
  }

  pub fn with_use_of_loan(mut self, use_of_loan: i32) -> LoanPartDetails {
    self.use_of_loan = Some(use_of_loan);
    self
  }

  pub fn use_of_loan(&self) -> Option<&i32> {
    self.use_of_loan.as_ref()
  }

  pub fn reset_use_of_loan(&mut self) {
    self.use_of_loan = None;
  }

  pub fn set_income_verification_status(&mut self, income_verification_status: i32) {
    self.income_verification_status = Some(income_verification_status);
  }

  pub fn with_income_verification_status(mut self, income_verification_status: i32) -> LoanPartDetails {
    self.income_verification_status = Some(income_verification_status);
    self
  }

  pub fn income_verification_status(&self) -> Option<&i32> {
    self.income_verification_status.as_ref()
  }

  pub fn reset_income_verification_status(&mut self) {
    self.income_verification_status = None;
  }

  pub fn set_loan_status_code(&mut self, loan_status_code: i32) {
    self.loan_status_code = Some(loan_status_code);
  }

  pub fn with_loan_status_code(mut self, loan_status_code: i32) -> LoanPartDetails {
    self.loan_status_code = Some(loan_status_code);
    self
  }

  pub fn loan_status_code(&self) -> Option<&i32> {
    self.loan_status_code.as_ref()
  }

  pub fn reset_loan_status_code(&mut self) {
    self.loan_status_code = None;
  }

  pub fn set_user_name(&mut self, user_name: String) {
    self.user_name = Some(user_name);
  }

  pub fn with_user_name(mut self, user_name: String) -> LoanPartDetails {
    self.user_name = Some(user_name);
    self
  }

  pub fn user_name(&self) -> Option<&String> {
    self.user_name.as_ref()
  }

  pub fn reset_user_name(&mut self) {
    self.user_name = None;
  }

  pub fn set_gender(&mut self, gender: i32) {
    self.gender = Some(gender);
  }

  pub fn with_gender(mut self, gender: i32) -> LoanPartDetails {
    self.gender = Some(gender);
    self
  }

  pub fn gender(&self) -> Option<&i32> {
    self.gender.as_ref()
  }

  pub fn reset_gender(&mut self) {
    self.gender = None;
  }

  pub fn set_date_of_birth(&mut self, date_of_birth: String) {
    self.date_of_birth = Some(date_of_birth);
  }

  pub fn with_date_of_birth(mut self, date_of_birth: String) -> LoanPartDetails {
    self.date_of_birth = Some(date_of_birth);
    self
  }

  pub fn date_of_birth(&self) -> Option<&String> {
    self.date_of_birth.as_ref()
  }

  pub fn reset_date_of_birth(&mut self) {
    self.date_of_birth = None;
  }

  pub fn set_signed_date(&mut self, signed_date: String) {
    self.signed_date = Some(signed_date);
  }

  pub fn with_signed_date(mut self, signed_date: String) -> LoanPartDetails {
    self.signed_date = Some(signed_date);
    self
  }

  pub fn signed_date(&self) -> Option<&String> {
    self.signed_date.as_ref()
  }

  pub fn reset_signed_date(&mut self) {
    self.signed_date = None;
  }

  pub fn set_re_scheduled_on(&mut self, re_scheduled_on: String) {
    self.re_scheduled_on = Some(re_scheduled_on);
  }

  pub fn with_re_scheduled_on(mut self, re_scheduled_on: String) -> LoanPartDetails {
    self.re_scheduled_on = Some(re_scheduled_on);
    self
  }

  pub fn re_scheduled_on(&self) -> Option<&String> {
    self.re_scheduled_on.as_ref()
  }

  pub fn reset_re_scheduled_on(&mut self) {
    self.re_scheduled_on = None;
  }

  pub fn set_debt_occured_on(&mut self, debt_occured_on: String) {
    self.debt_occured_on = Some(debt_occured_on);
  }

  pub fn with_debt_occured_on(mut self, debt_occured_on: String) -> LoanPartDetails {
    self.debt_occured_on = Some(debt_occured_on);
    self
  }

  pub fn debt_occured_on(&self) -> Option<&String> {
    self.debt_occured_on.as_ref()
  }

  pub fn reset_debt_occured_on(&mut self) {
    self.debt_occured_on = None;
  }

  pub fn set_debt_occured_on_for_secondary(&mut self, debt_occured_on_for_secondary: String) {
    self.debt_occured_on_for_secondary = Some(debt_occured_on_for_secondary);
  }

  pub fn with_debt_occured_on_for_secondary(mut self, debt_occured_on_for_secondary: String) -> LoanPartDetails {
    self.debt_occured_on_for_secondary = Some(debt_occured_on_for_secondary);
    self
  }

  pub fn debt_occured_on_for_secondary(&self) -> Option<&String> {
    self.debt_occured_on_for_secondary.as_ref()
  }

  pub fn reset_debt_occured_on_for_secondary(&mut self) {
    self.debt_occured_on_for_secondary = None;
  }

  pub fn set_next_payment_nr(&mut self, next_payment_nr: i32) {
    self.next_payment_nr = Some(next_payment_nr);
  }

  pub fn with_next_payment_nr(mut self, next_payment_nr: i32) -> LoanPartDetails {
    self.next_payment_nr = Some(next_payment_nr);
    self
  }

  pub fn next_payment_nr(&self) -> Option<&i32> {
    self.next_payment_nr.as_ref()
  }

  pub fn reset_next_payment_nr(&mut self) {
    self.next_payment_nr = None;
  }

  pub fn set_next_payment_date(&mut self, next_payment_date: String) {
    self.next_payment_date = Some(next_payment_date);
  }

  pub fn with_next_payment_date(mut self, next_payment_date: String) -> LoanPartDetails {
    self.next_payment_date = Some(next_payment_date);
    self
  }

  pub fn next_payment_date(&self) -> Option<&String> {
    self.next_payment_date.as_ref()
  }

  pub fn reset_next_payment_date(&mut self) {
    self.next_payment_date = None;
  }

  pub fn set_next_payment_sum(&mut self, next_payment_sum: f64) {
    self.next_payment_sum = Some(next_payment_sum);
  }

  pub fn with_next_payment_sum(mut self, next_payment_sum: f64) -> LoanPartDetails {
    self.next_payment_sum = Some(next_payment_sum);
    self
  }

  pub fn next_payment_sum(&self) -> Option<&f64> {
    self.next_payment_sum.as_ref()
  }

  pub fn reset_next_payment_sum(&mut self) {
    self.next_payment_sum = None;
  }

  pub fn set_nr_of_scheduled_payments(&mut self, nr_of_scheduled_payments: i32) {
    self.nr_of_scheduled_payments = Some(nr_of_scheduled_payments);
  }

  pub fn with_nr_of_scheduled_payments(mut self, nr_of_scheduled_payments: i32) -> LoanPartDetails {
    self.nr_of_scheduled_payments = Some(nr_of_scheduled_payments);
    self
  }

  pub fn nr_of_scheduled_payments(&self) -> Option<&i32> {
    self.nr_of_scheduled_payments.as_ref()
  }

  pub fn reset_nr_of_scheduled_payments(&mut self) {
    self.nr_of_scheduled_payments = None;
  }

  pub fn set_last_payment_date(&mut self, last_payment_date: String) {
    self.last_payment_date = Some(last_payment_date);
  }

  pub fn with_last_payment_date(mut self, last_payment_date: String) -> LoanPartDetails {
    self.last_payment_date = Some(last_payment_date);
    self
  }

  pub fn last_payment_date(&self) -> Option<&String> {
    self.last_payment_date.as_ref()
  }

  pub fn reset_last_payment_date(&mut self) {
    self.last_payment_date = None;
  }

  pub fn set_principal_repaid(&mut self, principal_repaid: f64) {
    self.principal_repaid = Some(principal_repaid);
  }

  pub fn with_principal_repaid(mut self, principal_repaid: f64) -> LoanPartDetails {
    self.principal_repaid = Some(principal_repaid);
    self
  }

  pub fn principal_repaid(&self) -> Option<&f64> {
    self.principal_repaid.as_ref()
  }

  pub fn reset_principal_repaid(&mut self) {
    self.principal_repaid = None;
  }

  pub fn set_interest_repaid(&mut self, interest_repaid: f64) {
    self.interest_repaid = Some(interest_repaid);
  }

  pub fn with_interest_repaid(mut self, interest_repaid: f64) -> LoanPartDetails {
    self.interest_repaid = Some(interest_repaid);
    self
  }

  pub fn interest_repaid(&self) -> Option<&f64> {
    self.interest_repaid.as_ref()
  }

  pub fn reset_interest_repaid(&mut self) {
    self.interest_repaid = None;
  }

  pub fn set_late_amount_paid(&mut self, late_amount_paid: f64) {
    self.late_amount_paid = Some(late_amount_paid);
  }

  pub fn with_late_amount_paid(mut self, late_amount_paid: f64) -> LoanPartDetails {
    self.late_amount_paid = Some(late_amount_paid);
    self
  }

  pub fn late_amount_paid(&self) -> Option<&f64> {
    self.late_amount_paid.as_ref()
  }

  pub fn reset_late_amount_paid(&mut self) {
    self.late_amount_paid = None;
  }

  pub fn set_principal_remaining(&mut self, principal_remaining: f64) {
    self.principal_remaining = Some(principal_remaining);
  }

  pub fn with_principal_remaining(mut self, principal_remaining: f64) -> LoanPartDetails {
    self.principal_remaining = Some(principal_remaining);
    self
  }

  pub fn principal_remaining(&self) -> Option<&f64> {
    self.principal_remaining.as_ref()
  }

  pub fn reset_principal_remaining(&mut self) {
    self.principal_remaining = None;
  }

  pub fn set_principal_late_amount(&mut self, principal_late_amount: f64) {
    self.principal_late_amount = Some(principal_late_amount);
  }

  pub fn with_principal_late_amount(mut self, principal_late_amount: f64) -> LoanPartDetails {
    self.principal_late_amount = Some(principal_late_amount);
    self
  }

  pub fn principal_late_amount(&self) -> Option<&f64> {
    self.principal_late_amount.as_ref()
  }

  pub fn reset_principal_late_amount(&mut self) {
    self.principal_late_amount = None;
  }

  pub fn set_interest_late_amount(&mut self, interest_late_amount: f64) {
    self.interest_late_amount = Some(interest_late_amount);
  }

  pub fn with_interest_late_amount(mut self, interest_late_amount: f64) -> LoanPartDetails {
    self.interest_late_amount = Some(interest_late_amount);
    self
  }

  pub fn interest_late_amount(&self) -> Option<&f64> {
    self.interest_late_amount.as_ref()
  }

  pub fn reset_interest_late_amount(&mut self) {
    self.interest_late_amount = None;
  }

  pub fn set_penalty_late_amount(&mut self, penalty_late_amount: f64) {
    self.penalty_late_amount = Some(penalty_late_amount);
  }

  pub fn with_penalty_late_amount(mut self, penalty_late_amount: f64) -> LoanPartDetails {
    self.penalty_late_amount = Some(penalty_late_amount);
    self
  }

  pub fn penalty_late_amount(&self) -> Option<&f64> {
    self.penalty_late_amount.as_ref()
  }

  pub fn reset_penalty_late_amount(&mut self) {
    self.penalty_late_amount = None;
  }

  pub fn set_late_amount_total(&mut self, late_amount_total: f64) {
    self.late_amount_total = Some(late_amount_total);
  }

  pub fn with_late_amount_total(mut self, late_amount_total: f64) -> LoanPartDetails {
    self.late_amount_total = Some(late_amount_total);
    self
  }

  pub fn late_amount_total(&self) -> Option<&f64> {
    self.late_amount_total.as_ref()
  }

  pub fn reset_late_amount_total(&mut self) {
    self.late_amount_total = None;
  }

  pub fn set_principal_write_off_amount(&mut self, principal_write_off_amount: f64) {
    self.principal_write_off_amount = Some(principal_write_off_amount);
  }

  pub fn with_principal_write_off_amount(mut self, principal_write_off_amount: f64) -> LoanPartDetails {
    self.principal_write_off_amount = Some(principal_write_off_amount);
    self
  }

  pub fn principal_write_off_amount(&self) -> Option<&f64> {
    self.principal_write_off_amount.as_ref()
  }

  pub fn reset_principal_write_off_amount(&mut self) {
    self.principal_write_off_amount = None;
  }

  pub fn set_interest_write_off_amount(&mut self, interest_write_off_amount: f64) {
    self.interest_write_off_amount = Some(interest_write_off_amount);
  }

  pub fn with_interest_write_off_amount(mut self, interest_write_off_amount: f64) -> LoanPartDetails {
    self.interest_write_off_amount = Some(interest_write_off_amount);
    self
  }

  pub fn interest_write_off_amount(&self) -> Option<&f64> {
    self.interest_write_off_amount.as_ref()
  }

  pub fn reset_interest_write_off_amount(&mut self) {
    self.interest_write_off_amount = None;
  }

  pub fn set_penalty_write_off_amount(&mut self, penalty_write_off_amount: f64) {
    self.penalty_write_off_amount = Some(penalty_write_off_amount);
  }

  pub fn with_penalty_write_off_amount(mut self, penalty_write_off_amount: f64) -> LoanPartDetails {
    self.penalty_write_off_amount = Some(penalty_write_off_amount);
    self
  }

  pub fn penalty_write_off_amount(&self) -> Option<&f64> {
    self.penalty_write_off_amount.as_ref()
  }

  pub fn reset_penalty_write_off_amount(&mut self) {
    self.penalty_write_off_amount = None;
  }

  pub fn set_debt_servicing_cost_main_amount(&mut self, debt_servicing_cost_main_amount: f64) {
    self.debt_servicing_cost_main_amount = Some(debt_servicing_cost_main_amount);
  }

  pub fn with_debt_servicing_cost_main_amount(mut self, debt_servicing_cost_main_amount: f64) -> LoanPartDetails {
    self.debt_servicing_cost_main_amount = Some(debt_servicing_cost_main_amount);
    self
  }

  pub fn debt_servicing_cost_main_amount(&self) -> Option<&f64> {
    self.debt_servicing_cost_main_amount.as_ref()
  }

  pub fn reset_debt_servicing_cost_main_amount(&mut self) {
    self.debt_servicing_cost_main_amount = None;
  }

  pub fn set_debt_servicing_cost_interest_amount(&mut self, debt_servicing_cost_interest_amount: f64) {
    self.debt_servicing_cost_interest_amount = Some(debt_servicing_cost_interest_amount);
  }

  pub fn with_debt_servicing_cost_interest_amount(mut self, debt_servicing_cost_interest_amount: f64) -> LoanPartDetails {
    self.debt_servicing_cost_interest_amount = Some(debt_servicing_cost_interest_amount);
    self
  }

  pub fn debt_servicing_cost_interest_amount(&self) -> Option<&f64> {
    self.debt_servicing_cost_interest_amount.as_ref()
  }

  pub fn reset_debt_servicing_cost_interest_amount(&mut self) {
    self.debt_servicing_cost_interest_amount = None;
  }

  pub fn set_debt_servicing_cost_penalty_amount(&mut self, debt_servicing_cost_penalty_amount: f64) {
    self.debt_servicing_cost_penalty_amount = Some(debt_servicing_cost_penalty_amount);
  }

  pub fn with_debt_servicing_cost_penalty_amount(mut self, debt_servicing_cost_penalty_amount: f64) -> LoanPartDetails {
    self.debt_servicing_cost_penalty_amount = Some(debt_servicing_cost_penalty_amount);
    self
  }

  pub fn debt_servicing_cost_penalty_amount(&self) -> Option<&f64> {
    self.debt_servicing_cost_penalty_amount.as_ref()
  }

  pub fn reset_debt_servicing_cost_penalty_amount(&mut self) {
    self.debt_servicing_cost_penalty_amount = None;
  }

}



