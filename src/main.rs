use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::symbols::Marker::{Braille, Dot};
use tui::symbols::DOT;
use tui::text::{Span, Spans};
use tui::widgets::{
    Axis, BarChart, Block, BorderType, Borders, Cell, Chart, Dataset, Gauge, GraphType, List,
    ListItem, Paragraph, Row, Table, Tabs, Wrap,
};
use tui::Terminal;

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    gauge_sample(terminal);
    Ok(())
}

fn block_sample(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let block = Block::default()
            .title("Block Sample")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style {
                fg: (Some(Color::Red)),
                bg: (Some(Color::Blue)),
                add_modifier: (Modifier::UNDERLINED),
                sub_modifier: (Modifier::empty()),
            });
        // ターミナルの全面サイズを取得する
        // let rendering_area = f.size();
        // f.render_widget(block, rendering_area);
        // ターミナルのサイズを直接指定する
        // let rendering_area_top = Rect::new(0, 0, 20, 10);
        // let rendering_area_bottom = Rect::new(0, 10, 20, 10);
        // f.render_widget(block.clone(), rendering_area_top);
        // f.render_widget(block.clone(), rendering_area_bottom);
        let rendering_areas_left = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            // f.size()とかもあり
            .split(Rect {
                x: 0,
                y: 0,
                width: 50,
                height: 50,
            });
        let rendering_areas_right = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            // f.size()とかもあり
            .split(Rect {
                x: 50,
                y: 0,
                width: 50,
                height: 50,
            });
        f.render_widget(block.clone(), rendering_areas_left[0]);
        f.render_widget(block.clone(), rendering_areas_left[1]);
        f.render_widget(block.clone(), rendering_areas_right[0]);
        f.render_widget(block.clone(), rendering_areas_right[1]);
    });
}

fn tabs_sample(mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>) {
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

fn list_sample(mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>) {
    let _ = terminal.draw(|f| {
        let items = [
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3"),
        ];
        let list = List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(list, rendering_area);
    });
}

fn table_sample(
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

fn paragraph_sample(
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

fn chart_sample(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let datasets = vec![
            Dataset::default()
                .name("data1")
                .marker(Dot)
                .graph_type(GraphType::Scatter)
                .style(Style::default().fg(Color::Cyan))
                .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
            Dataset::default()
                .name("data2")
                .marker(Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Magenta))
                .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
        ];
        let chart = Chart::new(datasets)
            .block(Block::default().title("Chart"))
            .x_axis(
                Axis::default()
                    .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 10.0])
                    .labels(
                        ["0.0", "5.0", "10.0"]
                            .iter()
                            .cloned()
                            .map(Span::from)
                            .collect(),
                    ),
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([0.0, 10.0])
                    .labels(
                        ["0.0", "5.0", "10.0"]
                            .iter()
                            .cloned()
                            .map(Span::from)
                            .collect(),
                    ),
            );
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(chart, rendering_area);
    });
}

fn bar_chart_sample(
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

fn gauge_sample(
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
