use backon::Retryable;
use opentelemetry::global;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(feature = "console") {
        println!("console");
        console_subscriber::init();
    } else {
        println!("telemetry");
        // * run: docker run -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name("warp-docker")
            .install_simple()?;

        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        tracing_subscriber::registry()
            .with(opentelemetry)
            //? Continue logging to stdout
            .with(tracing_subscriber::fmt::Layer::default())
            .try_init()?;
    }

    // #[cfg(feature = "local")]
    // {
    //     std::env::set_var("DATABASE_URL", "http://localhost:8080");
    // }
    if cfg!(feature = "local") {
        std::env::set_var("DATABASE_URL", "http://localhost:8080");
    } else {
        std::env::set_var("DATABASE_URL", "http://sqld:8080");
    }

    ping_database
        .retry(
            &backon::ExponentialBuilder::default()
                .with_factor(1.2)
                .with_max_times(10),
        )
        .await?;

    let db_url = std::env::var("DATABASE_URL")?;
    let db = libsql::Database::open_remote(db_url, "").unwrap();
    let conn = db.connect().unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS users (username)", ())
        .await
        .unwrap();

    conn.execute("INSERT INTO users (username) VALUES ('alice')", ())
        .await
        .unwrap();

    if let Ok(mut a) = conn.query("SELECT * FROM users", ()).await {
        tracing::info!("rows?: {:?}", a.next().is_ok())
    }
    tracing::info!("hi 4560 👋");
    //? GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| {
        tracing::info!("✅ alo request");
        format!("Hello, {}!", name)
    });
    warp::serve(hello).run(([0, 0, 0, 0], 4560)).await;
    Ok(())
}

//? async fn fetch<TUrl: reqwest::IntoUrl>(url: TUrl, tag: &str) -> anyhow::Result<()> {
async fn ping_database() -> anyhow::Result<String> {
    let url = std::env::var("DATABASE_URL")?;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/version", url))
        .send()
        .await
        .map_err(|err| {
            tracing::error!("❌ Attempt: {}", err);
            anyhow::anyhow!("🍕 some kind of error")
        })?;

    if !response.status().is_success() {
        tracing::error!("Attempt # status code: {}", response.status());
        anyhow::bail!("🍔 some kind of error");
    }

    let pong = response.text().await?;
    tracing::info!("✅ Service is available!: {}", pong);
    Ok(pong)
}
