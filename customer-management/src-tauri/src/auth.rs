use std::sync::Mutex;

use pbkdf2::pbkdf2_hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredUserCredentials {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}

#[derive(Debug, Default)]
pub struct SessionState {
    current_user: Mutex<Option<AuthenticatedUser>>,
}

impl SessionState {
    pub fn set_current_user(&self, user: AuthenticatedUser) {
        *self.current_user.lock().expect("session mutex poisoned") = Some(user);
    }

    pub fn current_user(&self) -> Option<AuthenticatedUser> {
        self.current_user
            .lock()
            .expect("session mutex poisoned")
            .clone()
    }

    pub fn clear(&self) {
        *self.current_user.lock().expect("session mutex poisoned") = None;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuthError {
    pub message: &'static str,
}

impl AuthError {
    pub fn authentication_required() -> Self {
        Self {
            message: "authentication required",
        }
    }

    fn invalid_credentials() -> Self {
        Self {
            message: "invalid username or password",
        }
    }

    fn invalid_hash_format() -> Self {
        Self {
            message: "invalid stored password hash format",
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuthError {}

#[derive(Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub session: SessionState,
}

#[derive(Debug, FromRow)]
struct UserCredentialsRow {
    id: String,
    username: String,
    password_hash: String,
    is_admin: bool,
}

fn to_authenticated_user(credentials: &StoredUserCredentials) -> AuthenticatedUser {
    AuthenticatedUser {
        id: credentials.id.clone(),
        username: credentials.username.clone(),
        is_admin: credentials.is_admin,
    }
}

pub fn verify_password_hash(stored_hash: &str, password: &str) -> Result<bool, AuthError> {
    let mut parts = stored_hash.split('$');
    let algorithm = parts.next();
    let iterations = parts.next();
    let salt = parts.next();
    let expected_hash = parts.next();

    if algorithm != Some("pbkdf2_sha256")
        || iterations.is_none()
        || salt.is_none()
        || expected_hash.is_none()
        || parts.next().is_some()
    {
        return Err(AuthError::invalid_hash_format());
    }

    let iterations = iterations
        .and_then(|value| value.parse::<u32>().ok())
        .ok_or_else(AuthError::invalid_hash_format)?;
    let salt = salt.ok_or_else(AuthError::invalid_hash_format)?;
    let expected = hex::decode(expected_hash.ok_or_else(AuthError::invalid_hash_format)?)
        .map_err(|_| AuthError::invalid_hash_format())?;

    let mut derived = vec![0_u8; expected.len()];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt.as_bytes(), iterations, &mut derived);

    Ok(derived == expected)
}

pub fn authenticate_record(
    credentials: &StoredUserCredentials,
    password: &str,
    session: &SessionState,
) -> Result<AuthenticatedUser, AuthError> {
    let is_valid = verify_password_hash(&credentials.password_hash, password)
        .map_err(|_| AuthError::invalid_credentials())?;

    if !is_valid {
        return Err(AuthError::invalid_credentials());
    }

    let authenticated = to_authenticated_user(credentials);
    session.set_current_user(authenticated.clone());

    Ok(authenticated)
}

pub async fn sign_in_with_password(
    pool: &PgPool,
    session: &SessionState,
    username: &str,
    password: &str,
) -> Result<AuthenticatedUser, AuthError> {
    let row = sqlx::query_as::<_, UserCredentialsRow>(
        "SELECT id, username, password_hash, is_admin FROM users WHERE username = $1",
    )
    .bind(username)
    .fetch_optional(pool)
    .await
    .map_err(|_| AuthError::invalid_credentials())?
    .ok_or_else(AuthError::invalid_credentials)?;

    let credentials = StoredUserCredentials {
        id: row.id,
        username: row.username,
        password_hash: row.password_hash,
        is_admin: row.is_admin,
    };

    authenticate_record(&credentials, password, session)
}

#[tauri::command]
pub async fn sign_in(
    username: String,
    password: String,
    state: tauri::State<'_, AppState>,
) -> Result<AuthenticatedUser, String> {
    sign_in_with_password(&state.pool, &state.session, &username, &password)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn current_user(state: tauri::State<'_, AppState>) -> Option<AuthenticatedUser> {
    state.session.current_user()
}

#[tauri::command]
pub fn sign_out(state: tauri::State<'_, AppState>) {
    state.session.clear();
}
