use customer_management_lib::crm::{
    convert_lead_to_customer_for_session, create_lead_for_session, get_lead_for_session,
    list_leads_for_session, CreateLeadInput,
};
use customer_management_lib::domain::{CustomerStatus, LeadStatus};
use customer_management_lib::storage::{connect_pool, migrate_database, DatabaseConfig};

fn seeded_user_credentials() -> customer_management_lib::auth::StoredUserCredentials {
    customer_management_lib::auth::StoredUserCredentials {
        id: "user-alice".into(),
        username: "alice".into(),
        password_hash: "pbkdf2_sha256$100000$seed-alice-salt$230ff9a22428019bcb02da4f93400a54c327fa51c672965ff2da262a9185f4d7".into(),
        is_admin: false,
    }
}

#[test]
fn test_convert_lead_to_customer() {
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

        let _alice = customer_management_lib::auth::authenticate_record(
            &seeded_user_credentials(),
            "alice123",
            &session,
        )
        .expect("seeded alice should authenticate");

        let lead = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Conversion Test Lead".into(),
                phone: "555-CONVERT".into(),
                company: "Convert Corp".into(),
                email: Some("convert@test.com".into()),
                notes: Some("Convert this lead".into()),
                address: Some("123 Conversion St".into()),
                status: LeadStatus::New,
            },
        )
        .await
        .expect("lead should be created");

        let lead_id = lead.id.clone();

        let customer = convert_lead_to_customer_for_session(
            &pool,
            &session,
            &lead_id,
            "active",
        )
        .await
        .expect("lead should be converted to customer");

        assert_eq!(customer.name, "Conversion Test Lead");
        assert_eq!(customer.phone, "555-CONVERT");
        assert_eq!(customer.company, "Convert Corp");
        assert_eq!(customer.email, Some("convert@test.com".to_string()));
        assert_eq!(customer.notes, Some("Convert this lead".to_string()));
        assert_eq!(customer.address, Some("123 Conversion St".to_string()));
        assert_eq!(customer.status, CustomerStatus::Active);
        assert_eq!(customer.owner.id, "user-alice");

        let converted_lead = get_lead_for_session(&pool, &session, &lead_id)
            .await
            .expect("lead should still exist");
        assert_eq!(converted_lead.status, LeadStatus::Converted);

        session.clear();
    });
}

#[test]
fn test_converted_lead_excluded_from_active_list() {
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

        let _alice = customer_management_lib::auth::authenticate_record(
            &seeded_user_credentials(),
            "alice123",
            &session,
        )
        .expect("seeded alice should authenticate");

        let new_lead = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Active Lead".into(),
                phone: "555-ACTIVE".into(),
                company: "Active Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("new lead should be created");

        let _converted_lead = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Converted Lead".into(),
                phone: "555-CONV".into(),
                company: "Conv Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::InProgress,
            },
        )
        .await
        .expect("lead should be created");

        let lead_id = _converted_lead.id.clone();

        convert_lead_to_customer_for_session(
            &pool,
            &session,
            &lead_id,
            "active",
        )
        .await
        .expect("lead should be converted");

        let active_leads = list_leads_for_session(&pool, &session, None, None, None)
            .await
            .expect("should list leads");

        assert!(
            active_leads.iter().all(|l| l.name != "Converted Lead"),
            "converted lead should not appear in active list"
        );
        assert!(
            active_leads.iter().any(|l| l.name == "Active Lead"),
            "non-converted lead should still appear"
        );

        session.clear();
    });
}

#[test]
fn test_converted_lead_available_with_status_filter() {
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

        let _alice = customer_management_lib::auth::authenticate_record(
            &seeded_user_credentials(),
            "alice123",
            &session,
        )
        .expect("seeded alice should authenticate");

        let lead = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Filter Converted Lead".into(),
                phone: "555-FILTER".into(),
                company: "Filter Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::Assigned,
            },
        )
        .await
        .expect("lead should be created");

        let lead_id = lead.id.clone();

        convert_lead_to_customer_for_session(
            &pool,
            &session,
            &lead_id,
            "active",
        )
        .await
        .expect("lead should be converted");

        let converted_leads = list_leads_for_session(&pool, &session, None, None, Some("converted"))
            .await
            .expect("should filter leads by converted status");

        assert!(
            converted_leads.iter().any(|l| l.name == "Filter Converted Lead"),
            "converted lead should appear when filtered by converted status"
        );

        session.clear();
    });
}

#[test]
fn test_non_convertible_lead_rejected() {
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

        let _alice = customer_management_lib::auth::authenticate_record(
            &seeded_user_credentials(),
            "alice123",
            &session,
        )
        .expect("seeded alice should authenticate");

        let lead = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Already Converted Lead".into(),
                phone: "555-ALREADY".into(),
                company: "Already Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::Converted,
            },
        )
        .await
        .expect("lead should be created");

        let lead_id = lead.id.clone();

        let result = convert_lead_to_customer_for_session(
            &pool,
            &session,
            &lead_id,
            "active",
        )
        .await;

        assert!(result.is_err(), "already converted lead should not be convertible");

        let closed_lead = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Closed Lead".into(),
                phone: "555-CLOSED".into(),
                company: "Closed Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::Closed,
            },
        )
        .await
        .expect("closed lead should be created");

        let closed_lead_id = closed_lead.id.clone();

        let closed_result = convert_lead_to_customer_for_session(
            &pool,
            &session,
            &closed_lead_id,
            "active",
        )
        .await;

        assert!(closed_result.is_err(), "closed lead should not be convertible");

        session.clear();
    });
}