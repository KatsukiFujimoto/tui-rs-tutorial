use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{BarChart, Block, Borders};
use tui::Terminal;

pub fn render_widget_bar_chart(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let bar_chart = BarChart::default()
            .block(Block::default().title("BarChart").borders(Borders::ALL))
            .bar_width(3)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::Yellow).bg(Color::Red))
            .value_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .label_style(Style::default().fg(Color::White))
            .data(&[("B0", 0), ("B1", 2), ("B2", 4), ("B3", 3)])
            .max(4);
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(bar_chart, rendering_area);
    });
}
