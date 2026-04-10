use customer_management_lib::crm::{
    create_customer_for_session, create_customer_followup_for_session,
    list_customer_followups_for_session, CreateCustomerInput,
};
use customer_management_lib::domain::CustomerStatus;
use customer_management_lib::storage::{connect_pool, migrate_database, DatabaseConfig};

fn seeded_user_credentials() -> customer_management_lib::auth::StoredUserCredentials {
    customer_management_lib::auth::StoredUserCredentials {
        id: "user-alice".into(),
        username: "alice".into(),
        password_hash: "pbkdf2_sha256$100000$seed-alice-salt$5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8".into(),
        is_admin: false,
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
fn test_create_customer_followup() {
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

        let customer = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Customer FollowUp Test".into(),
                phone: "555-0001".into(),
                company: "FollowUp Test Co".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        let customer_id = customer.id.clone();
        let follow_up = create_customer_followup_for_session(
            &pool,
            &session,
            &customer_id,
            "Initial contact made",
            "2025-01-15T10:00:00Z",
            "phone",
            "left voicemail",
            Some("2025-01-20T10:00:00Z"),
        )
        .await
        .expect("follow-up should be created");

        assert!(matches!(
            &follow_up.target,
            customer_management_lib::domain::FollowUpTarget::Customer(id) if *id == customer_id
        ));
        assert_eq!(follow_up.content, "Initial contact made");
        assert_eq!(follow_up.follow_up_time, "2025-01-15T10:00:00Z");
        assert_eq!(follow_up.method, "phone");
        assert_eq!(follow_up.result, "left voicemail");
        assert_eq!(
            follow_up.next_follow_up_time,
            Some("2025-01-20T10:00:00Z".to_string())
        );

        session.clear();
    });
}

#[test]
fn test_list_customer_followups_sorted_newest_first() {
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

        let customer = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Customer Sort Test".into(),
                phone: "555-0002".into(),
                company: "Sort Test Co".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        let customer_id = customer.id.clone();

        create_customer_followup_for_session(
            &pool,
            &session,
            &customer_id,
            "First follow-up",
            "2025-01-10T10:00:00Z",
            "phone",
            "no answer",
            None,
        )
        .await
        .expect("first follow-up should be created");

        create_customer_followup_for_session(
            &pool,
            &session,
            &customer_id,
            "Second follow-up",
            "2025-01-15T10:00:00Z",
            "email",
            "sent proposal",
            None,
        )
        .await
        .expect("second follow-up should be created");

        create_customer_followup_for_session(
            &pool,
            &session,
            &customer_id,
            "Third follow-up",
            "2025-01-20T10:00:00Z",
            "phone",
            "meeting scheduled",
            None,
        )
        .await
        .expect("third follow-up should be created");

        let follow_ups =
            list_customer_followups_for_session(&pool, &session, &customer_id)
                .await
                .expect("should list follow-ups");

        assert_eq!(follow_ups.len(), 3);
        assert_eq!(follow_ups[0].content, "Third follow-up");
        assert_eq!(follow_ups[0].follow_up_time, "2025-01-20T10:00:00Z");
        assert_eq!(follow_ups[1].content, "Second follow-up");
        assert_eq!(follow_ups[1].follow_up_time, "2025-01-15T10:00:00Z");
        assert_eq!(follow_ups[2].content, "First follow-up");
        assert_eq!(follow_ups[2].follow_up_time, "2025-01-10T10:00:00Z");

        session.clear();
    });
}

#[test]
fn test_non_owner_cannot_create_customer_followup() {
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

        let customer = create_customer_for_session(
            &pool,
            &alice_session,
            CreateCustomerInput {
                name: "Alice's Customer".into(),
                phone: "555-0003".into(),
                company: "Alice Co".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        let customer_id = customer.id.clone();

        let bob_session = customer_management_lib::auth::SessionState::default();
        let _bob = customer_management_lib::auth::authenticate_record(
            &seeded_other_user_credentials(),
            "bob123",
            &bob_session,
        )
        .expect("seeded bob should authenticate");

        let result = create_customer_followup_for_session(
            &pool,
            &bob_session,
            &customer_id,
            "Trying to add follow-up",
            "2025-01-15T10:00:00Z",
            "phone",
            "test",
            None,
        )
        .await;

        assert!(result.is_err(), "non-owner should not be able to create follow-up");

        alice_session.clear();
        bob_session.clear();
    });
}