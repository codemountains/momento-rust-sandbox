use momento::response::{DictionaryFetch, DictionaryGet};
use momento::{CollectionTtl, MomentoError, SimpleCacheClient};
use std::collections::HashMap;
pub async fn dictionary(c: &mut SimpleCacheClient, n: &str) -> Result<(), MomentoError> {
    // Dictionary のセット
    let key = "my_dict".to_string();
    let dict = HashMap::from_iter([
        ("my_dict_1", "my_dict_value_1"),
        ("my_dict_2", "my_dict_value_2"),
    ]);

    println!("Setting key: {key}, value: {:?}", dict);
    let ttl = CollectionTtl::default();
    match c.dictionary_set(n, key.clone(), dict.clone(), ttl).await {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };

    // Dictionary の取得
    match c.dictionary_fetch(n, key.clone()).await {
        Ok(r) => match r {
            DictionaryFetch::Hit { value } => {
                let v: HashMap<String, String> = value.try_into().expect("I stored a dictionary!");
                println!("Got value: {:?}", v);
            }
            DictionaryFetch::Miss => {
                println!("Cache miss!");
            }
        },
        Err(err) => {
            eprintln!("{err}");
        }
    };

    // Dictionary の取得（キーでの絞り込み）
    match c.dictionary_get(n, key.clone(), vec!["my_dict_1"]).await {
        Ok(r) => match r {
            DictionaryGet::Hit { value } => {
                let v: HashMap<String, String> = value.try_into().expect("I stored a dictionary!");
                println!("Got value: {:?}", v);
            }
            DictionaryGet::Miss => {
                println!("Cache miss!");
            }
        },
        Err(err) => {
            eprintln!("{err}");
        }
    };

    Ok(())
}
