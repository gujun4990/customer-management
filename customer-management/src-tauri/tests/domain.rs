use customer_management_lib::domain::{
    CustomerRecord, CustomerStatus, FollowUpRecord, FollowUpTarget, LeadRecord, LeadStatus,
    UserRecord,
};
use serde_json::json;

#[test]
fn lead_status_values_are_fixed() {
    assert_eq!(
        LeadStatus::all(),
        vec![
            LeadStatus::New,
            LeadStatus::Assigned,
            LeadStatus::InProgress,
            LeadStatus::Converted,
            LeadStatus::Closed,
        ]
    );
}

#[test]
fn customer_status_values_are_fixed() {
    assert_eq!(
        CustomerStatus::all(),
        vec![
            CustomerStatus::Active,
            CustomerStatus::Silent,
            CustomerStatus::Lost,
        ]
    );
}

#[test]
fn lead_record_keeps_required_and_optional_fields() {
    let owner = UserRecord::new("user-1", "ops-admin", true);
    let lead = LeadRecord::new(
        "lead-1",
        "Alice Zhang",
        "13800138000",
        "Acme Co",
        Some("alice@example.com"),
        Some("Important account"),
        Some("Shanghai"),
        owner.clone(),
        LeadStatus::New,
    );

    assert_eq!(lead.id, "lead-1");
    assert_eq!(lead.name, "Alice Zhang");
    assert_eq!(lead.phone, "13800138000");
    assert_eq!(lead.company, "Acme Co");
    assert_eq!(lead.email.as_deref(), Some("alice@example.com"));
    assert_eq!(lead.notes.as_deref(), Some("Important account"));
    assert_eq!(lead.address.as_deref(), Some("Shanghai"));
    assert_eq!(lead.owner.id, "user-1");
    assert_eq!(lead.status, LeadStatus::New);
}

#[test]
fn customer_record_keeps_required_and_optional_fields() {
    let owner = UserRecord::new("user-2", "sales-admin", false);
    let customer = CustomerRecord::new(
        "customer-1",
        "Bob Li",
        "13900139000",
        "Beta Ltd",
        None,
        Some("Long-term customer"),
        None,
        owner.clone(),
        CustomerStatus::Active,
    );

    assert_eq!(customer.id, "customer-1");
    assert_eq!(customer.name, "Bob Li");
    assert_eq!(customer.phone, "13900139000");
    assert_eq!(customer.company, "Beta Ltd");
    assert_eq!(customer.email, None);
    assert_eq!(customer.notes.as_deref(), Some("Long-term customer"));
    assert_eq!(customer.address, None);
    assert_eq!(customer.owner.id, "user-2");
    assert_eq!(customer.status, CustomerStatus::Active);
}

#[test]
fn follow_up_records_keep_target_identity() {
    let lead_follow_up = FollowUpRecord::new_for_lead(
        "follow-up-1",
        "lead-1",
        "Called and confirmed interest",
        "2026-04-10T10:00:00Z",
        "phone",
        "interested",
        Some("2026-04-12T10:00:00Z"),
    );

    let customer_follow_up = FollowUpRecord::new_for_customer(
        "follow-up-2",
        "customer-1",
        "Sent onboarding notes",
        "2026-04-10T11:00:00Z",
        "email",
        "delivered",
        None,
    );

    assert_eq!(lead_follow_up.target, FollowUpTarget::Lead("lead-1".into()));
    assert_eq!(lead_follow_up.content, "Called and confirmed interest");
    assert_eq!(lead_follow_up.follow_up_time, "2026-04-10T10:00:00Z");
    assert_eq!(lead_follow_up.method, "phone");
    assert_eq!(lead_follow_up.result, "interested");
    assert_eq!(
        lead_follow_up.next_follow_up_time.as_deref(),
        Some("2026-04-12T10:00:00Z")
    );
    assert_eq!(
        customer_follow_up.target,
        FollowUpTarget::Customer("customer-1".into())
    );
    assert_eq!(customer_follow_up.content, "Sent onboarding notes");
    assert_eq!(customer_follow_up.follow_up_time, "2026-04-10T11:00:00Z");
    assert_eq!(customer_follow_up.method, "email");
    assert_eq!(customer_follow_up.result, "delivered");
    assert_eq!(customer_follow_up.next_follow_up_time, None);
}

#[test]
fn statuses_serialize_to_expected_wire_values() {
    assert_eq!(serde_json::to_value(LeadStatus::New).unwrap(), json!("new"));
    assert_eq!(
        serde_json::to_value(LeadStatus::Assigned).unwrap(),
        json!("assigned")
    );
    assert_eq!(
        serde_json::to_value(LeadStatus::InProgress).unwrap(),
        json!("in_progress")
    );
    assert_eq!(
        serde_json::to_value(LeadStatus::Converted).unwrap(),
        json!("converted")
    );
    assert_eq!(
        serde_json::to_value(LeadStatus::Closed).unwrap(),
        json!("closed")
    );
    assert_eq!(
        serde_json::to_value(CustomerStatus::Active).unwrap(),
        json!("active")
    );
    assert_eq!(
        serde_json::to_value(CustomerStatus::Silent).unwrap(),
        json!("silent")
    );
    assert_eq!(
        serde_json::to_value(CustomerStatus::Lost).unwrap(),
        json!("lost")
    );
}
