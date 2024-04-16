use crate::repo::tag_data_repository::TagDataRepository;
use anyhow::Result;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use cli_clipboard;
use ratatui::{prelude::*, widgets::*};
use std::io;
use std::io::stdout;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TagData {
    pub tag: String,
    pub command: String,
}

impl TagData {
    fn new(tag: String, command: String) -> TagData {
        TagData { tag, command }
    }
}

pub fn search<T: TagDataRepository>(repo: &T, search_str: String) -> Result<Vec<TagData>> {
    let all_tags = repo.get_all_tags();

    let mut results: Vec<TagData> = Vec::new();
    for tag in all_tags {
        if tag.starts_with(&search_str) {
            match repo.get_tag_data(&tag) {
                Some(cmd) => {
                    results.push(TagData::new(tag, cmd));
                }
                None => println!("Command not found"),
            }
        }
    }
    results.sort();
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::unittest_repository::UnitTestRepository;

    #[test]
    fn test_search_single_tag() {
        let mut repo = UnitTestRepository::new();
        repo.add_tag_data("test".to_string(), "echo test".to_string());
        let result = search(&repo, "test".to_string()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].tag, "test");
        assert_eq!(result[0].command, "echo test");

        repo.remove_tag_data("test");
        let result = search(&repo, "test".to_string()).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_search_different_prefix() {
        let mut repo = UnitTestRepository::new();
        repo.add_tag_data("test".to_string(), "echo test".to_string());
        repo.add_tag_data("hoge".to_string(), "echo hoge".to_string());
        let result = search(&repo, "test".to_string()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].tag, "test");
        assert_eq!(result[0].command, "echo test");

        repo.remove_tag_data("test");
        repo.remove_tag_data("hoge");
        let result = search(&repo, "test".to_string()).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_search_same_prefix() {
        let mut repo = UnitTestRepository::new();
        repo.add_tag_data("test".to_string(), "echo test".to_string());
        repo.add_tag_data("test2".to_string(), "echo test2".to_string());
        let result = search(&repo, "test".to_string()).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].tag, "test");
        assert_eq!(result[0].command, "echo test");
        assert_eq!(result[1].tag, "test2");
        assert_eq!(result[1].command, "echo test2");

        repo.remove_tag_data("test");
        let mut result = search(&repo, "test".to_string()).unwrap();
        assert_eq!(result.len(), 1);

        repo.remove_tag_data("hoge");
        result = search(&repo, "hoge".to_string()).unwrap();
        assert_eq!(result.len(), 0);
    }
}

#[derive(Default)]
struct App {
    input: String,
    cursor_input_position: usize,
    cursor_commnad_position: usize,
    suggestions: Vec<TagData>,
}

impl App {
    fn move_cursor_left(&mut self, size: usize) {
        let cursor_moved_left = self.cursor_input_position.saturating_sub(size);
        self.cursor_input_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self, size: usize) {
        let cursor_moved_right = self.cursor_input_position.saturating_add(size);
        self.cursor_input_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_input_position, new_char);
        self.move_cursor_right(1);
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_input_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_input_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left(1);
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn choose_suggestion(&mut self) {
        cli_clipboard::set_contents(
            self.suggestions[self.cursor_commnad_position]
                .command
                .clone(),
        )
        .unwrap();
    }

    fn auto_complete<T: TagDataRepository>(&mut self, repo: &T) {
        self.suggestions.clear();
        self.cursor_commnad_position = 0;
        let tags = search(repo, self.input.clone()).unwrap();
        for tag in tags {
            self.suggestions.push(tag);
        }
    }

    pub fn get_current_command_input(&self) -> usize {
        self.cursor_commnad_position
    }

    pub fn add_current_command_input(&mut self, add: i32) {
        if self.suggestions.is_empty() {
            self.cursor_commnad_position = 0;
            return;
        }

        if add > 0 {
            if self.suggestions.len() - 1 <= self.cursor_commnad_position {
                self.cursor_commnad_position = 0;
                return;
            }
            self.cursor_commnad_position += 1;
        } else {
            if self.cursor_commnad_position == 0 {
                self.cursor_commnad_position = self.suggestions.len() - 1;
                return;
            }
            self.cursor_commnad_position -= 1;
        }
    }
}

pub fn search_by_input<T: TagDataRepository>(repo: &T) -> Result<()>
where
    T: TagDataRepository,
{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let app = App::default();

    run_app(&mut terminal, app, repo)?;

    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn run_app<B: Backend, T: TagDataRepository>(
    terminal: &mut Terminal<B>,
    mut app: App,
    repo: &T,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        if !app.suggestions.is_empty() {
                            app.choose_suggestion();
                            return Ok(());
                        }
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                        app.auto_complete(repo);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                        app.auto_complete(repo);
                    }
                    KeyCode::Left => {
                        app.move_cursor_left(1);
                    }
                    KeyCode::Right => {
                        app.move_cursor_right(1);
                    }
                    KeyCode::Tab => {
                        app.auto_complete(repo);
                    }
                    KeyCode::Down => {
                        app.add_current_command_input(1);
                    }
                    KeyCode::Up => {
                        app.add_current_command_input(-1);
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}

const TITLE_INPUT: &str = "Input";
const TITLE_RESULT: &str = "Search results";

fn render(f: &mut Frame, app: &App) {
    let text = vec![
        Line::from(vec![
            Span::styled("Press any key:", Style::new().bold()),
            Span::raw("to start auto-complete tag and command,"),
            ".".into(),
        ]),
        Line::from(vec![
            Span::styled("key Left, key Right:", Style::new().bold()),
            Span::raw("move cursor in INPUT window"),
            ".".into(),
        ]),
        Line::from(vec![
            Span::styled("key Up, key Down:", Style::new().bold()),
            Span::raw("move cursor in Search results window"),
            ".".into(),
        ]),
        Line::from(vec![
            Span::styled("Enter:", Style::new().bold()),
            Span::raw("to choose the command to clipboard and exit search mode"),
            ".".into(),
        ]),
        Line::from(vec![
            Span::styled("Esc:", Style::new().bold()),
            Span::raw("to exit search mode"),
            ".".into(),
        ]),
    ];

    let vertical = Layout::vertical([
        Constraint::Length(text.len() as u16),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);
    let [help_area, input_area, messages_area] = vertical.areas(f.size());
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title(TITLE_INPUT));
    f.render_widget(input, input_area);
    f.set_cursor(
        input_area.x + app.cursor_input_position as u16 + 1,
        input_area.y + 1,
    );

    let messages: Vec<ListItem> = app
        .suggestions
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = if i == app.get_current_command_input() {
                Line::from(Span::raw(format!("{0}: {1}", m.tag, m.command)).on_white())
            } else {
                Line::from(Span::raw(format!("{0}: {1}", m.tag, m.command)))
            };

            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title(TITLE_RESULT));
    f.render_widget(messages, messages_area);
}
