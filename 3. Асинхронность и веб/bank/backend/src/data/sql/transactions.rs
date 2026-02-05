use crate::{
    domain::{
        account::Account,
        transaction::{self, Transaction, TransactionRepository},
    },
    infrastructure::error::ErrorApi,
};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::{prelude::FromRow, PgPool};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct TransactionRow {
    id: Uuid,
    operation: transaction::Operation,
    amount: f64,
    from_id: Option<Uuid>,
    to_id: Option<Uuid>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<TransactionRow> for Transaction {
    fn from(row: TransactionRow) -> Self {
        let token = transaction::get_token();
        Transaction::new(
            token,
            row.id,
            row.operation,
            row.amount,
            row.from_id,
            row.to_id,
            row.created_at,
        )
    }
}

pub struct TransactionSQLRepo(pub Arc<PgPool>);

#[async_trait]
impl TransactionRepository for TransactionSQLRepo {
    async fn create_deposit(&mut self, amount: f64, to: &Account) -> Result<Transaction, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        let tx = transaction::factory::create_deposit(id, amount, *to.id(), created_at)?;
        info!("{:#?}", tx);

        let row = sqlx::query_as!(
            TransactionRow,
            r#"
        INSERT INTO transactions (id, operation, amount, from_id, to_id, created_at)
        VALUES ($1, 'deposit', $2, NULL, $3, $4)
        RETURNING id, operation as "operation!: transaction::Operation",
            amount, from_id, to_id, created_at
        "#,
            tx.id(),
            tx.amount(),
            *tx.to_id(),
            tx.created_at()
        )
        .fetch_one(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        Ok(row.into())
    }

    async fn create_withdrawal(
        &mut self,
        amount: f64,
        from: &Account,
    ) -> Result<Transaction, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        let tx = transaction::factory::create_withdrawal(id, amount, *from.id(), created_at)?;

        let row = sqlx::query_as!(
            TransactionRow,
            r#"
        INSERT INTO transactions (id, operation, amount, from_id, to_id, created_at)
        VALUES ($1, 'withdrawal', $2, $3, NULL, $4)
        RETURNING id, operation as "operation!: transaction::Operation",
                  amount, from_id, to_id, created_at
        "#,
            tx.id(),
            tx.amount(),
            *tx.from_id(),
            tx.created_at()
        )
        .fetch_one(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        Ok(row.into())
    }

    async fn create_transfer(
        &mut self,
        amount: f64,
        from: &Account,
        to: &Account,
    ) -> Result<Transaction, ErrorApi> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        let tx =
            transaction::factory::create_transfer(id, amount, *from.id(), *to.id(), created_at)?;

        let row = sqlx::query_as!(
            TransactionRow,
            r#"
        INSERT INTO transactions (id, operation, amount, from_id, to_id, created_at)
        VALUES ($1, 'transfer', $2, $3, $4, $5)
        RETURNING id, operation as "operation!: transaction::Operation",
                  amount, from_id, to_id, created_at
        "#,
            tx.id(),
            tx.amount(),
            *tx.from_id(),
            *tx.to_id(),
            tx.created_at()
        )
        .fetch_one(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        Ok(row.into())
    }

    async fn delete(&mut self, transaction: &Transaction) -> Result<(), ErrorApi> {
        let res = sqlx::query!(
            r#"
        DELETE FROM transactions
        WHERE id = $1
        "#,
            transaction.id()
        )
        .execute(self.0.as_ref())
        .await
        .map_err(|e| ErrorApi::DataBase(e.to_string()))?;

        if res.rows_affected() == 0 {
            return Err(ErrorApi::DataBase("Transaction not found".into()));
        }

        Ok(())
    }

    async fn get_by_id(&self, id: Uuid) -> Option<Transaction> {
        let row = sqlx::query_as!(
            TransactionRow,
            r#"
        SELECT id,
               operation as "operation!: transaction::Operation",
               amount,
               from_id,
               to_id,
               created_at
        FROM transactions
        WHERE id = $1
        "#,
            id
        )
        .fetch_optional(self.0.as_ref())
        .await
        .ok()??;

        Some(row.into())
    }

    async fn gets_by_account(&self, account: &Account) -> Option<Vec<Transaction>> {
        let rows = sqlx::query_as!(
            TransactionRow,
            r#"
        SELECT id,
               operation as "operation!: transaction::Operation",
               amount,
               from_id,
               to_id,
               created_at
        FROM transactions
        WHERE from_id = $1 OR to_id = $1
        ORDER BY created_at DESC
        "#,
            account.id()
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
