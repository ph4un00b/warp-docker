#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //? construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    //? use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;
    jamon("hola❗");
    Ok(())
}

/*
 * The easiest way to emit spans is with the instrument proc-macro annotation provided by tracing,
 * which re-writes the bodies of functions to emit
 * spans each time they are invoked; e.g.:
 *
 * Each invocation of jamon will emit a tracing Span that:
 *  🍕 has a verbosity level of info (the "middle ground" verbosity),
 *  🍕 is named jamon,
 *  🍕 has fields field_1, whose values are the arguments of jamon
 */
#[tracing::instrument]
fn jamon(field_1: &str) {
    tracing::info!(field_1)
}

// * out: INFO jamon{field_1="hola❗"}: tracing: field_1="hola❗"
