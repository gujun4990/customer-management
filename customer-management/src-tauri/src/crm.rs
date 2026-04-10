use sqlx::{FromRow, PgPool};

use crate::{
    auth::{AppState, AuthError, AuthenticatedUser, SessionState},
    domain::{CustomerRecord, CustomerStatus, FollowUpRecord, FollowUpTarget, LeadRecord, LeadStatus, UserRecord},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrmError {
    message: &'static str,
}

impl CrmError {
    pub fn authentication_required() -> Self {
        Self {
            message: "authentication required",
        }
    }

    fn data_unavailable() -> Self {
        Self {
            message: "crm data unavailable",
        }
    }

    pub fn unauthorized_reassignment() -> Self {
        Self {
            message: "only owner or admin can reassign",
        }
    }

    pub fn unauthorized_followup() -> Self {
        Self {
            message: "only owner or admin can create follow-up",
        }
    }

    pub fn unauthorized_access() -> Self {
        Self {
            message: "only owner or admin can access",
        }
    }
}

impl std::fmt::Display for CrmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CrmError {}

#[derive(Debug, FromRow)]
struct LeadRow {
    id: String,
    name: String,
    phone: String,
    company: String,
    email: Option<String>,
    notes: Option<String>,
    address: Option<String>,
    owner_id: String,
    owner_username: String,
    owner_is_admin: bool,
    status: String,
}

#[derive(Debug, FromRow)]
struct CustomerRow {
    id: String,
    name: String,
    phone: String,
    company: String,
    email: Option<String>,
    notes: Option<String>,
    address: Option<String>,
    owner_id: String,
    owner_username: String,
    owner_is_admin: bool,
    status: String,
}

#[derive(Debug, FromRow)]
struct FollowUpRow {
    id: String,
    _target_type: String,
    target_id: String,
    content: String,
    follow_up_time: String,
    method: String,
    result: String,
    next_follow_up_time: Option<String>,
}

pub fn ensure_authenticated_session(session: &SessionState) -> Result<AuthenticatedUser, AuthError> {
    session
        .current_user()
        .ok_or_else(AuthError::authentication_required)
}

fn parse_lead_status(value: &str) -> Result<LeadStatus, CrmError> {
    match value {
        "new" => Ok(LeadStatus::New),
        "assigned" => Ok(LeadStatus::Assigned),
        "in_progress" => Ok(LeadStatus::InProgress),
        "converted" => Ok(LeadStatus::Converted),
        "closed" => Ok(LeadStatus::Closed),
        _ => Err(CrmError::data_unavailable()),
    }
}

fn parse_customer_status(value: &str) -> Result<CustomerStatus, CrmError> {
    match value {
        "active" => Ok(CustomerStatus::Active),
        "silent" => Ok(CustomerStatus::Silent),
        "lost" => Ok(CustomerStatus::Lost),
        _ => Err(CrmError::data_unavailable()),
    }
}

pub async fn list_leads_for_session(
    pool: &PgPool,
    session: &SessionState,
    search: Option<&str>,
    owner_id: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<LeadRecord>, CrmError> {
    ensure_authenticated_session(session).map_err(|_| CrmError::authentication_required())?;

    let search_pattern = search.map(|s| format!("%{}%", s));

    let rows = sqlx::query_as::<_, LeadRow>(
        r#"
        SELECT
            leads.id,
            leads.name,
            leads.phone,
            leads.company,
            leads.email,
            leads.notes,
            leads.address,
            users.id AS owner_id,
            users.username AS owner_username,
            users.is_admin AS owner_is_admin,
            leads.status::text AS status
        FROM leads
        JOIN users ON users.id = leads.owner_id
        WHERE ($1::text IS NULL OR leads.name ILIKE $1 OR leads.company ILIKE $1 OR leads.email ILIKE $1 OR leads.phone ILIKE $1)
          AND ($2::text IS NULL OR users.id = $2)
          AND ($3::text IS NULL OR leads.status::text = $3)
        ORDER BY leads.id
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(owner_id)
    .bind(status)
    .fetch_all(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    rows.into_iter()
        .map(|row| {
            Ok(LeadRecord {
                id: row.id,
                name: row.name,
                phone: row.phone,
                company: row.company,
                email: row.email,
                notes: row.notes,
                address: row.address,
                owner: UserRecord {
                    id: row.owner_id,
                    username: row.owner_username,
                    is_admin: row.owner_is_admin,
                },
                status: parse_lead_status(&row.status)?,
            })
        })
        .collect()
}

pub async fn list_customers_for_session(
    pool: &PgPool,
    session: &SessionState,
    search: Option<&str>,
    owner_id: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<CustomerRecord>, CrmError> {
    ensure_authenticated_session(session).map_err(|_| CrmError::authentication_required())?;

    let search_pattern = search.map(|s| format!("%{}%", s));

    let rows = sqlx::query_as::<_, CustomerRow>(
        r#"
        SELECT
            customers.id,
            customers.name,
            customers.phone,
            customers.company,
            customers.email,
            customers.notes,
            customers.address,
            users.id AS owner_id,
            users.username AS owner_username,
            users.is_admin AS owner_is_admin,
            customers.status::text AS status
        FROM customers
        JOIN users ON users.id = customers.owner_id
        WHERE ($1::text IS NULL OR customers.name ILIKE $1 OR customers.company ILIKE $1 OR customers.email ILIKE $1 OR customers.phone ILIKE $1)
          AND ($2::text IS NULL OR users.id = $2)
          AND ($3::text IS NULL OR customers.status::text = $3)
        ORDER BY customers.id
        "#,
    )
    .bind(search_pattern.as_deref())
    .bind(owner_id)
    .bind(status)
    .fetch_all(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    rows.into_iter()
        .map(|row| {
            Ok(CustomerRecord {
                id: row.id,
                name: row.name,
                phone: row.phone,
                company: row.company,
                email: row.email,
                notes: row.notes,
                address: row.address,
                owner: UserRecord {
                    id: row.owner_id,
                    username: row.owner_username,
                    is_admin: row.owner_is_admin,
                },
                status: parse_customer_status(&row.status)?,
            })
        })
        .collect()
}

#[derive(Debug)]
pub struct CreateLeadInput {
    pub name: String,
    pub phone: String,
    pub company: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub status: LeadStatus,
}

#[derive(Debug)]
pub struct CreateCustomerInput {
    pub name: String,
    pub phone: String,
    pub company: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub status: CustomerStatus,
}

#[derive(Debug)]
pub struct UpdateLeadInput {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub status: Option<LeadStatus>,
}

#[derive(Debug)]
pub struct UpdateCustomerInput {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub status: Option<CustomerStatus>,
}

#[derive(Debug)]
pub struct ReassignLeadOwnerInput {
    pub new_owner_id: String,
}

#[derive(Debug)]
pub struct ReassignCustomerOwnerInput {
    pub new_owner_id: String,
}

pub async fn create_lead_for_session(
    pool: &PgPool,
    session: &SessionState,
    input: CreateLeadInput,
) -> Result<LeadRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let id = uuid::Uuid::new_v4().to_string();
    let owner_id = user.id.clone();
    let owner_username = user.username.clone();
    let owner_is_admin = user.is_admin;
    let status_str = format!("{:?}", input.status).to_lowercase();

    sqlx::query(
        r#"
        INSERT INTO leads (id, name, phone, company, email, notes, address, owner_id, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(&id)
    .bind(&input.name)
    .bind(&input.phone)
    .bind(&input.company)
    .bind(&input.email)
    .bind(&input.notes)
    .bind(&input.address)
    .bind(&owner_id)
    .bind(&status_str)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(LeadRecord {
        id,
        name: input.name,
        phone: input.phone,
        company: input.company,
        email: input.email,
        notes: input.notes,
        address: input.address,
        owner: UserRecord {
            id: owner_id,
            username: owner_username,
            is_admin: owner_is_admin,
        },
        status: input.status,
    })
}

pub async fn create_customer_for_session(
    pool: &PgPool,
    session: &SessionState,
    input: CreateCustomerInput,
) -> Result<CustomerRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let id = uuid::Uuid::new_v4().to_string();
    let owner_id = user.id.clone();
    let owner_username = user.username.clone();
    let owner_is_admin = user.is_admin;
    let status_str = format!("{:?}", input.status).to_lowercase();

    sqlx::query(
        r#"
        INSERT INTO customers (id, name, phone, company, email, notes, address, owner_id, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(&id)
    .bind(&input.name)
    .bind(&input.phone)
    .bind(&input.company)
    .bind(&input.email)
    .bind(&input.notes)
    .bind(&input.address)
    .bind(&owner_id)
    .bind(&status_str)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(CustomerRecord {
        id,
        name: input.name,
        phone: input.phone,
        company: input.company,
        email: input.email,
        notes: input.notes,
        address: input.address,
        owner: UserRecord {
            id: owner_id,
            username: owner_username,
            is_admin: owner_is_admin,
        },
        status: input.status,
    })
}

pub async fn get_lead_for_session(
    pool: &PgPool,
    session: &SessionState,
    lead_id: &str,
) -> Result<LeadRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let row = sqlx::query_as::<_, LeadRow>(
        r#"
        SELECT
            leads.id,
            leads.name,
            leads.phone,
            leads.company,
            leads.email,
            leads.notes,
            leads.address,
            users.id AS owner_id,
            users.username AS owner_username,
            users.is_admin AS owner_is_admin,
            leads.status::text AS status
        FROM leads
        JOIN users ON users.id = leads.owner_id
        WHERE leads.id = $1
        "#,
    )
    .bind(lead_id)
    .fetch_one(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    if row.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_access());
    }

    Ok(LeadRecord {
        id: row.id,
        name: row.name,
        phone: row.phone,
        company: row.company,
        email: row.email,
        notes: row.notes,
        address: row.address,
        owner: UserRecord {
            id: row.owner_id,
            username: row.owner_username,
            is_admin: row.owner_is_admin,
        },
        status: parse_lead_status(&row.status)?,
    })
}

pub async fn update_lead_for_session(
    pool: &PgPool,
    session: &SessionState,
    lead_id: &str,
    input: UpdateLeadInput,
) -> Result<LeadRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let existing = sqlx::query_as::<_, LeadRow>(
        r#"
        SELECT id, name, phone, company, email, notes, address, owner_id, owner_username, owner_is_admin, status::text AS status
        FROM leads
        JOIN users ON users.id = leads.owner_id
        WHERE leads.id = $1
        "#,
    )
    .bind(lead_id)
    .fetch_one(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    if existing.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_access());
    }

    let new_name = input.name.unwrap_or(existing.name);
    let new_phone = input.phone.unwrap_or(existing.phone);
    let new_company = input.company.unwrap_or(existing.company);
    let new_email = input.email.or(existing.email);
    let new_notes = input.notes.or(existing.notes);
    let new_address = input.address.or(existing.address);
    let new_status = input
        .status
        .map(|s| format!("{:?}", s).to_lowercase())
        .unwrap_or(existing.status);

    sqlx::query(
        r#"
        UPDATE leads
        SET name = $1, phone = $2, company = $3, email = $4, notes = $5, address = $6, status = $7
        WHERE id = $8
        "#,
    )
    .bind(&new_name)
    .bind(&new_phone)
    .bind(&new_company)
    .bind(&new_email)
    .bind(&new_notes)
    .bind(&new_address)
    .bind(&new_status)
    .bind(lead_id)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(LeadRecord {
        id: lead_id.into(),
        name: new_name,
        phone: new_phone,
        company: new_company,
        email: new_email,
        notes: new_notes,
        address: new_address,
        owner: UserRecord {
            id: existing.owner_id,
            username: existing.owner_username,
            is_admin: existing.owner_is_admin,
        },
        status: parse_lead_status(&new_status)?,
    })
}

pub async fn get_customer_for_session(
    pool: &PgPool,
    session: &SessionState,
    customer_id: &str,
) -> Result<CustomerRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let row = sqlx::query_as::<_, CustomerRow>(
        r#"
        SELECT
            customers.id,
            customers.name,
            customers.phone,
            customers.company,
            customers.email,
            customers.notes,
            customers.address,
            users.id AS owner_id,
            users.username AS owner_username,
            users.is_admin AS owner_is_admin,
            customers.status::text AS status
        FROM customers
        JOIN users ON users.id = customers.owner_id
        WHERE customers.id = $1
        "#,
    )
    .bind(customer_id)
    .fetch_one(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    if row.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_access());
    }

    Ok(CustomerRecord {
        id: row.id,
        name: row.name,
        phone: row.phone,
        company: row.company,
        email: row.email,
        notes: row.notes,
        address: row.address,
        owner: UserRecord {
            id: row.owner_id,
            username: row.owner_username,
            is_admin: row.owner_is_admin,
        },
        status: parse_customer_status(&row.status)?,
    })
}

pub async fn update_customer_for_session(
    pool: &PgPool,
    session: &SessionState,
    customer_id: &str,
    input: UpdateCustomerInput,
) -> Result<CustomerRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let existing = sqlx::query_as::<_, CustomerRow>(
        r#"
        SELECT id, name, phone, company, email, notes, address, owner_id, owner_username, owner_is_admin, status::text AS status
        FROM customers
        JOIN users ON users.id = customers.owner_id
        WHERE customers.id = $1
        "#,
    )
    .bind(customer_id)
    .fetch_one(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    if existing.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_access());
    }

    let new_name = input.name.unwrap_or(existing.name);
    let new_phone = input.phone.unwrap_or(existing.phone);
    let new_company = input.company.unwrap_or(existing.company);
    let new_email = input.email.or(existing.email);
    let new_notes = input.notes.or(existing.notes);
    let new_address = input.address.or(existing.address);
    let new_status = input
        .status
        .map(|s| format!("{:?}", s).to_lowercase())
        .unwrap_or(existing.status);

    sqlx::query(
        r#"
        UPDATE customers
        SET name = $1, phone = $2, company = $3, email = $4, notes = $5, address = $6, status = $7
        WHERE id = $8
        "#,
    )
    .bind(&new_name)
    .bind(&new_phone)
    .bind(&new_company)
    .bind(&new_email)
    .bind(&new_notes)
    .bind(&new_address)
    .bind(&new_status)
    .bind(customer_id)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(CustomerRecord {
        id: customer_id.into(),
        name: new_name,
        phone: new_phone,
        company: new_company,
        email: new_email,
        notes: new_notes,
        address: new_address,
        owner: UserRecord {
            id: existing.owner_id,
            username: existing.owner_username,
            is_admin: existing.owner_is_admin,
        },
        status: parse_customer_status(&new_status)?,
    })
}

