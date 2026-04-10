use customer_management_lib::crm::{
    convert_lead_to_customer_for_session, create_customer_followup_for_session,
    create_lead_followup_for_session, create_lead_for_session, CreateLeadInput,
};
use customer_management_lib::domain::LeadStatus;
use customer_management_lib::storage::{connect_pool, migrate_database, DatabaseConfig};

fn seeded_admin_credentials() -> customer_management_lib::auth::StoredUserCredentials {
    customer_management_lib::auth::StoredUserCredentials {
        id: "user-admin".into(),
        username: "admin".into(),
        password_hash: "pbkdf2_sha256$100000$seed-admin-salt$763c1c0d50e9f82b2c3540702bc019df7137c0447de9763e7dc60c4151dfde64".into(),
        is_admin: true,
    }
}

#[test]
fn test_full_business_flow() {
    let test_database_url = match std::env::var("TEST_DATABASE_URL") {
        Ok(value) => value,
        Err(_) => return,
    };

    tauri::async_runtime::block_on(async move {
        let config = DatabaseConfig {
            database_url: test_database_url,
            max_connections: 1,
        };

        let pool = connect_pool(&config).await.expect("pool should connect");
        migrate_database(&pool).await.expect("migrations should apply");

        let session = customer_management_lib::auth::SessionState::default();

        // Step 1: Login
        let _user = customer_management_lib::auth::authenticate_record(
            &seeded_admin_credentials(),
            "admin123",
            &session,
        )
        .expect("admin should authenticate");
        assert!(session.current_user().is_some());

        // Step 2: Create lead
        let lead = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "E2E Test Company".into(),
                phone: "555-E2E-001".into(),
                company: "E2E Corp".into(),
                email: Some("contact@e2e.com".into()),
                notes: Some("Business flow test lead".into()),
                address: Some("123 E2E Street".into()),
                status: LeadStatus::New,
            },
        )
        .await
        .expect("lead should be created");
        let lead_id = lead.id.clone();
        assert_eq!(lead.owner.username, "admin");

        // Step 3: Add follow-up to lead
        let lead_follow_up = create_lead_followup_for_session(
            &pool,
            &session,
            &lead_id,
            "Initial contact - interested in product",
            "2025-01-15T10:00:00Z",
            "phone",
            "positive response",
            Some("2025-01-22T10:00:00Z"),
        )
        .await
        .expect("lead follow-up should be created");
        assert!(matches!(
            lead_follow_up.target,
            customer_management_lib::domain::FollowUpTarget::Lead(id) if id == lead_id
        ));

        // Step 4: Convert lead to customer
        let customer = convert_lead_to_customer_for_session(
            &pool,
            &session,
            &lead_id,
            "active",
        )
        .await
        .expect("lead should be converted to customer");
        assert_eq!(customer.name, "E2E Test Company");
        assert_eq!(customer.company, "E2E Corp");
        assert_eq!(customer.status, customer_management_lib::domain::CustomerStatus::Active);

        let customer_id = customer.id.clone();

        // Step 5: Continue follow-up on customer
        let customer_follow_up = create_customer_followup_for_session(
            &pool,
            &session,
            &customer_id,
            "Follow-up after conversion - sent proposal",
            "2025-01-25T14:00:00Z",
            "email",
            "proposal sent",
            Some("2025-02-01T10:00:00Z"),
        )
        .await
        .expect("customer follow-up should be created");
        assert!(matches!(
            customer_follow_up.target,
            customer_management_lib::domain::FollowUpTarget::Customer(id) if id == customer_id
        ));

        // Verify complete flow
        assert_eq!(lead_follow_up.content, "Initial contact - interested in product");
        assert_eq!(customer_follow_up.content, "Follow-up after conversion - sent proposal");

        session.clear();
        assert!(session.current_user().is_none());

        println!("Full business flow completed successfully!");
    });
}