use tui::backend::TermionBackend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};
use tui::Terminal;

pub fn render_widget_paragraph(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let text = vec![
            Spans::from(vec![
                Span::raw("First"),
                Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
                Span::raw("."),
            ]),
            Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
        ];
        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(paragraph, rendering_area);
    });
}
