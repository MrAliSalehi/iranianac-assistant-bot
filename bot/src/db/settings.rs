use sqlx::{query, PgPool};

use crate::Res;

pub async fn reminder_enabled(db: &PgPool) -> eyre::Result<bool> {
    Ok(query!("select reminder_enabled from settings limit 1")
        .fetch_one(db)
        .await?
        .reminder_enabled)
}

pub async fn reminder_status(db: &PgPool, enabled: bool) -> Res {
    query!("update settings set reminder_enabled=$1", enabled)
        .execute(db)
        .await?;
    Ok(())
}
