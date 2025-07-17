use sqlx::types::Uuid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
    Service,
    Person,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    #[sqlx(rename = "type")]
    pub r#type: AccountType,
}
