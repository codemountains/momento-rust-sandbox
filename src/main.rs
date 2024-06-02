mod cache_client;
mod dictionary;
mod list;
mod scalar;
mod set;
mod sorted_set;

use crate::cache_client::cache_client;
use crate::dictionary::dictionary;
use crate::list::list;
use crate::scalar::scalar;
use crate::set::set;
use crate::sorted_set::sorted_set;
use momento::{CredentialProvider, MomentoError, SimpleCacheClientBuilder};
use std::process;
use std::time::Duration;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), MomentoError> {
    dotenv().expect(".env file not found");

    // SimpleCacheClient の生成
    let mut client = match SimpleCacheClientBuilder::new(
        CredentialProvider::from_env_var("MOMENTO_AUTH_TOKEN".to_string())?,
        Duration::from_secs(60),
    ) {
        Ok(client) => client,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }
    .build();

    // キャッシュ名
    let cache_name = std::env::var("MOMENTO_CACHE_NAME").expect("Cache name must be set!");

    match scalar(&mut client, &cache_name).await {
        Ok(_) => {}
        Err(_) => {}
    }

    match dictionary(&mut client, &cache_name).await {
        Ok(_) => {}
        Err(_) => {}
    }

    match list(&mut client, &cache_name).await {
        Ok(_) => {}
        Err(_) => {}
    }

    match set(&mut client, &cache_name).await {
        Ok(_) => {}
        Err(_) => {}
    }

    match sorted_set(&mut client, &cache_name).await {
        Ok(_) => {}
        Err(_) => {}
    }

    // CacheClient バージョン
    match cache_client().await {
        Ok(_) => {}
        Err(_) => {}
    }

    Ok(())
}
