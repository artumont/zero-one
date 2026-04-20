use ratatui::{DefaultTerminal, Frame, crossterm};

/// The main entry point for running the TUI interface. This function initializes the terminal and starts the application loop.
pub fn run_tui() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

/// The main application loop that handles rendering and user input.
fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

/// The rendering function that draws the TUI interface. This is where you can add your widgets and layout.
/// TODO: Replace the placeholder rendering logic with actual widgets and layout for the application.
fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
