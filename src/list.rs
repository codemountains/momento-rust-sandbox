use momento::{CollectionTtl, MomentoError, SimpleCacheClient};

pub async fn list(c: &mut SimpleCacheClient, n: &str) -> Result<(), MomentoError> {
    // List のセット
    let key = "my_lst".to_string();
    let list = ["my_list_1", "my_list_2", "my_list_3"];

    println!("Setting key: {key}, value: {:?}", list);
    let ttl = CollectionTtl::default();
    match c.list_set(n, key.clone(), list, ttl).await {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };

    // List の取得
    match c.list_fetch(n, key.clone()).await {
        Ok(r) => match r {
            Some(entry) => {
                let values: Vec<String> = entry
                    .value()
                    .iter()
                    .map(|v| String::from_utf8(v.clone()).expect(""))
                    .collect();
                println!("Got value: {:?}", values);
            }
            None => {
                println!("Cache miss!");
            }
        },
        Err(err) => {
            eprintln!("{err}");
        }
    };

    Ok(())
}
