use customer_management_lib::auth::{
    authenticate_record, sign_in_with_password, verify_password_hash, AuthenticatedUser,
    SessionState, StoredUserCredentials,
};
use customer_management_lib::storage::{
    connect_pool, migrate_database, DatabaseConfig,
};

fn seeded_admin_credentials() -> StoredUserCredentials {
    StoredUserCredentials {
        id: "user-admin".into(),
        username: "admin".into(),
        password_hash: "pbkdf2_sha256$100000$seed-admin-salt$763c1c0d50e9f82b2c3540702bc019df7137c0447de9763e7dc60c4151dfde64".into(),
        is_admin: true,
    }
}

#[test]
fn seeded_password_hash_accepts_valid_password() {
    let valid = verify_password_hash(&seeded_admin_credentials().password_hash, "admin123")
        .expect("hash should parse");

    assert!(valid);
}

#[test]
fn seeded_password_hash_rejects_wrong_password() {
    let valid = verify_password_hash(&seeded_admin_credentials().password_hash, "wrong-password")
        .expect("hash should parse");

    assert!(!valid);
}

#[test]
fn session_state_tracks_current_user() {
    let session = SessionState::default();
    let user = AuthenticatedUser {
        id: "user-admin".into(),
        username: "admin".into(),
        is_admin: true,
    };

    session.set_current_user(user.clone());
    assert_eq!(session.current_user(), Some(user));

    session.clear();
    assert_eq!(session.current_user(), None);
}

#[test]
fn authenticate_record_sets_current_user_session() {
    let session = SessionState::default();
    let authenticated = authenticate_record(&seeded_admin_credentials(), "admin123", &session)
        .expect("credentials should authenticate");

    assert_eq!(authenticated.username, "admin");
    assert_eq!(session.current_user(), Some(authenticated));
}

#[test]
fn authenticate_record_rejects_wrong_password_with_generic_error() {
    let session = SessionState::default();
    let error = authenticate_record(&seeded_admin_credentials(), "wrong-password", &session)
        .expect_err("wrong password should fail");

    assert_eq!(error.to_string(), "invalid username or password");
    assert_eq!(session.current_user(), None);
}

#[test]
fn authenticate_record_rejects_malformed_hash_with_generic_error() {
    let session = SessionState::default();
    let malformed = StoredUserCredentials {
        password_hash: "not-a-valid-hash".into(),
        ..seeded_admin_credentials()
    };

    let error = authenticate_record(&malformed, "admin123", &session)
        .expect_err("malformed hash should fail");

    assert_eq!(error.to_string(), "invalid username or password");
    assert_eq!(session.current_user(), None);
}

#[test]
fn postgres_sign_in_smoke_test_runs_when_test_database_url_is_present() {
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
        let authenticated = sign_in_with_password(&pool, &session, "admin", "admin123")
            .await
            .expect("seeded admin should sign in");

        assert_eq!(authenticated.username, "admin");
        assert_eq!(session.current_user(), Some(authenticated));
    });
}
