use mage::{run, App, Config};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    let filter = EnvFilter::from_default_env()
        .add_directive("wgpu=warn".parse().unwrap())
        .add_directive("mage=trace".parse().unwrap());
    tracing_subscriber::fmt::fmt()
        .without_time()
        .compact()
        .with_env_filter(filter)
        .init();

    info!("Starting...");

    let app = HelloApp::new();
    let config = Config::default();

    let _ = run(app, config).await;
}

struct HelloApp {}

impl HelloApp {
    fn new() -> Self {
        Self {}
    }
}

impl App for HelloApp {
    fn tick(&mut self, _tick_input: mage::TickInput) -> mage::TickResult {
        mage::TickResult::Continue
    }

    fn present(&mut self, _present_input: mage::PresentInput) -> mage::PresentResult {
        mage::PresentResult::Changed
    }
}
