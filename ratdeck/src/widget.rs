use alloc::format;
use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

pub struct CheeseMeter {
    pub label: &'static str,
    pub value: u16,
}

impl Widget for CheeseMeter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 6 || area.height < 3 {
            return;
        }
        let bar_y = area.y + area.height / 2;
        let bar_x = area.x + 1;
        let bar_w = area.width.saturating_sub(2);
        let fill = (bar_w as u32 * self.value as u32 / 100) as u16;

        for i in 0..bar_w {
            let symbol = if i < fill { "█" } else { "░" };
            buf[(bar_x + i, bar_y)]
                .set_symbol(symbol)
                .set_style(Style::new().yellow());
        }

        let label = format!("{} {}%", self.label, self.value);
        buf.set_string(bar_x, area.y + 1, label, Style::new().white());
    }
}
