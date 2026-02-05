use crate::{
    domain::{
        account::{self, Account, AccountRepository},
        user::User,
    },
    infrastructure::error::ErrorApi,
};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct AccountRow {
    id: Uuid,
    user_id: Uuid,
    balance: f64,
}

impl From<AccountRow> for Account {
    fn from(row: AccountRow) -> Self {
        let token = account::get_token();
        Account::new(token, row.id, row.user_id, row.balance)
    }
}

pub struct AccountSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl AccountRepository for AccountSQLRepo {
    async fn create(
        &mut self,
        user: &User,
        init_balance: Option<f64>,
    ) -> Result<Account, ErrorApi> {
        let id = Uuid::new_v4();
        let account = account::factory::create(id, *user.id(), init_balance.unwrap_or(0.0))?;

        let row = sqlx::query_as!(
            AccountRow,
            r#"
            INSERT INTO accounts (id, user_id, balance)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, balance
            "#,
            account.id(),
            account.user_id(),
            account.balance()
        )
        .fetch_one(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        Ok(row.into())
    }

    async fn update(&mut self, account: &Account) -> Result<(), ErrorApi> {
        let affected = sqlx::query!(
            r#"
            UPDATE accounts
            SET balance = $1
            WHERE id = $2
            "#,
            account.balance(),
            account.id()
        )
        .execute(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?
        .rows_affected();

        if affected == 0 {
            return Err(ErrorApi::DataBase("Account not found".into()));
        }

        Ok(())
    }

    async fn delete(&mut self, account: &Account) -> Result<(), ErrorApi> {
        let affected = sqlx::query!(
            r#"
            DELETE FROM accounts
            WHERE id = $1
            "#,
            account.id(),
        )
        .execute(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?
        .rows_affected();

        if affected == 0 {
            return Err(ErrorApi::DataBase("Account not found".into()));
        }

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Option<Account> {
        sqlx::query_as!(
            AccountRow,
            r#"
            SELECT id, user_id, balance
            FROM accounts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.0.as_ref())
        .await
        .ok()?
        .map(Into::into)
    }

    async fn gets_by_user(&self, user: &User) -> Option<Vec<Account>> {
        let rows = sqlx::query_as!(
            AccountRow,
            r#"
            SELECT id, user_id, balance
            FROM accounts
            WHERE user_id = $1
            "#,
            *user.id()
        )
        .fetch_all(self.0.as_ref())
        .await
        .ok()?;

        if rows.is_empty() {
            return None;
        }

        Some(rows.into_iter().map(Into::into).collect())
    }
}