pub async fn reassign_lead_owner_for_session(
    pool: &PgPool,
    session: &SessionState,
    lead_id: &str,
    new_owner_id: &str,
) -> Result<LeadRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let existing = sqlx::query_as::<_, LeadRow>(
        r#"
        SELECT id, name, phone, company, email, notes, address, owner_id, owner_username, owner_is_admin, status::text AS status
        FROM leads
        JOIN users ON users.id = leads.owner_id
        WHERE leads.id = $1
        "#,
    )
    .bind(lead_id)
    .fetch_one(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    if existing.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_reassignment());
    }

    let new_owner_row = sqlx::query_as::<_, (String, String, bool)>(
        "SELECT id, username, is_admin FROM users WHERE id = $1",
    )
    .bind(new_owner_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?
    .ok_or_else(CrmError::data_unavailable)?;

    let (owner_id, owner_username, owner_is_admin) = new_owner_row;

    sqlx::query(
        "UPDATE leads SET owner_id = $1 WHERE id = $2",
    )
    .bind(new_owner_id)
    .bind(lead_id)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(LeadRecord {
        id: lead_id.into(),
        name: existing.name,
        phone: existing.phone,
        company: existing.company,
        email: existing.email,
        notes: existing.notes,
        address: existing.address,
        owner: UserRecord {
            id: owner_id,
            username: owner_username,
            is_admin: owner_is_admin,
        },
        status: parse_lead_status(&existing.status)?,
    })
}

