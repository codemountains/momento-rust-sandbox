use momento::config::configurations::laptop;
use momento::requests::cache::basic::get::Get;
use momento::{CacheClient, CredentialProvider, MomentoError};
use std::time::Duration;

const CACHE_NAME: &str = "momento-sandbox";

pub async fn cache_client() -> Result<(), MomentoError> {
    let cache_client = CacheClient::builder()
        .default_ttl(Duration::from_secs(60))
        .configuration(laptop::latest())
        .credential_provider(CredentialProvider::from_env_var(
            "MOMENTO_AUTH_TOKEN".to_string(),
        )?)
        .build()?;

    match cache_client.set(CACHE_NAME, "mykey", "myvalue").await {
        Ok(_) => println!("Successfully stored key 'mykey' with value 'myvalue'"),
        Err(e) => println!("Error: {}", e),
    }

    let value: String = match cache_client.get(CACHE_NAME, "mykey").await? {
        Get::Hit { value } => value.try_into().expect("I stored a string!"),
        Get::Miss => {
            println!("Cache miss!");
            return Ok(()); // probably you'll do something else here
        }
    };

    println!("Successfully retrieved value: {}", value);

    Ok(())
}
