use backon::Retryable;

async fn fetch() -> anyhow::Result<String> {
    let response = reqwest::get("https://httpbingo.org/unstable?failure_rate=0.9").await?;
    if !response.status().is_success() {
        println!("retry: {}", response.status());
        anyhow::bail!("some kind of error");
    }

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