pub async fn reassign_customer_owner_for_session(
    pool: &PgPool,
    session: &SessionState,
    customer_id: &str,
    new_owner_id: &str,
) -> Result<CustomerRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let existing = sqlx::query_as::<_, CustomerRow>(
        r#"
        SELECT id, name, phone, company, email, notes, address, owner_id, owner_username, owner_is_admin, status::text AS status
        FROM customers
        JOIN users ON users.id = customers.owner_id
        WHERE customers.id = $1
        "#,
    )
    .bind(customer_id)
    .fetch_one(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    if existing.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_reassignment());
    }

    let new_owner_row = sqlx::query_as::<_, (String, String, bool)>(
        "SELECT id, username, is_admin FROM users WHERE id = $1",
    )
    .bind(new_owner_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?
    .ok_or_else(CrmError::data_unavailable)?;

    let (owner_id, owner_username, owner_is_admin) = new_owner_row;

    sqlx::query(
        "UPDATE customers SET owner_id = $1 WHERE id = $2",
    )
    .bind(new_owner_id)
    .bind(customer_id)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(CustomerRecord {
        id: customer_id.into(),
        name: existing.name,
        phone: existing.phone,
        company: existing.company,
        email: existing.email,
        notes: existing.notes,
        address: existing.address,
        owner: UserRecord {
            id: owner_id,
            username: owner_username,
            is_admin: owner_is_admin,
        },
        status: parse_customer_status(&existing.status)?,
    })
}

