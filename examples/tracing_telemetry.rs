use opentelemetry::global;
use std::fmt::Display;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use warp::Filter;

#[derive(Debug)]
struct Tracable(i32, i32, i32);

impl Display for Tracable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "👀serializada")?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //? Allows you to pass along context (i.e., trace IDs) across services
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    //? Sets up the machinery needed to export data to Jaeger
    //? There are other OTel crates that provide pipelines for the vendors
    //? mentioned earlier.
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("ex-tracing-telemetry")
        .install_simple()?;

    //? Create a tracing layer with the configured tracer
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    //? The SubscriberExt and SubscriberInitExt traits are needed to extend the
    //? Registry to accept `opentelemetry (the OpenTelemetryLayer type).
    // todo: @see https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples
    tracing_subscriber::registry()
        .with(opentelemetry)
        //? Continue logging to stdout
        .with(fmt::Layer::default())
        .try_init()?;

    let tuple = Tracable(1, 2, 3);
    jamon("hola❗", &tuple);

    let hello = warp::path!("hello" / String).map(|name| {
        tracing::info!("✅ alo request");
        format!("Hello, {}!", name)
    });

    warp::serve(hello).run(([0, 0, 0, 0], 4560)).await;

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
#[tracing::instrument(
    name = "My::name",
    skip(field_1),
    fields(
        //? `%` will serialize with `Display`
        tu_mamacita = %field_2
    ),
)]
fn jamon(field_1: &str, field_2: &Tracable) {
    tracing::info!(field_1)
}
