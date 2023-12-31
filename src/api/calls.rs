use serde_json::Value;
use surf::Error;

use super::init_clients::get_poke_client;

pub async fn fetch_pokemon_data(id: i32) -> Result<(String, String), Error> {
    let client = get_poke_client().await?;

    let mut res = client.get(format!("pokemon/{}", id)).send().await?;

    let body: Value = res.body_json().await?;

    let image = body["sprites"]["regular"].as_str().unwrap_or_default();
    let name = body["name"]["fr"].as_str().unwrap_or_default();

    Ok((image.to_string(), name.to_string()))
}