#[derive(Debug)]
pub struct CreateFollowUpInput {
    pub content: String,
    pub follow_up_time: String,
    pub method: String,
    pub result: String,
    pub next_follow_up_time: Option<String>,
}

pub async fn create_lead_followup_for_session(
    pool: &PgPool,
    session: &SessionState,
    lead_id: &str,
    content: &str,
    follow_up_time: &str,
    method: &str,
    result: &str,
    next_follow_up_time: Option<&str>,
) -> Result<FollowUpRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let lead = sqlx::query_as::<_, LeadRow>(
        r#"
        SELECT id, name, phone, company, email, notes, address, owner_id, owner_username, owner_is_admin, status::text AS status
        FROM leads
        JOIN users ON users.id = leads.owner_id
        WHERE leads.id = $1
        "#,
    )
    .bind(lead_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?
    .ok_or_else(CrmError::data_unavailable)?;

    if lead.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_followup());
    }

    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO follow_ups (id, target_type, lead_id, content, follow_up_time, method, result, next_follow_up_time)
        VALUES ($1, 'lead', $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&id)
    .bind(lead_id)
    .bind(content)
    .bind(follow_up_time)
    .bind(method)
    .bind(result)
    .bind(next_follow_up_time)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(FollowUpRecord::new_for_lead(
        &id,
        lead_id,
        content,
        follow_up_time,
        method,
        result,
        next_follow_up_time,
    ))
}

