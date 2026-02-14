extern crate alloc;

use alloc::{vec, vec::Vec};
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Axis, Block, BorderType, Chart, Dataset, GraphType,
        Paragraph
    },
    Frame,
};
use tui_big_text::{BigText, PixelSize};

const TICK_MS: u32 = 100;
const LOGO_TICKS: u32 = 3000 / TICK_MS;
const LOGO_LETTERS: [&str; 2] = ["ANT", "TUI"];
const VIEW_TICKS: u32 = 3000 / TICK_MS;

pub struct App {
    tick: u32,
}

impl App {
    pub fn new() -> Self {
        Self { tick: 0 }
    }

    pub fn on_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn render(&self, f: &mut Frame) {
        let area = f.area();
        if self.tick < LOGO_TICKS {
            let idx = (self.tick * LOGO_LETTERS.len() as u32) / LOGO_TICKS;
            let letter = LOGO_LETTERS[idx.min((LOGO_LETTERS.len() - 1) as u32) as usize];
            let big_text = BigText::builder()
                .pixel_size(PixelSize::Quadrant)
                .style(Style::new().white())
                .lines(vec![letter.into()])
                .build();
            f.render_widget(big_text, area);
            return;
        }

        let title = self.clock_title();
        let frame_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);
        let inner_area = frame_block.inner(area);
        f.render_widget(frame_block, area);

        match self.view_mode() {
            ViewMode::Chart => self.render_chart(f, inner_area),
            ViewMode::Canvas => self.render_canvas(f, inner_area),
            ViewMode::Swarm => self.render_swarm(f, inner_area),
        }
    }

    fn clock_title(&self) -> alloc::string::String {
        let spinner = ['-', '\\', '|', '/'];
        let sp = spinner[(self.tick as usize) % spinner.len()];
        let phase = (self.tick / 10) % 100;
        alloc::format!("{sp} T{:02}", phase)
    }

    fn render_chart(&self, f: &mut Frame, area: Rect) {
        let count = area.width.max(2) as usize;
        let points = self.signal_points(count);
        let dataset = Dataset::default()
            .name("trail")
            .marker(ratatui::symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::new().white())
            .data(&points);

        let x_axis = Axis::default().bounds([0.0, (count.saturating_sub(1)) as f64]);
        let y_axis = Axis::default().bounds([0.0, 1.0]);
        let chart = Chart::new(vec![dataset]).x_axis(x_axis).y_axis(y_axis);
        f.render_widget(chart, area);
    }

    fn render_canvas(&self, f: &mut Frame, area: Rect) {
        let ants = self.ant_points();
        let canvas = Canvas::default()
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
            .paint(move |ctx| {
                ctx.draw(&CanvasLine {
                    x1: 10.0,
                    y1: 20.0,
                    x2: 90.0,
                    y2: 20.0,
                    color: Color::White,
                });
                ctx.draw(&CanvasLine {
                    x1: 40.0,
                    y1: 20.0,
                    x2: 40.0,
                    y2: 80.0,
                    color: Color::White,
                });
                ctx.draw(&CanvasLine {
                    x1: 40.0,
                    y1: 80.0,
                    x2: 80.0,
                    y2: 80.0,
                    color: Color::White,
                });
                ctx.draw(&Points {
                    coords: &ants,
                    color: Color::LightYellow,
                });
            });
        f.render_widget(canvas, area);
    }

    fn render_swarm(&self, f: &mut Frame, area: Rect) {
        let width = area.width.max(1) as usize;
        let height = area.height.max(1) as usize;
        let mut buf = alloc::string::String::with_capacity((width + 1) * height);
        for y in 0..height {
            for x in 0..width {
                let hash = (x as u32 * 13)
                    .wrapping_add(y as u32 * 7)
                    .wrapping_add(self.tick * 3);
                let ch = match hash % 16 {
                    0 | 1 => 'o',
                    2 | 3 | 4 => '.',
                    5 | 6 => ':',
                    _ => ' ',
                };
                buf.push(ch);
            }
            if y + 1 != height {
                buf.push('\n');
            }
        }
        let para = Paragraph::new(buf).style(Style::new().white());
        f.render_widget(para, area);
    }

    fn signal_points(&self, count: usize) -> Vec<(f64, f64)> {
        let mut points = Vec::with_capacity(count);
        for i in 0..count {
            let x = i as f64;
            let wave = triangle_ratio(self.tick + i as u32, 32, 0.05, 0.95);
            points.push((x, wave));
        }
        points
    }

    fn ant_points(&self) -> Vec<(f64, f64)> {
        let mut points = Vec::with_capacity(6);
        for i in 0..6u32 {
            let t = (self.tick + i * 11) % 100;
            let (x, y) = if t < 45 {
                let x = 10.0 + (t as f64) * 1.6;
                (x, 20.0)
            } else if t < 75 {
                let y = 20.0 + ((t - 45) as f64) * 2.0;
                (40.0, y)
            } else {
                let x = 40.0 + ((t - 75) as f64) * 2.0;
                (x, 80.0)
            };
            points.push((x, y));
        }
        points
    }

    fn view_mode(&self) -> ViewMode {
        let t = self.tick.saturating_sub(LOGO_TICKS);
        let phase = (t / VIEW_TICKS) % 3;
        match phase {
            0 => ViewMode::Chart,
            1 => ViewMode::Canvas,
            _ => ViewMode::Swarm,
        }
    }
}

#[derive(Clone, Copy)]
enum ViewMode {
    Chart,
    Canvas,
    Swarm,
}

fn triangle_ratio(tick: u32, period: u32, min: f64, max: f64) -> f64 {
    if period < 2 || min >= max {
        return min;
    }
    let span = max - min;
    let half = (period / 2) as f64;
    let phase = (tick % period) as f64;
    let value = if phase < half {
        (phase * span) / half
    } else {
        ((period as f64 - phase) * span) / half
    };
    min + value
}
