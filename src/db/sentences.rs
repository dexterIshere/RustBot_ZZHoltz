use sqlx::Error;
use sqlx::PgPool;
use sqlx::Row;
#[derive(Debug)]
pub struct Sentence {
    pub id: u32,
    pub insulte: String,
}

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

pub async fn trash_lister(pool: &PgPool) -> Result<Vec<Sentence>, Error> {
    let mut language = vec![];

    let recs = sqlx::query("SELECT id, insulte FROM insultes")
        .fetch_all(pool)
        .await?;

    for rec in recs {
        let id: i32 = rec.get("id");
        let insulte: String = rec.get("insulte");
        language.push(Sentence {
            id: id.try_into().unwrap(),
            insulte,
        });
    }

    Ok(language)
}

pub async fn select_random_sentence(pool: &PgPool) -> Result<String, sqlx::Error> {
    let random_trash: (String,) =
        sqlx::query_as("SELECT insulte FROM insultes ORDER BY RANDOM() LIMIT 1")
            .fetch_one(pool)
            .await?;

    Ok(random_trash.0)
}
