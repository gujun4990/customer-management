use customer_management_lib::crm::{
    create_lead_for_session, reassign_lead_owner_for_session, CreateLeadInput,
};
use customer_management_lib::domain::LeadStatus;
use customer_management_lib::storage::{connect_pool, migrate_database, DatabaseConfig};

fn seeded_user_credentials() -> customer_management_lib::auth::StoredUserCredentials {
    customer_management_lib::auth::StoredUserCredentials {
        id: "user-alice".into(),
        username: "alice".into(),
        password_hash: "pbkdf2_sha256$100000$seed-alice-salt$5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8".into(),
        is_admin: false,
    }
}

fn seeded_admin_credentials() -> customer_management_lib::auth::StoredUserCredentials {
    customer_management_lib::auth::StoredUserCredentials {
        id: "user-admin".into(),
        username: "admin".into(),
        password_hash: "pbkdf2_sha256$100000$seed-admin-salt$763c1c0d50e9f82b2c3540702bc019df7137c0447de9763e7dc60c4151dfde64".into(),
        is_admin: true,
    }
}

fn seeded_other_user_credentials() -> customer_management_lib::auth::StoredUserCredentials {
    customer_management_lib::auth::StoredUserCredentials {
        id: "user-bob".into(),
        username: "bob".into(),
        password_hash: "pbkdf2_sha256$100000$seed-bob-salt$d033e22ae348aeb5660fc2140aec35850c4da997".into(),
        is_admin: false,
    }
}

#[test]
fn test_owner_can_reassign_their_lead() {
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
                name: "Test Company".into(),
                phone: "555-0001".into(),
                company: "Test Co".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("lead should be created");

        let lead_id = lead.id.clone();

        let bob_id = "user-bob".to_string();

        let reassigned = reassign_lead_owner_for_session(&pool, &session, &lead_id, &bob_id)
            .await
            .expect("owner should be able to reassign their lead");

        assert_eq!(reassigned.owner.id, bob_id);

        session.clear();
    });
}

#[test]
fn test_admin_can_reassign_any_lead() {
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
                name: "Admin Test Company".into(),
                phone: "555-0002".into(),
                company: "Admin Test Co".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("lead should be created");

        let lead_id = lead.id.clone();

        session.clear();

        let _admin = customer_management_lib::auth::authenticate_record(
            &seeded_admin_credentials(),
            "admin123",
            &session,
        )
        .expect("seeded admin should authenticate");

        let admin_id = "user-admin".to_string();

        let reassigned = reassign_lead_owner_for_session(&pool, &session, &lead_id, &admin_id)
            .await
            .expect("admin should be able to reassign any lead");

        assert_eq!(reassigned.owner.id, admin_id);

        session.clear();
    });
}

#[test]
fn test_non_owner_cannot_reassign() {
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
                name: "Non-owner Test Company".into(),
                phone: "555-0003".into(),
                company: "Non-owner Test Co".into(),
                email: None,
                notes: None,
                address: None,
                status: LeadStatus::New,
            },
        )
        .await
        .expect("lead should be created");

        let lead_id = lead.id.clone();

        session.clear();

        let _bob = customer_management_lib::auth::authenticate_record(
            &seeded_other_user_credentials(),
            "bob123",
            &session,
        )
        .expect("seeded bob should authenticate");

        let charlie_id = "user-charlie".to_string();

        let result = reassign_lead_owner_for_session(&pool, &session, &lead_id, &charlie_id)
            .await;

        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.to_string().contains("only owner or admin can reassign"));

        session.clear();
    });
}