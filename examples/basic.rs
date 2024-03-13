use mage_core::{
    image::Point, load_font_image, run, App, Colour, Config, Font, PresentInput, PresentResult,
    TickInput, TickResult, WindowSize,
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
        window_size: WindowSize::FixedCellWithPixelSize(20 * 16, 10 * 16),
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
        let mut image = present_input.new_image();

        use rand::Rng;
        let mut rng = rand::thread_rng();
        image.fore_image.iter_mut().for_each(|c| {
            *c = rng.gen::<u32>() | 0xff000000;
        });
        image.back_image.iter_mut().for_each(|c| {
            *c = rng.gen::<u32>() | 0xff000000;
        });
        image.text_image.iter_mut().for_each(|c| {
            *c = rng.gen::<u8>() as u32;
        });

        let fps = (1.0 / self.dt) as u32;
        let message = format!("FPS: {} ", fps);
        image.draw_string(
            Point::default(),
            &message,
            Colour::White.into(),
            Colour::LightRed.into(),
        );

        present_input.blit(
            present_input.rect(),
            image.rect(),
            &image,
            Colour::Black.into(),
        );

        PresentResult::Changed
    }
}
