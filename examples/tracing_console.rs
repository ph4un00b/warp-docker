use warp::Filter;

// * install: cargo install --locked tokio-console
// * run in other terminal: tokio-console
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    console_subscriber::init();

    let hello = warp::path!("hello" / String).map(|name| {
        tracing::info!("alo request");
        format!("Hello, {}!", name)
    });

    warp::serve(hello).run(([0, 0, 0, 0], 4560)).await;

    Ok(())
}
