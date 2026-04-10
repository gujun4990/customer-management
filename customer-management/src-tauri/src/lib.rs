pub mod auth;
pub mod crm;
pub mod domain;
pub mod storage;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = tauri::async_runtime::block_on(async {
        let config = storage::DatabaseConfig::from_env().expect("DATABASE_URL must be configured");
        let pool = storage::connect_pool(&config)
            .await
            .expect("database pool must connect");
        storage::migrate_database(&pool)
            .await
            .expect("database migrations must apply");

        auth::AppState {
            pool,
            session: auth::SessionState::default(),
        }
    });

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            auth::sign_in,
            auth::current_user,
            auth::sign_out,
            crm::list_leads,
            crm::list_customers,
            crm::create_lead,
            crm::create_customer,
            crm::get_lead,
            crm::get_customer,
            crm::update_lead,
            crm::update_customer,
            crm::reassign_lead_owner,
            crm::reassign_customer_owner,
            crm::create_lead_followup,
            crm::list_lead_followups,
            crm::create_customer_followup,
            crm::list_customer_followups,
            crm::convert_lead_to_customer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
