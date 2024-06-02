use momento::{CollectionTtl, MomentoError, SimpleCacheClient};

pub async fn set(c: &mut SimpleCacheClient, n: &str) -> Result<(), MomentoError> {
    // Set のセット
    let key = "my_set".to_string();
    let sets = vec![
        "my_set_1", "my_set_2", "my_set_3", "my_set_3", "my_set_4", "my_set_5", "my_set_6",
        "my_set_6", "my_set_7", "my_set_8",
    ];

    println!("Setting key: {key}, value: {:?}", sets);
    let ttl = CollectionTtl::default();
    match c.set_add_elements(n, key.clone(), sets.clone(), ttl).await {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };

    // Set の取得
    match c.set_fetch(n, key.clone()).await {
        Ok(r) => match r.value {
            Some(hs) => {
                let values: Vec<String> = hs
                    .into_iter()
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
