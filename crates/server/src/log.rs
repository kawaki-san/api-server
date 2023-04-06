use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn initialise() {
    let crate_name = env!("CARGO_CRATE_NAME");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{crate_name}=debug,tower_http=debug").into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
