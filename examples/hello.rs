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

    fn present(&mut self, present_input: mage::PresentInput) -> mage::PresentResult {
        // Hello
        present_input.fore_image[0] = 0xff0000ff;
        present_input.back_image[0] = 0xff000000;
        present_input.text_image[0] = 0x00000048;

        present_input.fore_image[1] = 0xff00ff00;
        present_input.back_image[1] = 0xff000000;
        present_input.text_image[1] = 0x00000065;

        present_input.fore_image[2] = 0xff00ffff;
        present_input.back_image[2] = 0xff000000;
        present_input.text_image[2] = 0x0000006c;

        present_input.fore_image[3] = 0xffff0000;
        present_input.back_image[3] = 0xff000000;
        present_input.text_image[3] = 0x0000006c;

        present_input.fore_image[4] = 0xffff00ff;
        present_input.back_image[4] = 0xff000000;
        present_input.text_image[4] = 0x0000006f;

        present_input.fore_image[present_input.width as usize - 1] = 0xff0000ff;
        present_input.back_image[present_input.width as usize - 1] = 0xff000000;
        present_input.text_image[present_input.width as usize - 1] = 0x00000041;

        present_input.fore_image[present_input.width as usize + 0] = 0xff0000ff;
        present_input.back_image[present_input.width as usize + 0] = 0xff000000;
        present_input.text_image[present_input.width as usize + 0] = 0x00000048;

        present_input.fore_image[present_input.width as usize + 1] = 0xff00ff00;
        present_input.back_image[present_input.width as usize + 1] = 0xff000000;
        present_input.text_image[present_input.width as usize + 1] = 0x00000065;

        present_input.fore_image[present_input.width as usize + 2] = 0xff00ffff;
        present_input.back_image[present_input.width as usize + 2] = 0xff000000;
        present_input.text_image[present_input.width as usize + 2] = 0x0000006c;

        present_input.fore_image[present_input.width as usize + 3] = 0xffff0000;
        present_input.back_image[present_input.width as usize + 3] = 0xff000000;
        present_input.text_image[present_input.width as usize + 3] = 0x0000006c;

        present_input.fore_image[present_input.width as usize + 4] = 0xffff00ff;
        present_input.back_image[present_input.width as usize + 4] = 0xff000000;
        present_input.text_image[present_input.width as usize + 4] = 0x0000006f;

        mage::PresentResult::Changed
    }
}
