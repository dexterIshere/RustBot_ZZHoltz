use crate::api::client::init_clients::get_poke_client;
use serde_json::Value;
use surf::Error;

pub async fn test() -> Result<String, Error> {
    let client = get_poke_client().await?;

    let mut res = client.get("pokemon/1").send().await?;

    let body: Value = res.body_json().await?;

    match body["name"]["fr"].as_str() {
        Some(name) => Ok(name.to_string()),
        None => Err(surf::Error::from_str(
            400,
            "Champ 'name.fr' manquant ou invalide",
        )),
    }
}
