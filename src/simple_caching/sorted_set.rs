use momento::response::SortedSetFetch;
use momento::sorted_set::{Order, SortedSetElement};
use momento::{CollectionTtl, MomentoError, SimpleCacheClient};

pub async fn sorted_set(c: &mut SimpleCacheClient, n: &str) -> Result<(), MomentoError> {
    // Sorted set のセット
    let key = "my_sorted_set".to_string();
    let sorted_sets = vec![
        SortedSetElement {
            value: "my_sorted_set_200".into(),
            score: 200.0,
        },
        SortedSetElement {
            value: "my_sorted_set_100".into(),
            score: 100.0,
        },
        SortedSetElement {
            value: "my_sorted_set_50".into(),
            score: 50.0,
        },
        SortedSetElement {
            value: "my_sorted_set_1".into(),
            score: 1.0,
        },
    ];

    println!("Setting key: {key}, value: {:?}", sorted_sets);
    let ttl = CollectionTtl::default();
    match c
        .sorted_set_put(n, key.clone(), sorted_sets.clone(), ttl)
        .await
    {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };

    // Sorted set のインデックスが 1 以上 2 以下までのデータを取得
    match c
        .sorted_set_fetch_by_index(n, key.clone(), Order::Ascending, 1..=2)
        .await
    {
        Ok(r) => match r {
            SortedSetFetch::Hit { elements } => {
                let values: Vec<String> = elements
                    .into_iter()
                    .map(|v| String::from_utf8(v.value).expect(""))
                    .collect();
                println!("Got value: {:?}", values);
            }
            SortedSetFetch::Miss => {
                println!("Cache miss!");
            }
        },
        Err(err) => {
            eprintln!("{err}");
        }
    };

    // Sorted set のスコアが 100 以上 200 以下までのデータを取得
    match c
        .sorted_set_fetch_by_score(n, key.clone(), Order::Ascending, 100.0..=200.0)
        .await
    {
        Ok(r) => match r {
            SortedSetFetch::Hit { elements } => {
                let values: Vec<String> = elements
                    .into_iter()
                    .map(|v| String::from_utf8(v.value).expect(""))
                    .collect();
                println!("Got value: {:?}", values);
            }
            SortedSetFetch::Miss => {
                println!("Cache miss!");
            }
        },
        Err(err) => {
            eprintln!("{err}");
        }
    };

    Ok(())
}
