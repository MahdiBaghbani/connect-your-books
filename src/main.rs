use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracing_subscriber::filter::LevelFilter;

fn main() {
    // loads the .env file from the current directory or parents.
    dotenvy::dotenv().ok();

    // parse an `EnvFilter` configuration from environment variable.
    let filter: EnvFilter = EnvFilter::builder()
        .with_default_directive(LevelFilter::ERROR.into())
        .with_env_var("CYB_TRACE_LEVEL")
        .from_env_lossy();

    // apply the filter to this layer *only*.
    let filtered_layer = fmt::layer().with_level(true).with_filter(filter);

    tracing_subscriber::registry().with(filtered_layer).init();

    // run api server.
    api::run();
}
