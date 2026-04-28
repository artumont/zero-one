use ratatui::{
	crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
	buffer::Buffer,
	layout::Rect,
	style::{Color, Style},
	text::Text,
	widgets::{Paragraph, StatefulWidget, Widget},
};

#[derive(Debug, Clone, Default)]
pub struct TextInputState {
	pub value: String,
	pub cursor: usize,
	pub scroll_x: u16,
	pub scroll_y: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputAction {
	None,
	Submit(String),
	Exit,
}

impl TextInputState {
	pub fn handle_key(&mut self, key: KeyEvent) -> InputAction {
		match key.code {
			KeyCode::Esc => return InputAction::Exit,
			KeyCode::Enter if key.modifiers.contains(KeyModifiers::SHIFT) => {
				self.insert_char('\n');
			}
			KeyCode::Enter => {
				let submitted = self.value.trim_end().to_owned();
				self.clear();
				return InputAction::Submit(submitted);
			}
			KeyCode::Backspace => {
				self.backspace();
			}
			KeyCode::Delete => {
				self.delete();
			}
			KeyCode::Left => {
				self.move_left();
			}
			KeyCode::Right => {
				self.move_right();
			}
			KeyCode::Home => {
				self.move_to_line_start();
			}
			KeyCode::End => {
				self.move_to_line_end();
			}
			KeyCode::Char(character) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
				self.insert_char(character);
			}
			_ => {}
		}

		InputAction::None
	}

	pub fn clear(&mut self) {
		self.value.clear();
		self.cursor = 0;
		self.scroll_x = 0;
		self.scroll_y = 0;
	}

	fn insert_char(&mut self, character: char) {
		self.value.insert(self.cursor, character);
		self.cursor += character.len_utf8();
	}

	fn backspace(&mut self) {
		if self.cursor == 0 {
			return;
		}

		let previous = previous_char_boundary(&self.value, self.cursor);
		self.value.drain(previous..self.cursor);
		self.cursor = previous;
	}

	fn delete(&mut self) {
		if self.cursor >= self.value.len() {
			return;
		}

		let next = next_char_boundary(&self.value, self.cursor);
		self.value.drain(self.cursor..next);
	}

	fn move_left(&mut self) {
		self.cursor = previous_char_boundary(&self.value, self.cursor);
	}

	fn move_right(&mut self) {
		self.cursor = next_char_boundary(&self.value, self.cursor);
	}

	fn move_to_line_start(&mut self) {
		let line_start = self.value[..self.cursor].rfind('\n').map_or(0, |index| index + 1);
		self.cursor = line_start;
	}

	fn move_to_line_end(&mut self) {
		let line_end = self.value[self.cursor..]
			.find('\n')
			.map_or(self.value.len(), |offset| self.cursor + offset);
		self.cursor = line_end;
	}
}

#[derive(Debug, Clone)]
pub struct TextInput<'a> {
	placeholder: &'a str,
}

impl<'a> TextInput<'a> {
	pub fn new(_title: &'a str) -> Self {
		Self {
			placeholder: "",
		}
	}

	pub fn placeholder(mut self, placeholder: &'a str) -> Self {
		self.placeholder = placeholder;
		self
	}
}

impl StatefulWidget for TextInput<'_> {
	type State = TextInputState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		let inner_width = area.width;
		let inner_height = area.height;

		let (cursor_row, cursor_col) = cursor_position(&state.value, state.cursor);
		if cursor_row < state.scroll_y {
			state.scroll_y = cursor_row;
		} else if cursor_row >= state.scroll_y.saturating_add(inner_height) && inner_height > 0 {
			state.scroll_y = cursor_row.saturating_sub(inner_height.saturating_sub(1));
		}

		if cursor_col < state.scroll_x {
			state.scroll_x = cursor_col;
		} else if cursor_col >= state.scroll_x.saturating_add(inner_width) && inner_width > 0 {
			state.scroll_x = cursor_col.saturating_sub(inner_width.saturating_sub(1));
		}

		let is_empty = state.value.is_empty();

		let content = if is_empty {
			Text::styled(self.placeholder, Style::default().fg(Color::DarkGray))
		} else {
			Text::from(state.value.as_str())
		};

		let paragraph = Paragraph::new(content).scroll((state.scroll_y, state.scroll_x));

		paragraph.render(area, buf);
	}
}

pub fn cursor_position(text: &str, cursor: usize) -> (u16, u16) {
	let prefix = &text[..cursor];
	let row = prefix.lines().count().saturating_sub(1) as u16;
	let col = prefix.rsplit('\n').next().map_or(0, |line| line.chars().count()) as u16;
	(row, col)
}

fn previous_char_boundary(text: &str, index: usize) -> usize {
	text[..index]
		.char_indices()
		.last()
		.map_or(0, |(boundary, _)| boundary)
}

fn next_char_boundary(text: &str, index: usize) -> usize {
	if index >= text.len() {
		return text.len();
	}

	index + text[index..].chars().next().map_or(0, char::len_utf8)
}
