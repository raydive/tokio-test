use futures::{stream, StreamExt, TryStreamExt};

#[tokio::main]
async fn main() {
    println!("futures;;streamを使った非同期処理サンプル");
    let result = stream::iter(1..=1000)
        .chunks(100)
        .map(|chunk| {
            tokio::spawn(async move {
                println!("{:?}", chunk);
                let a = chunk.iter().fold(0, |acc, x| acc + x);
                anyhow::Result::<i32>::Ok(a)
            })
        })
        .buffered(20)
        .map(|x| x? )
        .try_fold(Vec::<i32>::new(), |mut acc, x| async move {
            acc.push(x);
            anyhow::Result::Ok(acc)
        }).await;
    
    match result {
        Ok(x) => println!("Result: {:?}", x),
        Err(e) => println!("Error: {}", e),
    }

    println!("-----------------------------");
    println!("futures;;streamを使った同期処理サンプル");
    let result = stream::iter(1..=1000)
    .chunks(100)
    .then(|chunk| {
        tokio::spawn(async move {
            println!("{:?}", chunk);
            let a = chunk.iter().fold(0, |acc, x| acc + x);
            anyhow::Result::<i32>::Ok(a)
        })
    })
    .map(|x| x? )
    .try_fold(Vec::<i32>::new(), |mut acc, x| async move {
        acc.push(x);
        anyhow::Result::Ok(acc)
    }).await;

    match result {
        Ok(x) => println!("Result: {:?}", x),
        Err(e) => println!("Error: {}", e),
    }
}
