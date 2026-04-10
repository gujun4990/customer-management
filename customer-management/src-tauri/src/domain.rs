use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
}

impl UserRecord {
    pub fn new(id: &str, username: &str, is_admin: bool) -> Self {
        Self {
            id: id.into(),
            username: username.into(),
            is_admin,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeadStatus {
    New,
    Assigned,
    InProgress,
    Converted,
    Closed,
}

impl LeadStatus {
    pub fn all() -> Vec<Self> {
        vec![
            Self::New,
            Self::Assigned,
            Self::InProgress,
            Self::Converted,
            Self::Closed,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerStatus {
    Active,
    Silent,
    Lost,
}

impl CustomerStatus {
    pub fn all() -> Vec<Self> {
        vec![Self::Active, Self::Silent, Self::Lost]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeadRecord {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub company: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub owner: UserRecord,
    pub status: LeadStatus,
}

impl LeadRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        name: &str,
        phone: &str,
        company: &str,
        email: Option<&str>,
        notes: Option<&str>,
        address: Option<&str>,
        owner: UserRecord,
        status: LeadStatus,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            phone: phone.into(),
            company: company.into(),
            email: email.map(Into::into),
            notes: notes.map(Into::into),
            address: address.map(Into::into),
            owner,
            status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomerRecord {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub company: String,
    pub email: Option<String>,
    pub notes: Option<String>,
    pub address: Option<String>,
    pub owner: UserRecord,
    pub status: CustomerStatus,
}

impl CustomerRecord {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        name: &str,
        phone: &str,
        company: &str,
        email: Option<&str>,
        notes: Option<&str>,
        address: Option<&str>,
        owner: UserRecord,
        status: CustomerStatus,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            phone: phone.into(),
            company: company.into(),
            email: email.map(Into::into),
            notes: notes.map(Into::into),
            address: address.map(Into::into),
            owner,
            status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "id", rename_all = "snake_case")]
pub enum FollowUpTarget {
    Lead(String),
    Customer(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowUpRecord {
    pub id: String,
    pub target: FollowUpTarget,
    pub content: String,
    pub follow_up_time: String,
    pub method: String,
    pub result: String,
    pub next_follow_up_time: Option<String>,
}

impl FollowUpRecord {
    pub fn new_for_lead(
        id: &str,
        lead_id: &str,
        content: &str,
        follow_up_time: &str,
        method: &str,
        result: &str,
        next_follow_up_time: Option<&str>,
    ) -> Self {
        Self {
            id: id.into(),
            target: FollowUpTarget::Lead(lead_id.into()),
            content: content.into(),
            follow_up_time: follow_up_time.into(),
            method: method.into(),
            result: result.into(),
            next_follow_up_time: next_follow_up_time.map(Into::into),
        }
    }

    pub fn new_for_customer(
        id: &str,
        customer_id: &str,
        content: &str,
        follow_up_time: &str,
        method: &str,
        result: &str,
        next_follow_up_time: Option<&str>,
    ) -> Self {
        Self {
            id: id.into(),
            target: FollowUpTarget::Customer(customer_id.into()),
            content: content.into(),
            follow_up_time: follow_up_time.into(),
            method: method.into(),
            result: result.into(),
            next_follow_up_time: next_follow_up_time.map(Into::into),
        }
    }
}
