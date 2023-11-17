#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

#[cfg(test)]
mod test;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("{APP_NAME} v{APP_VERSION} - developed by {APP_AUTHORS}");

    Ok(())
}