pub async fn list_lead_followups_for_session(
    pool: &PgPool,
    session: &SessionState,
    lead_id: &str,
) -> Result<Vec<FollowUpRecord>, CrmError> {
    ensure_authenticated_session(session).map_err(|_| CrmError::authentication_required())?;

    let rows = sqlx::query_as::<_, FollowUpRow>(
        r#"
        SELECT id, target_type, COALESCE(lead_id, customer_id) AS target_id, 
               content, follow_up_time, method, result, next_follow_up_time
        FROM follow_ups
        WHERE target_type = 'lead' AND lead_id = $1
        ORDER BY follow_up_time DESC
        "#,
    )
    .bind(lead_id)
    .fetch_all(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    rows.into_iter()
        .map(|row| {
            Ok(FollowUpRecord {
                id: row.id,
                target: FollowUpTarget::Lead(row.target_id),
                content: row.content,
                follow_up_time: row.follow_up_time,
                method: row.method,
                result: row.result,
                next_follow_up_time: row.next_follow_up_time,
            })
        })
        .collect()
}

pub async fn create_customer_followup_for_session(
    pool: &PgPool,
    session: &SessionState,
    customer_id: &str,
    content: &str,
    follow_up_time: &str,
    method: &str,
    result: &str,
    next_follow_up_time: Option<&str>,
) -> Result<FollowUpRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let customer = sqlx::query_as::<_, CustomerRow>(
        r#"
        SELECT id, name, phone, company, email, notes, address, owner_id, owner_username, owner_is_admin, status::text AS status
        FROM customers
        JOIN users ON users.id = customers.owner_id
        WHERE customers.id = $1
        "#,
    )
    .bind(customer_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?
    .ok_or_else(CrmError::data_unavailable)?;

    if customer.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_followup());
    }

    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        r#"
        INSERT INTO follow_ups (id, target_type, customer_id, content, follow_up_time, method, result, next_follow_up_time)
        VALUES ($1, 'customer', $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&id)
    .bind(customer_id)
    .bind(content)
    .bind(follow_up_time)
    .bind(method)
    .bind(result)
    .bind(next_follow_up_time)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(FollowUpRecord::new_for_customer(
        &id,
        customer_id,
        content,
        follow_up_time,
        method,
        result,
        next_follow_up_time,
    ))
}

