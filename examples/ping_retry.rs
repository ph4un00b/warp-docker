use anyhow::Ok;

// * @see https://github.com/matter-labs/zksync-era/blob/v7.0.0-rc0/core/lib/utils/src/http_with_retries.rs
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://httpbingo.org/unstable?failure_rate=0.9";
    let max_retries = 5;
    let policy =
        reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(max_retries);
    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
            policy,
        ))
        .build();

    let res = client.get(url).send().await?;
    if !res.status().is_success() {
        println!("{}", res.status());
        anyhow::bail!("some kind of error");
    } else {
        print!("✅ succeeded");
    }

    Ok(())
}
