use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;
use tui_rs_tutorial::examples::widget_block;

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    widget_block::render_widget_block(terminal);
    Ok(())
}
