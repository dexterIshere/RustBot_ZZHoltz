use sqlx::Error;
use sqlx::PgPool;

pub async fn add_trash(pool: &PgPool, new_trash: &str) -> Result<i64, Error> {
    let id: (i64,) = sqlx::query_as("INSERT INTO insultes (insulte) VALUES ($1) RETURNING id")
        .bind(new_trash)
        .fetch_one(pool)
        .await?;

    Ok(id.0)
}

pub async fn delete_trash(pool: &PgPool, trash_to_delete: i64) -> Result<u64, Error> {
    let lignes_affectees = sqlx::query("DELETE FROM insultes WHERE id = $1")
        .bind(trash_to_delete)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(lignes_affectees)
}
