use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window};
use mousefood::embedded_graphics::geometry;
use mousefood::embedded_graphics::pixelcolor::Rgb565;
use mousefood::error::Error;
use mousefood::prelude::*;
use ratatui::Terminal;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[path = "../app.rs"]
mod app;
use app::App;

fn main() -> Result<(), Error> {
    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let simulator_window = Rc::new(RefCell::new(Window::new(
        "antui simulator",
        &output_settings,
    )));
    simulator_window.borrow_mut().set_max_fps(30);

    let mut display = SimulatorDisplay::<Rgb565>::new(geometry::Size::new(72, 40));

    let window_handle = Rc::clone(&simulator_window);
    let backend_config = EmbeddedBackendConfig {
        flush_callback: Box::new(move |display| {
            window_handle.borrow_mut().update(display);
        }),
        ..Default::default()
    };
    let backend: EmbeddedBackend<SimulatorDisplay<_>, _> =
        EmbeddedBackend::new(&mut display, backend_config);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_duration = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            app.render(f);
        })?;

        let now = Instant::now();
        if now.duration_since(last_tick) >= tick_duration {
            app.on_tick();
            last_tick = now;
        }

        let window = simulator_window.borrow_mut();
        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                return Ok(());
            }
        }
    }
}
