use derive_new::new;
use futures::{stream, StreamExt, TryStreamExt};
use std::io::Error;
use tracing_opentelemetry::OpenTelemetrySpanExt;

mod otel;

#[derive(Debug, new)]
struct Task {
    id: i32,
}

#[tokio::main]
async fn main() {
    let tracer_provider = otel::init_provider();
    // mainでtracer_providerが生きてないと、traceが出力されない (考えれば当たり前か_(:3 」∠)_)
    otel::set_telemetry(&tracer_provider);

    {
        tracing::info!("futures;;streamを使った非同期処理サンプル");
        let result = stream::iter(1..=1000)
            .chunks(100)
            .map(|chunk| {
                let span = tracing::info_span!("fold chunk", ?chunk);
                tokio::spawn(async move {
                    let a = chunk.iter().fold(0, |acc, x| acc + x);
                    span.set_attribute("chunked a", a.to_string());
                    anyhow::Result::<i32>::Ok(a)
                })
            })
            .buffered(20)
            .map(|x| x?)
            .try_fold(Vec::<i32>::new(), |mut acc, x| async move {
                acc.push(x);
                anyhow::Result::Ok(acc)
            })
            .await;

        match result {
            Ok(x) => println!("Result: {:?}", x),
            Err(e) => println!("Error: {}", e),
        }
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
        .map(|x| x?)
        .try_fold(Vec::<i32>::new(), |mut acc, x| async move {
            acc.push(x);
            anyhow::Result::Ok(acc)
        })
        .await;

    match result {
        Ok(x) => println!("Result: {:?}", x),
        Err(e) => println!("Error: {}", e),
    }

    println!("futures;;streamを使った非同期処理サンプル");
    let result = stream::iter(1..=1000)
        .chunks(100)
        .map(|chunk| {
            tokio::spawn(async move {
                println!("{:?}", chunk);
                let a = async_func(&chunk).await?;
                anyhow::Result::<i32>::Ok(a)
            })
        })
        .buffered(20)
        .map(|x| x?)
        .try_fold(Vec::<i32>::new(), |mut acc, x| async move {
            acc.push(x);
            anyhow::Result::Ok(acc)
        })
        .await;

    match result {
        Ok(x) => println!("Result: {:?}", x),
        Err(e) => println!("Error: {}", e),
    }

    let result = stream::iter(1..=1000)
        .chunks(100)
        .map(|chunk| {
            tokio::spawn(async move {
                println!("{:?}", chunk);
                let a = async_create_task(&chunk).await?;
                anyhow::Result::<Task>::Ok(a)
            })
        })
        .buffered(20)
        .map(|x| x?)
        .try_fold(Vec::<Task>::new(), |mut acc, x| async move {
            acc.push(x);
            anyhow::Result::Ok(acc)
        })
        .await;

    match result {
        Ok(x) => println!("Result: {:?}", x),
        Err(e) => println!("Error: {}", e),
    }

    opentelemetry::global::shutdown_tracer_provider();
}

#[tracing::instrument]
async fn async_func(x: &[i32]) -> Result<i32, Error> {
    let a = x.iter().fold(0, |acc, x| acc + x);
    Ok(a)
}

#[tracing::instrument]
async fn async_create_task(x: &[i32]) -> Result<Task, Error> {
    let a = x.iter().fold(0, |acc, x| acc + x);
    Ok(Task::new(a))
}
