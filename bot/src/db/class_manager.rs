use sqlx::{query, query_as, PgPool};

use crate::{models::class_model::ClassModel, Res};

pub type ClassName = String;

pub type ProfessorName = String;

pub async fn add(class: &ClassModel, db: &PgPool) -> eyre::Result<bool> {
    Ok(query!(
        "select insert_new($1,$2,$3,$4,$5) as res",
        class.name,
        class.professor_name,
        class.thread_id,
        class.start_time,
        class.day
    )
    .fetch_one(db)
    .await?
    .res
    .unwrap_or(false))
}

pub async fn set_status(
    db: &PgPool,
    id: i32,
    enabled: bool,
) -> eyre::Result<(bool, ClassName, ProfessorName)> {
    let d = query!(
        "update class set enabled=$1 where id=$2 returning enabled,name,professor_name",
        enabled,
        id
    )
    .fetch_one(db)
    .await?;
    Ok((d.enabled, d.name, d.professor_name))
}

pub async fn get_all_by_thread_id(db: &PgPool, thread_id: i32) -> eyre::Result<Vec<ClassModel>> {
    Ok(query_as!(
        ClassModel,
        "select * from class where thread_id=$1",
        thread_id
    )
    .fetch_all(db)
    .await?)
}

pub async fn get_classes_for_notification(db: &PgPool) -> eyre::Result<Vec<ClassModel>> {
    Ok(
        query_as!(
            ClassModel,
            "SELECT * FROM class WHERE enabled = true AND (last_notification IS NULL OR last_notification <= (current_timestamp - interval '2 hours'));"
        ).fetch_all(db).await?
    )
}

pub async fn mark_notification(db: &PgPool, id: i32) -> Res {
    query!("update class set last_notification=now() where id =$1", id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn get_all_classes(db: &PgPool) -> eyre::Result<Vec<ClassModel>> {
    Ok(query_as!(ClassModel, "select * from class")
        .fetch_all(db)
        .await?)
}
