#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //? construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    //? use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;
    jamon("hola❗");
    Ok(())
}

#[tracing::instrument]
fn jamon(i: &str) {
    tracing::info!(?i)
}

// * out: INFO jamon{i="hola❗"}: tracing: i="hola❗"
