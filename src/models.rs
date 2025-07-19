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

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub r#type: AccountType,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
