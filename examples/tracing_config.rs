#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        //? Use a more compact, abbreviated log format
        .compact()
        //? Display source code file paths
        .with_file(true)
        //? Display source code line numbers
        .with_line_number(true)
        //? Display the thread ID an event was recorded on
        .with_thread_ids(true)
        //? Don't display the event's target (module path)
        .with_target(false)
        //? Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    jamon("hola❗");
    Ok(())
}

#[tracing::instrument]
fn jamon(i: &str) {
    tracing::info!(?i)
}

// * with-target(t): INFO ThreadId(01) jamon: tracing_config: examples\tracing_config.rs:24: i="hola❗" i="hola
// * with-target(f): INFO ThreadId(01) jamon: examples\tracing_config.rs:24: i="hola❗" i="hola❗"
