// * @see https://github.com/matter-labs/zksync-era/blob/v7.0.0-rc0/core/lib/utils/src/http_with_retries.rs
#[tokio::main]
async fn main() {
    let url = "https://examplejamon.com";
    let max_retries = 5;
    let client = reqwest::Client::new();

    for retry in 1..=max_retries {
        match client.get(url).send().await {
            Ok(response) if response.status().is_success() => {
                println!("Service is available!");
                break;
            }
            Ok(response) => println!(
                "Attempt {} failed with status code: {}",
                retry,
                response.status()
            ),
            Err(err) => println!("❌ Attempt {} failed with error: {}", retry, err),
        }

        if retry < max_retries {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
