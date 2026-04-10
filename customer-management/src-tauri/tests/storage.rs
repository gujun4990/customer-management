use customer_management_lib::storage::DatabaseConfig;

#[test]
fn database_config_reads_postgres_environment() {
    std::env::set_var(
        "DATABASE_URL",
        "postgres://crm:crm@localhost:5432/customer_management",
    );
    std::env::remove_var("DATABASE_MAX_CONNECTIONS");

    let config = DatabaseConfig::from_env().expect("config should load from environment");

    assert_eq!(
        config.database_url,
        "postgres://crm:crm@localhost:5432/customer_management"
    );
    assert_eq!(config.max_connections, 5);
}

#[test]
fn schema_migration_defines_postgres_enums_and_tables() {
    let schema = include_str!("../migrations/0001_create_schema.sql");

    assert!(schema.contains("CREATE TYPE lead_status AS ENUM"));
    assert!(schema.contains("CREATE TYPE customer_status AS ENUM"));
    assert!(schema.contains("CREATE TABLE IF NOT EXISTS users"));
    assert!(schema.contains("CREATE TABLE IF NOT EXISTS leads"));
    assert!(schema.contains("CREATE TABLE IF NOT EXISTS customers"));
    assert!(schema.contains("CREATE TABLE IF NOT EXISTS follow_ups"));
    assert!(schema.contains("lead_id TEXT REFERENCES leads(id)"));
    assert!(schema.contains("customer_id TEXT REFERENCES customers(id)"));
    assert!(schema.contains("target_type = 'lead'"));
    assert!(schema.contains("target_type = 'customer'"));
}

#[test]
fn seed_migration_contains_admin_users_and_sample_crm_records() {
    let seed = include_str!("../migrations/0002_seed_data.sql");

    assert!(seed.contains("admin"));
    assert!(seed.contains("sales"));
    assert!(seed.contains("pbkdf2_sha256$100000$seed-admin-salt$"));
    assert!(seed.contains("pbkdf2_sha256$100000$seed-sales-salt$"));
    assert!(seed.contains("lead-alice"));
    assert!(seed.contains("customer-carol"));
    assert!(seed.contains("follow-up-lead-alice"));
}

#[test]
fn postgres_bootstrap_smoke_test_runs_when_test_database_url_is_present() {
    let test_database_url = match std::env::var("TEST_DATABASE_URL") {
        Ok(value) => value,
        Err(_) => return,
    };

    tauri::async_runtime::block_on(async move {
        let config = DatabaseConfig {
            database_url: test_database_url,
            max_connections: 1,
        };

        let pool = customer_management_lib::storage::connect_pool(&config)
            .await
            .expect("pool should connect");
        customer_management_lib::storage::migrate_database(&pool)
            .await
            .expect("migrations should apply");
        let summary = customer_management_lib::storage::fetch_seed_summary(&pool)
            .await
            .expect("seed summary should load");

        assert!(summary.users >= 2);
        assert!(summary.leads >= 2);
        assert!(summary.customers >= 1);
        assert!(summary.follow_ups >= 2);
    });
}
