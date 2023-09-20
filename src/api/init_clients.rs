use std::convert::TryInto;
use surf::Client;
use surf::Config;
use surf::Url;

pub async fn get_poke_client() -> Result<Client, surf::Error> {
    let config = Config::new()
        .set_base_url(Url::parse("https://api-pokemon-fr.vercel.app/api/v1/")?)
        .try_into()?;

    let client: Client = config;

    Ok(client)
}
