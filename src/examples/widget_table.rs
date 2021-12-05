use tui::backend::TermionBackend;
use tui::layout::{Constraint, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::Terminal;

pub fn render_widget_table(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let table = Table::new(vec![
            // Row can be created from simple strings.
            Row::new(vec!["Row11", "Row12", "Row13"]),
            // You can style the entire row.
            Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
            // If you need more control over the styling you may need to create Cells directly
            Row::new(vec![
                Cell::from("Row31"),
                Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                Cell::from(Spans::from(vec![
                    Span::raw("Row"),
                    Span::styled("33", Style::default().fg(Color::Green)),
                ])),
            ]),
            // If a Row need to display some content over multiple lines, you just have to change
            // its height.
            Row::new(vec![
                Cell::from("Row\n41"),
                Cell::from("Row\n42"),
                Cell::from("Row\n43"),
            ])
            .height(2),
        ])
        // You can set the style of the entire Table.
        .style(Style::default().fg(Color::White))
        // It has an optional header, which is simply a Row always visible at the top.
        .header(
            Row::new(vec!["Col1", "Col2", "Col3"])
                .style(Style::default().fg(Color::Yellow))
                // If you want some space between the header and the rest of the rows, you can always
                // specify some margin at the bottom.
                .bottom_margin(1),
        )
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::default().title("Table").borders(Borders::ALL))
        // Columns widths are constrained in the same way as Layout...
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(5),
            Constraint::Length(10),
        ])
        // ...and they can be separated by a fixed spacing.
        .column_spacing(1)
        // If you wish to highlight a row in any specific way when it is selected...
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(table, rendering_area);
    });
}
