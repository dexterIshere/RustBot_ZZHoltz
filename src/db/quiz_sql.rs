use sqlx::Error;
use sqlx::PgPool;

pub async fn add_winner(pool: &PgPool, winner_id: i64) -> Result<(), Error> {
    let result: Option<(i64,)> = sqlx::query_as("SELECT win FROM winners WHERE did = $1")
        .bind(winner_id)
        .fetch_optional(pool)
        .await?;

    println!("Victoires existantes : {:?}", result);

    if let Some(_) = result {
        sqlx::query("UPDATE winners SET win = win + 1 WHERE did = $1")
            .bind(winner_id)
            .execute(pool)
            .await?;
    } else {
        println!("Aucune victoire existante, ajout d'un nouveau gagnant");
        sqlx::query("INSERT INTO winners (did, win) VALUES ($1, 1)")
            .bind(winner_id)
            .execute(pool)
            .await?;
    }

    Ok(())
}
