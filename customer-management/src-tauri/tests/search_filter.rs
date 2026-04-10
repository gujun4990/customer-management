use customer_management_lib::crm::{
    create_customer_for_session, create_lead_for_session, list_customers_for_session,
    list_leads_for_session, CreateCustomerInput, CreateLeadInput,
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

fn seeded_other_user_credentials() -> customer_management_lib::auth::StoredUserCredentials {
    customer_management_lib::auth::StoredUserCredentials {
        id: "user-bob".into(),
        username: "bob".into(),
        password_hash: "pbkdf2_sha256$100000$seed-bob-salt$d3bf3e6ea06f66baf882c79abd47322ed4b928521ba771079b8d602a6b1a974a".into(),
        is_admin: false,
    }
}

#[test]
fn test_list_leads_filter_by_status() {
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

        let _lead1 = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Status Filter Test 1".into(),
                phone: "555-1001".into(),
                company: "Test Company A".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("lead1 should be created");

        let _lead2 = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Status Filter Test 2".into(),
                phone: "555-1002".into(),
                company: "Test Company B".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::Converted,
            },
        )
        .await
        .expect("lead2 should be created");

        let all_leads = list_leads_for_session(&pool, &session, None, None, None)
            .await
            .expect("should list all leads");
        assert!(all_leads.len() >= 2);

        let converted_leads = list_leads_for_session(&pool, &session, None, None, Some("converted"))
            .await
            .expect("should filter leads by converted status");
        assert_eq!(converted_leads.len(), 1);
        assert_eq!(converted_leads[0].status, LeadStatus::Converted);

        let new_leads = list_leads_for_session(&pool, &session, None, None, Some("new"))
            .await
            .expect("should filter leads by new status");
        assert_eq!(new_leads.len(), 1);
        assert_eq!(new_leads[0].status, LeadStatus::New);

        session.clear();
    });
}

#[test]
fn test_list_leads_search() {
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

        let _lead1 = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Search Alpha Corp".into(),
                phone: "555-2001".into(),
                company: "Alpha Inc".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("lead1 should be created");

        let _lead2 = create_lead_for_session(
            &pool,
            &session,
            CreateLeadInput {
                name: "Search Beta Corp".into(),
                phone: "555-2002".into(),
                company: "Beta Ltd".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("lead2 should be created");

        let search_alpha = list_leads_for_session(&pool, &session, Some("Alpha"), None, None)
            .await
            .expect("should search leads by Alpha");
        assert_eq!(search_alpha.len(), 1);
        assert!(search_alpha[0].name.contains("Alpha") || search_alpha[0].company.contains("Alpha"));

        let search_corp = list_leads_for_session(&pool, &session, Some("Corp"), None, None)
            .await
            .expect("should search leads by Corp");
        assert_eq!(search_corp.len(), 2);

        let search_xyz = list_leads_for_session(&pool, &session, Some("XYZNotFound"), None, None)
            .await
            .expect("should search leads by XYZ");
        assert_eq!(search_xyz.len(), 0);

        session.clear();
    });
}

#[test]
fn test_list_leads_filter_by_owner() {
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

        let alice_session = customer_management_lib::auth::SessionState::default();
        let _alice = customer_management_lib::auth::authenticate_record(
            &seeded_user_credentials(),
            "alice123",
            &alice_session,
        )
        .expect("seeded alice should authenticate");

        let bob_session = customer_management_lib::auth::SessionState::default();
        let _bob = customer_management_lib::auth::authenticate_record(
            &seeded_other_user_credentials(),
            "bob123",
            &bob_session,
        )
        .expect("seeded bob should authenticate");

        let alice_lead = create_lead_for_session(
            &pool,
            &alice_session,
            CreateLeadInput {
                name: "Alice's Lead".into(),
                phone: "555-3001".into(),
                company: "Alice Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("alice lead should be created");

        let _bob_lead = create_lead_for_session(
            &pool,
            &bob_session,
            CreateLeadInput {
                name: "Bob's Lead".into(),
                phone: "555-3002".into(),
                company: "Bob Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("bob lead should be created");

        let alice_id = "user-alice".to_string();
        let alice_leads = list_leads_for_session(&pool, &alice_session, None, Some(&alice_id), None)
            .await
            .expect("should filter leads by alice owner");
        assert!(alice_leads.iter().all(|l| l.owner.id == "user-alice"));

        let bob_id = "user-bob".to_string();
        let bob_leads = list_leads_for_session(&pool, &bob_session, None, Some(&bob_id), None)
            .await
            .expect("should filter leads by bob owner");
        assert!(bob_leads.iter().all(|l| l.owner.id == "user-bob"));

        alice_session.clear();
        bob_session.clear();
    });
}

#[test]
fn test_list_customers_filter_by_status() {
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

        let _customer1 = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Active Customer".into(),
                phone: "555-4001".into(),
                company: "Active Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer1 should be created");

        let _customer2 = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Lost Customer".into(),
                phone: "555-4002".into(),
                company: "Lost Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Lost,
            },
        )
        .await
        .expect("customer2 should be created");

        let active_customers =
            list_customers_for_session(&pool, &session, None, None, Some("active"))
                .await
                .expect("should filter customers by active status");
        assert_eq!(active_customers.len(), 1);
        assert_eq!(active_customers[0].status, CustomerStatus::Active);

        let lost_customers = list_customers_for_session(&pool, &session, None, None, Some("lost"))
            .await
            .expect("should filter customers by lost status");
        assert_eq!(lost_customers.len(), 1);
        assert_eq!(lost_customers[0].status, CustomerStatus::Lost);

        session.clear();
    });
}

#[test]
fn test_list_customers_search() {
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

        let _customer1 = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Search Gamma Ltd".into(),
                phone: "555-5001".into(),
                company: "Gamma Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer1 should be created");

        let _customer2 = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Search Delta Ltd".into(),
                phone: "555-5002".into(),
                company: "Delta Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer2 should be created");

        let search_gamma = list_customers_for_session(&pool, &session, Some("Gamma"), None, None)
            .await
            .expect("should search customers by Gamma");
        assert_eq!(search_gamma.len(), 1);

        let search_ltd = list_customers_for_session(&pool, &session, Some("Ltd"), None, None)
            .await
            .expect("should search customers by Ltd");
        assert_eq!(search_ltd.len(), 2);

        session.clear();
    });
}