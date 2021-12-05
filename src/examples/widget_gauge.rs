use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge};
use tui::Terminal;

pub fn render_widget_gauge(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(20);
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(gauge, rendering_area);
    });
}
