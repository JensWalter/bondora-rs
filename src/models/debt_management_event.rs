/// DebtManagementEvent : Information about the Loan's debt management stage event history
#[derive(Debug, Serialize, Deserialize)]
pub struct DebtManagementEvent {
    /// Date of event
    #[serde(rename = "CreatedOn")]
    pub created_on: Option<String>,
    /// Type of event
    #[serde(rename = "EventType")]
    pub event_type: Option<i32>,
    /// Type as a description, obsolete: use EventType
    #[serde(rename = "Description")]
    pub description: Option<String>,
}
