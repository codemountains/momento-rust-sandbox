use momento::response::Get;
use momento::{MomentoError, SimpleCacheClient};

pub async fn scalar(c: &mut SimpleCacheClient, n: &str) -> Result<(), MomentoError> {
    // キー・バリュー（文字列）のセット
    let key = "my_key".to_string();
    let value = "my_value".to_string();

    println!("Setting key: {key}, value: {value}");
    match c.set(n, key.clone(), value.clone(), None).await {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };

    // キー・バリュー（文字列）の取得
    match c.get(n, key.clone()).await {
        Ok(r) => match r {
            Get::Hit { value } => {
                let v: String = value.try_into().expect("I stored a string!");
                println!("Got value: {v}");
            }
            Get::Miss => {
                println!("Cache miss!");
            }
        },
        Err(err) => {
            eprintln!("{err}");
        }
    };

    Ok(())
}
