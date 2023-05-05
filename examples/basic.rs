use mage_core::{
    load_font_image, run, App, Config, Font, PresentInput, PresentResult, TickInput, TickResult,
};
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

    let app = TestApp::new();
    let config = Config {
        font: Font::Custom(load_font_image(include_bytes!("font3.png")).unwrap()),
        ..Default::default()
    };

    let _ = run(app, config).await;
}

struct TestApp {
    dt: f32,
}

impl TestApp {
    fn new() -> Self {
        Self { dt: 0.0 }
    }
}

impl App for TestApp {
    fn tick(&mut self, tick_input: TickInput) -> TickResult {
        self.dt = tick_input.dt.num_microseconds().unwrap() as f32 / 1_000_000.0;
        TickResult::Continue
    }

    fn present(&mut self, mut present_input: PresentInput) -> PresentResult {
        randomise(present_input.fore_image);
        randomise(present_input.back_image);
        randomise(present_input.text_image);

        let fps = (1.0 / self.dt) as u32;
        let message = format!("FPS: {} ", fps);
        let message = message.as_bytes();
        present_input.print_at(0, 0, message, 0xffffff, 0x0000ff);

        PresentResult::Changed
    }
}

fn randomise(image: &mut [u32]) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for pixel in image {
        *pixel = rng.gen::<u32>() | 0xff000000;
    }
}
