use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::symbols::DOT;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Tabs};
use tui::Terminal;

pub fn render_widget_tabs(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let titles: Vec<Spans> = ["Tab1", "Tab2", "Tab3", "Tab4"]
            .iter()
            .cloned()
            .map(Spans::from)
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().title("Tabs").borders(Borders::ALL))
            .select(1)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(DOT);
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(tabs, rendering_area);
    });
}