pub async fn list_customer_followups_for_session(
    pool: &PgPool,
    session: &SessionState,
    customer_id: &str,
) -> Result<Vec<FollowUpRecord>, CrmError> {
    ensure_authenticated_session(session).map_err(|_| CrmError::authentication_required())?;

    let rows = sqlx::query_as::<_, FollowUpRow>(
        r#"
        SELECT id, target_type, COALESCE(lead_id, customer_id) AS target_id, 
               content, follow_up_time, method, result, next_follow_up_time
        FROM follow_ups
        WHERE target_type = 'customer' AND customer_id = $1
        ORDER BY follow_up_time DESC
        "#,
    )
    .bind(customer_id)
    .fetch_all(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    rows.into_iter()
        .map(|row| {
            Ok(FollowUpRecord {
                id: row.id,
                target: FollowUpTarget::Customer(row.target_id),
                content: row.content,
                follow_up_time: row.follow_up_time,
                method: row.method,
                result: row.result,
                next_follow_up_time: row.next_follow_up_time,
            })
        })
        .collect()
}

pub async fn convert_lead_to_customer_for_session(
    pool: &PgPool,
    session: &SessionState,
    lead_id: &str,
    initial_customer_status: &str,
) -> Result<CustomerRecord, CrmError> {
    let user = ensure_authenticated_session(session)
        .map_err(|_| CrmError::authentication_required())?;

    let lead = sqlx::query_as::<_, LeadRow>(
        r#"
        SELECT id, name, phone, company, email, notes, address, owner_id, owner_username, owner_is_admin, status::text AS status
        FROM leads
        JOIN users ON users.id = leads.owner_id
        WHERE leads.id = $1
        "#,
    )
    .bind(lead_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?
    .ok_or_else(CrmError::data_unavailable)?;

    if lead.owner_id != user.id && !user.is_admin {
        return Err(CrmError::unauthorized_followup());
    }

    let current_status = parse_lead_status(&lead.status)?;
    match current_status {
        LeadStatus::Converted | LeadStatus::Closed => {
            return Err(CrmError {
                message: "lead not convertible",
            });
        }
        _ => {}
    }

    let customer_status = parse_customer_status(initial_customer_status)?;
    let status_str = format!("{:?}", customer_status).to_lowercase();

    let customer_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO customers (id, name, phone, company, email, notes, address, owner_id, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(&customer_id)
    .bind(&lead.name)
    .bind(&lead.phone)
    .bind(&lead.company)
    .bind(&lead.email)
    .bind(&lead.notes)
    .bind(&lead.address)
    .bind(&lead.owner_id)
    .bind(&status_str)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    let follow_up_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        r#"
        INSERT INTO follow_ups (id, target_type, lead_id, customer_id, content, follow_up_time, method, result)
        VALUES ($1, 'conversion', $2, $3, $4, NOW(), $5, $6)
        "#,
    )
    .bind(&follow_up_id)
    .bind(lead_id)
    .bind(&customer_id)
    .bind("Lead converted to customer")
    .bind("system")
    .bind("converted")
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    sqlx::query(
        r#"
        UPDATE leads SET status = 'converted' WHERE id = $1
        "#,
    )
    .bind(lead_id)
    .execute(pool)
    .await
    .map_err(|_| CrmError::data_unavailable())?;

    Ok(CustomerRecord::new(
        &customer_id,
        &lead.name,
        &lead.phone,
        &lead.company,
        lead.email.as_deref(),
        lead.notes.as_deref(),
        lead.address.as_deref(),
        UserRecord {
            id: lead.owner_id.clone(),
            username: lead.owner_username.clone(),
            is_admin: lead.owner_is_admin,
        },
        customer_status,
    ))
}

#[tauri::command]
pub async fn create_lead(
    state: tauri::State<'_, AppState>,
    name: String,
    phone: String,
    company: String,
    email: Option<String>,
    notes: Option<String>,
    address: Option<String>,
    status: String,
) -> Result<LeadRecord, String> {
    let status = parse_lead_status(&status).map_err(|e| e.to_string())?;
    let input = CreateLeadInput {
        name,
        phone,
        company,
        email,
        notes,
        address,
        status,
    };
    create_lead_for_session(&state.pool, &state.session, input)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_lead(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<LeadRecord, String> {
    get_lead_for_session(&state.pool, &state.session, &id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn update_lead(
    state: tauri::State<'_, AppState>,
    id: String,
    name: Option<String>,
    phone: Option<String>,
    company: Option<String>,
    email: Option<String>,
    notes: Option<String>,
    address: Option<String>,
    status: Option<String>,
) -> Result<LeadRecord, String> {
    let status = match status {
        Some(s) => Some(parse_lead_status(&s).map_err(|e| e.to_string())?),
        None => None,
    };
    let input = UpdateLeadInput {
        name,
        phone,
        company,
        email,
        notes,
        address,
        status,
    };
    update_lead_for_session(&state.pool, &state.session, &id, input)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn list_leads(
    state: tauri::State<'_, AppState>,
    search: Option<String>,
    owner_id: Option<String>,
    status: Option<String>,
) -> Result<Vec<LeadRecord>, String> {
    list_leads_for_session(
        &state.pool,
        &state.session,
        search.as_deref(),
        owner_id.as_deref(),
        status.as_deref(),
    )
    .await
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn list_customers(
    state: tauri::State<'_, AppState>,
    search: Option<String>,
    owner_id: Option<String>,
    status: Option<String>,
) -> Result<Vec<CustomerRecord>, String> {
    list_customers_for_session(
        &state.pool,
        &state.session,
        search.as_deref(),
        owner_id.as_deref(),
        status.as_deref(),
    )
    .await
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn create_customer(
    state: tauri::State<'_, AppState>,
    name: String,
    phone: String,
    company: String,
    email: Option<String>,
    notes: Option<String>,
    address: Option<String>,
    status: String,
) -> Result<CustomerRecord, String> {
    let status = parse_customer_status(&status).map_err(|e| e.to_string())?;
    let input = CreateCustomerInput {
        name,
        phone,
        company,
        email,
        notes,
        address,
        status,
    };
    create_customer_for_session(&state.pool, &state.session, input)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn get_customer(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<CustomerRecord, String> {
    get_customer_for_session(&state.pool, &state.session, &id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn update_customer(
    state: tauri::State<'_, AppState>,
    id: String,
    name: Option<String>,
    phone: Option<String>,
    company: Option<String>,
    email: Option<String>,
    notes: Option<String>,
    address: Option<String>,
    status: Option<String>,
) -> Result<CustomerRecord, String> {
    let status = match status {
        Some(s) => Some(parse_customer_status(&s).map_err(|e| e.to_string())?),
        None => None,
    };
    let input = UpdateCustomerInput {
        name,
        phone,
        company,
        email,
        notes,
        address,
        status,
    };
    update_customer_for_session(&state.pool, &state.session, &id, input)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn reassign_lead_owner(
    state: tauri::State<'_, AppState>,
    id: String,
    new_owner_id: String,
) -> Result<LeadRecord, String> {
    reassign_lead_owner_for_session(&state.pool, &state.session, &id, &new_owner_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn reassign_customer_owner(
    state: tauri::State<'_, AppState>,
    id: String,
    new_owner_id: String,
) -> Result<CustomerRecord, String> {
    reassign_customer_owner_for_session(&state.pool, &state.session, &id, &new_owner_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn create_lead_followup(
    state: tauri::State<'_, AppState>,
    lead_id: String,
    content: String,
    follow_up_time: String,
    method: String,
    result: String,
    next_follow_up_time: Option<String>,
) -> Result<FollowUpRecord, String> {
    create_lead_followup_for_session(
        &state.pool,
        &state.session,
        &lead_id,
        &content,
        &follow_up_time,
        &method,
        &result,
        next_follow_up_time.as_deref(),
    )
    .await
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn list_lead_followups(
    state: tauri::State<'_, AppState>,
    lead_id: String,
) -> Result<Vec<FollowUpRecord>, String> {
    list_lead_followups_for_session(&state.pool, &state.session, &lead_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn create_customer_followup(
    state: tauri::State<'_, AppState>,
    customer_id: String,
    content: String,
    follow_up_time: String,
    method: String,
    result: String,
    next_follow_up_time: Option<String>,
) -> Result<FollowUpRecord, String> {
    create_customer_followup_for_session(
        &state.pool,
        &state.session,
        &customer_id,
        &content,
        &follow_up_time,
        &method,
        &result,
        next_follow_up_time.as_deref(),
    )
    .await
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn list_customer_followups(
    state: tauri::State<'_, AppState>,
    customer_id: String,
) -> Result<Vec<FollowUpRecord>, String> {
    list_customer_followups_for_session(&state.pool, &state.session, &customer_id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn convert_lead_to_customer(
    state: tauri::State<'_, AppState>,
    lead_id: String,
    initial_status: String,
) -> Result<CustomerRecord, String> {
    convert_lead_to_customer_for_session(
        &state.pool,
        &state.session,
        &lead_id,
        &initial_status,
    )
    .await
    .map_err(|error| error.to_string())
}
