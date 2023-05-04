use mage::{run, App};
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

    let _ = run(app).await;
}

struct HelloApp {}

impl HelloApp {
    fn new() -> Self {
        Self {}
    }
}

impl App for HelloApp {
    fn tick(&mut self, tick_input: mage::TickInput) -> mage::TickResult {
        mage::TickResult::Continue
    }

    fn present(&mut self, present_input: mage::PresentInput) -> mage::PresentResult {
        mage::PresentResult::Changed
    }
}
