use backon::Retryable;
use warp::Filter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();

    ping_database
        .retry(&backon::ExponentialBuilder::default().with_max_times(5))
        .await?;
    println!("✅ fetch succeeded");

    // use tracing_subscriber::EnvFilter;

    // let db = libsql_client::Client::from_config(libsql_client::Config {
    //     url: url::Url::parse("http://127.0.0.1:8080").unwrap(),
    //     auth_token: None,
    // })
    // .await
    // .unwrap();
    let db =
        // libsql::Database::open_remote_with_connector("http://localhost:8080", "", https).unwrap();
        libsql::Database::open_remote("http://sqld:8080", "").unwrap();
    let conn = db.connect().unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS users (username)", ())
        .await
        .unwrap();

    conn.execute("INSERT INTO users (username) VALUES ('alice')", ())
        .await
        .unwrap();

    if let Ok(mut a) = conn.query("SELECT * FROM users", ()).await {
        println!("rows?: {:?}", a.next().is_ok())
    }
    println!("hi 4560 👋");
    //? GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    warp::serve(hello).run(([0, 0, 0, 0], 4560)).await;
    Ok(())
}

//? async fn fetch<TUrl: reqwest::IntoUrl>(url: TUrl, tag: &str) -> anyhow::Result<()> {
async fn ping_database() -> anyhow::Result<String> {
    let url = "http://sqld:8080";
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.map_err(|err| {
        println!("❌ Attempt - failed with error: {}", err);
        anyhow::anyhow!("some kind of error")
    })?;

    if !response.status().is_success() {
        println!("Attempt # failed with status code: {}", response.status());
        anyhow::bail!("some kind of error");
    }

    let pong = response.text().await?;
    println!("Service is available!: {}", pong);

    Ok(pong)
}

fn setup_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    // let filter = EnvFilter::builder()
    //     .with_default_directive(tracing_subscriber::filter LevelFilter::INFO.into())
    //     .from_env_lossy();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    // tracing_subscriber::registry().with(filter).init();
}
