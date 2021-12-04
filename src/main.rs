use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::symbols::DOT;
use tui::text::Spans;
use tui::widgets::{Block, BorderType, Borders, Tabs};
use tui::Terminal;

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    tabs_sample(terminal);
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
