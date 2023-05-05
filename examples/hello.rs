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

    fn present(&mut self, mut present_input: mage::PresentInput) -> mage::PresentResult {
        present_input.print_at(0, 0, b"Hello, World!", 0xff000000, 0xff00ffff);
        present_input.print_at(-1, 0, b"A", 0xff0000ff, 0x00000000);

        mage::PresentResult::Changed
    }
}
