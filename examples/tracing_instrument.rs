use std::fmt::Display;

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
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let tuple = Tracable(1, 2, 3);
    jamon("hola❗", &tuple);
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

// * out:  INFO My::name{field_2=Tracable(1, 2, 3) tu_mamacita=👀serializada}: tracing_instrument: field_1="hola❗"
