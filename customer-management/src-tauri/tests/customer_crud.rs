use customer_management_lib::crm::{
    create_customer_for_session, get_customer_for_session, list_customers_for_session,
    update_customer_for_session, CreateCustomerInput, UpdateCustomerInput,
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

#[test]
fn test_create_customer() {
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
                name: "Test Customer".into(),
                phone: "555-0001".into(),
                company: "Test Corp".into(),
                email: Some("test@test.com".into()),
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        assert_eq!(customer.name, "Test Customer");
        assert_eq!(customer.company, "Test Corp");
        assert_eq!(customer.email.as_deref(), Some("test@test.com"));

        session.clear();
    });
}

#[test]
fn test_get_customer() {
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

        let created = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Get Test Customer".into(),
                phone: "555-0002".into(),
                company: "Get Test Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        let customer_id = created.id.clone();

        let fetched = get_customer_for_session(&pool, &session, &customer_id)
            .await
            .expect("customer should be fetched");

        assert_eq!(fetched.id, customer_id);
        assert_eq!(fetched.name, "Get Test Customer");

        session.clear();
    });
}

#[test]
fn test_update_customer() {
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

        let created = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Original Name".into(),
                phone: "555-0003".into(),
                company: "Original Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        let customer_id = created.id.clone();

        let updated = update_customer_for_session(
            &pool,
            &session,
            &customer_id,
            UpdateCustomerInput {
                name: Some("Updated Name".into()),
                phone: None,
                company: None,
                email: Some("updated@test.com".into()),
                notes: None,
                address: None,
                status: Some(CustomerStatus::Silent),
            },
        )
        .await
        .expect("customer should be updated");

        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.email.as_deref(), Some("updated@test.com"));
        assert_eq!(updated.status, CustomerStatus::Silent);

        session.clear();
    });
}

#[test]
fn test_list_customers() {
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

        let customers = list_customers_for_session(&pool, &session, None, None, None)
            .await
            .expect("customers should be listed");

        assert!(!customers.is_empty());

        session.clear();
    });
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
fn test_non_owner_cannot_get_customer() {
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

        let created = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Alice Customer".into(),
                phone: "555-0004".into(),
                company: "Alice Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        let customer_id = created.id.clone();

        session.clear();

        let _bob = customer_management_lib::auth::authenticate_record(
            &seeded_other_user_credentials(),
            "bob123",
            &session,
        )
        .expect("seeded bob should authenticate");

        let result = get_customer_for_session(&pool, &session, &customer_id).await;

        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.to_string().contains("only owner or admin can access"));

        session.clear();
    });
}

#[test]
fn test_non_owner_cannot_update_customer() {
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

        let created = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Alice Customer".into(),
                phone: "555-0005".into(),
                company: "Alice Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        let customer_id = created.id.clone();

        session.clear();

        let _bob = customer_management_lib::auth::authenticate_record(
            &seeded_other_user_credentials(),
            "bob123",
            &session,
        )
        .expect("seeded bob should authenticate");

        let result = update_customer_for_session(
            &pool,
            &session,
            &customer_id,
            UpdateCustomerInput {
                name: Some("Hacked Name".into()),
                phone: None,
                company: None,
                email: None,
                notes: None,
                address: None,
                status: None,
            },
        )
        .await;

        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(error.to_string().contains("only owner or admin can access"));

        session.clear();
    });
}

#[test]
fn test_admin_can_get_any_customer() {
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

        let created = create_customer_for_session(
            &pool,
            &session,
            CreateCustomerInput {
                name: "Alice Customer".into(),
                phone: "555-0006".into(),
                company: "Alice Corp".into(),
                email: None,
                notes: None,
                address: None,
                status: CustomerStatus::Active,
            },
        )
        .await
        .expect("customer should be created");

        let customer_id = created.id.clone();

        session.clear();

        let _admin = customer_management_lib::auth::authenticate_record(
            &seeded_admin_credentials(),
            "admin123",
            &session,
        )
        .expect("seeded admin should authenticate");

        let result = get_customer_for_session(&pool, &session, &customer_id).await;

        assert!(result.is_ok());

        session.clear();
    });
}