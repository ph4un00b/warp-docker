// * cargo run --example ping_backon
use backon::Retryable;

async fn fetch() -> anyhow::Result<String> {
    let url = "https://httpbingo.org/unstable?failure_rate=0.9";
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.map_err(|err| {
        println!("❌ Attempt - failed with error: {}", err);
        anyhow::anyhow!("some kind of error")
    })?;

    if !response.status().is_success() {
        println!("Attempt # failed with status code: {}", response.status());
        anyhow::bail!("some kind of error");
    }

    println!("Service is available!");
    Ok(response.text().await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fetch
        .retry(&backon::ExponentialBuilder::default().with_max_times(5))
        .await?;
    println!("✅ fetch succeeded");

    Ok(())
}
