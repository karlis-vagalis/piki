use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "account_type")]
pub enum AccountType {
    Service,
    Person,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub r#type: AccountType,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Group {
    pub id: Uuid,
    pub managed_by: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub parent_id: Option<Uuid>,
    pub _lft: i64,
    pub _rgt: i64,
}
