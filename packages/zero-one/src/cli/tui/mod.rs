use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    crossterm,
};

mod widgets;

use widgets::text_input::{InputAction, TextInput, TextInputState};

/// The main entry point for running the TUI interface. This function initializes the terminal and starts the application loop.
pub fn run_tui() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

/// The main application loop that handles rendering and user input.
fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut input = TextInputState::default();
    let mut messages: Vec<String> = Vec::new();

    loop {
        terminal.draw(|frame| render(frame, &messages, &mut input))?;

        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        break Ok(());
                    }

                    match input.handle_key(key) {
                        InputAction::None => {}
                        InputAction::Submit(message) => {
                            if !message.trim().is_empty() {
                                messages.push(message);
                            }
                        }
                        InputAction::Exit => break Ok(()),
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }
}

/// The rendering function that draws the TUI interface. This is where you can add your widgets and layout.
fn render(frame: &mut Frame, messages: &[String], input: &mut TextInputState) {
    let layout = Layout::vertical([Constraint::Min(5), Constraint::Length(8)])
        .split(frame.area());

    let chat_text = if messages.is_empty() {
        "No messages yet. Start the conversation below.".to_owned()
    } else {
        messages.join("\n\n")
    };

    let history = Paragraph::new(chat_text)
        .block(Block::default().borders(Borders::ALL).title("Conversation"));
    frame.render_widget(history, layout[0]);

    let composer_block = Block::default()
        .borders(Borders::ALL)
        .title("Message")
        .border_style(Style::default().fg(Color::Cyan));
    let composer_inner = composer_block.inner(layout[1]);
    frame.render_widget(composer_block, layout[1]);
    let composer_layout = Layout::vertical([Constraint::Min(4), Constraint::Length(1), Constraint::Length(1)])
        .split(composer_inner);

    let composer = TextInput::new("Message")
        .placeholder("Ask something, press Enter to send");
    frame.render_stateful_widget(composer, composer_layout[0], input);

    if !input.value.is_empty() {
        let (cursor_row, cursor_col) = widgets::text_input::cursor_position(&input.value, input.cursor);
        let cursor_x = composer_layout[0].x + cursor_col.saturating_sub(input.scroll_x);
        let cursor_y = composer_layout[0].y + cursor_row.saturating_sub(input.scroll_y);
        frame.set_cursor_position((cursor_x, cursor_y));
    }

    let chips = Line::from(vec![
        Span::styled(" Build ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(" AI model ", Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(" zero-one ", Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]);
    let chips_row = Paragraph::new(chips);
    frame.render_widget(chips_row, composer_layout[1]);

    let help = Paragraph::new("Future model picker base")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, composer_layout[2]);
}
