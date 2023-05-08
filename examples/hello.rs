use mage_core::{
    image::{Char, Point},
    run, App, Colour, Config, PresentInput, PresentResult, TickInput, TickResult,
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
    fn tick(&mut self, _tick_input: TickInput) -> TickResult {
        TickResult::Continue
    }

    fn present(&mut self, mut present_input: PresentInput) -> PresentResult {
        let mut image = present_input.new_image();
        image.clear(Colour::White.into(), Colour::Black.into());

        image.draw_string(
            Point::new(0, 0),
            "Hello, World!",
            Colour::Black.into(),
            Colour::Yellow.into(),
        );
        image.draw_char(
            Point::new(image.width as i32 - 1, 0),
            Char::new_char('A', Colour::LightRed.into(), Colour::Black.into()),
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
