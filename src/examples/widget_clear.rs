use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Clear};
use tui::Terminal;

pub fn render_widget_clear(
    mut terminal: Terminal<TermionBackend<termion::raw::RawTerminal<std::io::Stdout>>>,
) {
    let _ = terminal.draw(|f| {
        let block = Block::default().title("Block").borders(Borders::ALL);
        let rendering_area = Rect::new(0, 0, 50, 10);
        f.render_widget(block, rendering_area); // now render the block widget
        f.render_widget(Clear, rendering_area); // <- this will clear/reset the area
    });
}
