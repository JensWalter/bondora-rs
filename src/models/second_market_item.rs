
/// SecondMarketItem : SecondaryMarket list item's information

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketItem {
  /// Item unique identifier
  #[serde(rename = "Id")]
  id: Option<String>,
  /// Loan status active from
  #[serde(rename = "LoanStatusActiveFrom")]
  loan_status_active_from: Option<String>,
  /// Latest debt management stage type
  #[serde(rename = "LatestDebtManagementStageType")]
  latest_debt_management_stage_type: Option<i32>,
  /// Latest debt management date
  #[serde(rename = "LatestDebtManagementDate")]
  latest_debt_management_date: Option<String>,
  /// Outstanding principal balance +/- discount or mark-up
  #[serde(rename = "Price")]
  price: Option<f64>,
  /// Secondary market purchase fee paid to Bondora
  #[serde(rename = "Fee")]
  fee: Option<f64>,
  /// Total amount paid for purchase
  #[serde(rename = "TotalCost")]
  total_cost: Option<f64>,
  /// Total amount still to be repaid by the borrower. This includes the principal balance, accrued interest and late charges as well as any future scheduled interest payments
  #[serde(rename = "OutstandingPayments")]
  outstanding_payments: Option<f64>,
  /// Discount rate percent
  #[serde(rename = "DesiredDiscountRate")]
  desired_discount_rate: Option<f64>,
  /// XIRR (extended internal rate of return) is a methodology to calculate the net return using the loan issued date and amount,   loan repayment dates and amounts and the principal balance according to the original repayment date.   All overdue principal payments are written off immediately. No provisions for future losses are made and only received (not accrued or scheduled)   interest payments are taken into account.
  #[serde(rename = "Xirr")]
  xirr: Option<f64>,
  /// Date when item was published
  #[serde(rename = "ListedOnDate")]
  listed_on_date: Option<String>,
  /// Webhook only:  Debt managment event collection
  #[serde(rename = "DebtManagmentEvents")]
  debt_managment_events: Option<Vec<crate::models::DebtManagementEvent>>,
  /// Webhook only:  Collection of all loan payments
  #[serde(rename = "LoanTransfers")]
  loan_transfers: Option<Vec<crate::models::LoanTransfer>>,
  /// Webhook only:  Collection of all loan scheduled payments.   Contains previous period values before rescheduling was made
  #[serde(rename = "ScheduledPayments")]
  scheduled_payments: Option<Vec<crate::models::ScheduledPayment>>,
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

impl SecondMarketItem {
  /// SecondaryMarket list item's information
  pub fn new() -> SecondMarketItem {
    SecondMarketItem {
      id: None,
      loan_status_active_from: None,
      latest_debt_management_stage_type: None,
      latest_debt_management_date: None,
      price: None,
      fee: None,
      total_cost: None,
      outstanding_payments: None,
      desired_discount_rate: None,
      xirr: None,
      listed_on_date: None,
      debt_managment_events: None,
      loan_transfers: None,
      scheduled_payments: None,
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

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> SecondMarketItem {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_loan_status_active_from(&mut self, loan_status_active_from: String) {
    self.loan_status_active_from = Some(loan_status_active_from);
  }

  pub fn with_loan_status_active_from(mut self, loan_status_active_from: String) -> SecondMarketItem {
    self.loan_status_active_from = Some(loan_status_active_from);
    self
  }

  pub fn loan_status_active_from(&self) -> Option<&String> {
    self.loan_status_active_from.as_ref()
  }

  pub fn reset_loan_status_active_from(&mut self) {
    self.loan_status_active_from = None;
  }

  pub fn set_latest_debt_management_stage_type(&mut self, latest_debt_management_stage_type: i32) {
    self.latest_debt_management_stage_type = Some(latest_debt_management_stage_type);
  }

  pub fn with_latest_debt_management_stage_type(mut self, latest_debt_management_stage_type: i32) -> SecondMarketItem {
    self.latest_debt_management_stage_type = Some(latest_debt_management_stage_type);
    self
  }

  pub fn latest_debt_management_stage_type(&self) -> Option<&i32> {
    self.latest_debt_management_stage_type.as_ref()
  }

  pub fn reset_latest_debt_management_stage_type(&mut self) {
    self.latest_debt_management_stage_type = None;
  }

  pub fn set_latest_debt_management_date(&mut self, latest_debt_management_date: String) {
    self.latest_debt_management_date = Some(latest_debt_management_date);
  }

  pub fn with_latest_debt_management_date(mut self, latest_debt_management_date: String) -> SecondMarketItem {
    self.latest_debt_management_date = Some(latest_debt_management_date);
    self
  }

  pub fn latest_debt_management_date(&self) -> Option<&String> {
    self.latest_debt_management_date.as_ref()
  }

  pub fn reset_latest_debt_management_date(&mut self) {
    self.latest_debt_management_date = None;
  }

  pub fn set_price(&mut self, price: f64) {
    self.price = Some(price);
  }

  pub fn with_price(mut self, price: f64) -> SecondMarketItem {
    self.price = Some(price);
    self
  }

  pub fn price(&self) -> Option<&f64> {
    self.price.as_ref()
  }

  pub fn reset_price(&mut self) {
    self.price = None;
  }

  pub fn set_fee(&mut self, fee: f64) {
    self.fee = Some(fee);
  }

  pub fn with_fee(mut self, fee: f64) -> SecondMarketItem {
    self.fee = Some(fee);
    self
  }

  pub fn fee(&self) -> Option<&f64> {
    self.fee.as_ref()
  }

  pub fn reset_fee(&mut self) {
    self.fee = None;
  }

  pub fn set_total_cost(&mut self, total_cost: f64) {
    self.total_cost = Some(total_cost);
  }

  pub fn with_total_cost(mut self, total_cost: f64) -> SecondMarketItem {
    self.total_cost = Some(total_cost);
    self
  }

  pub fn total_cost(&self) -> Option<&f64> {
    self.total_cost.as_ref()
  }

  pub fn reset_total_cost(&mut self) {
    self.total_cost = None;
  }

  pub fn set_outstanding_payments(&mut self, outstanding_payments: f64) {
    self.outstanding_payments = Some(outstanding_payments);
  }

  pub fn with_outstanding_payments(mut self, outstanding_payments: f64) -> SecondMarketItem {
    self.outstanding_payments = Some(outstanding_payments);
    self
  }

  pub fn outstanding_payments(&self) -> Option<&f64> {
    self.outstanding_payments.as_ref()
  }

  pub fn reset_outstanding_payments(&mut self) {
    self.outstanding_payments = None;
  }

  pub fn set_desired_discount_rate(&mut self, desired_discount_rate: f64) {
    self.desired_discount_rate = Some(desired_discount_rate);
  }

  pub fn with_desired_discount_rate(mut self, desired_discount_rate: f64) -> SecondMarketItem {
    self.desired_discount_rate = Some(desired_discount_rate);
    self
  }

  pub fn desired_discount_rate(&self) -> Option<&f64> {
    self.desired_discount_rate.as_ref()
  }

  pub fn reset_desired_discount_rate(&mut self) {
    self.desired_discount_rate = None;
  }

  pub fn set_xirr(&mut self, xirr: f64) {
    self.xirr = Some(xirr);
  }

  pub fn with_xirr(mut self, xirr: f64) -> SecondMarketItem {
    self.xirr = Some(xirr);
    self
  }

  pub fn xirr(&self) -> Option<&f64> {
    self.xirr.as_ref()
  }

  pub fn reset_xirr(&mut self) {
    self.xirr = None;
  }

  pub fn set_listed_on_date(&mut self, listed_on_date: String) {
    self.listed_on_date = Some(listed_on_date);
  }

  pub fn with_listed_on_date(mut self, listed_on_date: String) -> SecondMarketItem {
    self.listed_on_date = Some(listed_on_date);
    self
  }

  pub fn listed_on_date(&self) -> Option<&String> {
    self.listed_on_date.as_ref()
  }

  pub fn reset_listed_on_date(&mut self) {
    self.listed_on_date = None;
  }

  pub fn set_debt_managment_events(&mut self, debt_managment_events: Vec<crate::models::DebtManagementEvent>) {
    self.debt_managment_events = Some(debt_managment_events);
  }

  pub fn with_debt_managment_events(mut self, debt_managment_events: Vec<crate::models::DebtManagementEvent>) -> SecondMarketItem {
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

  pub fn with_loan_transfers(mut self, loan_transfers: Vec<crate::models::LoanTransfer>) -> SecondMarketItem {
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

  pub fn with_scheduled_payments(mut self, scheduled_payments: Vec<crate::models::ScheduledPayment>) -> SecondMarketItem {
    self.scheduled_payments = Some(scheduled_payments);
    self
  }

  pub fn scheduled_payments(&self) -> Option<&Vec<crate::models::ScheduledPayment>> {
    self.scheduled_payments.as_ref()
  }

  pub fn reset_scheduled_payments(&mut self) {
    self.scheduled_payments = None;
  }

  pub fn set_loan_part_id(&mut self, loan_part_id: String) {
    self.loan_part_id = Some(loan_part_id);
  }

  pub fn with_loan_part_id(mut self, loan_part_id: String) -> SecondMarketItem {
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

  pub fn with_amount(mut self, amount: f64) -> SecondMarketItem {
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

  pub fn with_auction_id(mut self, auction_id: String) -> SecondMarketItem {
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

  pub fn with_auction_name(mut self, auction_name: String) -> SecondMarketItem {
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

  pub fn with_auction_number(mut self, auction_number: i32) -> SecondMarketItem {
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

  pub fn with_auction_bid_number(mut self, auction_bid_number: i32) -> SecondMarketItem {
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

  pub fn with_investment_number(mut self, investment_number: String) -> SecondMarketItem {
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

  pub fn with_country(mut self, country: String) -> SecondMarketItem {
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

  pub fn with_credit_score(mut self, credit_score: f64) -> SecondMarketItem {
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

  pub fn with_rating(mut self, rating: String) -> SecondMarketItem {
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

  pub fn with_interest(mut self, interest: f64) -> SecondMarketItem {
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

  pub fn with_use_of_loan(mut self, use_of_loan: i32) -> SecondMarketItem {
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

  pub fn with_income_verification_status(mut self, income_verification_status: i32) -> SecondMarketItem {
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

  pub fn with_loan_status_code(mut self, loan_status_code: i32) -> SecondMarketItem {
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

  pub fn with_user_name(mut self, user_name: String) -> SecondMarketItem {
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

  pub fn with_gender(mut self, gender: i32) -> SecondMarketItem {
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

  pub fn with_date_of_birth(mut self, date_of_birth: String) -> SecondMarketItem {
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

  pub fn with_signed_date(mut self, signed_date: String) -> SecondMarketItem {
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

  pub fn with_re_scheduled_on(mut self, re_scheduled_on: String) -> SecondMarketItem {
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

  pub fn with_debt_occured_on(mut self, debt_occured_on: String) -> SecondMarketItem {
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

  pub fn with_debt_occured_on_for_secondary(mut self, debt_occured_on_for_secondary: String) -> SecondMarketItem {
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

  pub fn with_next_payment_nr(mut self, next_payment_nr: i32) -> SecondMarketItem {
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

  pub fn with_next_payment_date(mut self, next_payment_date: String) -> SecondMarketItem {
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

  pub fn with_next_payment_sum(mut self, next_payment_sum: f64) -> SecondMarketItem {
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

  pub fn with_nr_of_scheduled_payments(mut self, nr_of_scheduled_payments: i32) -> SecondMarketItem {
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

  pub fn with_last_payment_date(mut self, last_payment_date: String) -> SecondMarketItem {
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

  pub fn with_principal_repaid(mut self, principal_repaid: f64) -> SecondMarketItem {
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

  pub fn with_interest_repaid(mut self, interest_repaid: f64) -> SecondMarketItem {
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

  pub fn with_late_amount_paid(mut self, late_amount_paid: f64) -> SecondMarketItem {
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

  pub fn with_principal_remaining(mut self, principal_remaining: f64) -> SecondMarketItem {
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

  pub fn with_principal_late_amount(mut self, principal_late_amount: f64) -> SecondMarketItem {
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

  pub fn with_interest_late_amount(mut self, interest_late_amount: f64) -> SecondMarketItem {
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

  pub fn with_penalty_late_amount(mut self, penalty_late_amount: f64) -> SecondMarketItem {
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

  pub fn with_late_amount_total(mut self, late_amount_total: f64) -> SecondMarketItem {
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

  pub fn with_principal_write_off_amount(mut self, principal_write_off_amount: f64) -> SecondMarketItem {
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

  pub fn with_interest_write_off_amount(mut self, interest_write_off_amount: f64) -> SecondMarketItem {
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

  pub fn with_penalty_write_off_amount(mut self, penalty_write_off_amount: f64) -> SecondMarketItem {
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

  pub fn with_debt_servicing_cost_main_amount(mut self, debt_servicing_cost_main_amount: f64) -> SecondMarketItem {
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

  pub fn with_debt_servicing_cost_interest_amount(mut self, debt_servicing_cost_interest_amount: f64) -> SecondMarketItem {
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

  pub fn with_debt_servicing_cost_penalty_amount(mut self, debt_servicing_cost_penalty_amount: f64) -> SecondMarketItem {
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



