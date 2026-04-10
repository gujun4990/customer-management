use customer_management_lib::auth::{authenticate_record, AuthError, SessionState, StoredUserCredentials};
use customer_management_lib::crm::{ensure_authenticated_session, list_customers_for_session, list_leads_for_session, CrmError};
use customer_management_lib::storage::{connect_pool, migrate_database, DatabaseConfig};
use sqlx::postgres::PgPoolOptions;

fn seeded_admin_credentials() -> StoredUserCredentials {
    StoredUserCredentials {
        id: "user-admin".into(),
        username: "admin".into(),
        password_hash: "pbkdf2_sha256$100000$seed-admin-salt$763c1c0d50e9f82b2c3540702bc019df7137c0447de9763e7dc60c4151dfde64".into(),
        is_admin: true,
    }
}

#[test]
fn unauthenticated_crm_reads_are_rejected() {
    let session = SessionState::default();

    tauri::async_runtime::block_on(async move {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://crm:crm@localhost:5432/customer_management")
            .expect("lazy pool should construct");

        assert_eq!(
            ensure_authenticated_session(&session).expect_err("unauthenticated CRM access should fail"),
            AuthError::authentication_required()
        );
        assert_eq!(
            list_leads_for_session(&pool, &session, None, None, None)
                .await
                .expect_err("unauthenticated lead read should fail"),
            CrmError::authentication_required()
        );
        assert_eq!(
            list_customers_for_session(&pool, &session, None, None, None)
                .await
                .expect_err("unauthenticated customer read should fail"),
            CrmError::authentication_required()
        );
    });
}

#[test]
fn postgres_crm_reads_require_session_and_work_after_login() {
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

        let session = SessionState::default();

        assert_eq!(
            ensure_authenticated_session(&session)
                .expect_err("unauthenticated lead read should fail"),
            AuthError::authentication_required()
        );

        let authenticated = authenticate_record(&seeded_admin_credentials(), "admin123", &session)
            .expect("seeded admin should authenticate");
        assert_eq!(authenticated.username, "admin");

        let leads = list_leads_for_session(&pool, &session, None, None, None)
            .await
            .expect("authenticated lead read should succeed");
        let customers = list_customers_for_session(&pool, &session, None, None, None)
            .await
            .expect("authenticated customer read should succeed");

        assert!(leads.iter().any(|lead| lead.id == "lead-alice"));
        assert!(customers.iter().any(|customer| customer.id == "customer-carol"));

        session.clear();

        assert_eq!(
            list_customers_for_session(&pool, &session, None, None, None)
                .await
                .expect_err("signed-out customer read should fail"),
            CrmError::authentication_required()
        );
    });
}
