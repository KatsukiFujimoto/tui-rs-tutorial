use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Sparkline};
use tui::Terminal;

pub fn render_widget_sparkline(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let sparkline = Sparkline::default()
            .block(Block::default().title("Sparkline").borders(Borders::ALL))
            .data(&[0, 2, 3, 4, 1, 4, 10])
            .max(5)
            .style(Style::default().fg(Color::Red).bg(Color::White));
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(sparkline, rendering_area);
    });
}
