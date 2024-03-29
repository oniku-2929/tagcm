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
    Ok(results)
}

struct App<T>
where
    T: TagDataRepository,
{
    input: String,
    cursor_input_position: usize,
    cursor_commnad_position: usize,
    suggestions: Vec<TagData>,
    repo: T,
}

impl<T: TagDataRepository> Default for App<T>
where
    T: TagDataRepository,
{
    fn default() -> App<T> {
        App {
            input: String::new(),
            suggestions: Vec::new(),
            cursor_input_position: 0,
            cursor_commnad_position: 0,
            repo: TagDataRepository::new(),
        }
    }
}

impl<T: TagDataRepository> App<T> {
    fn set_repo(&mut self, repo: T) {
        self.repo = repo;
    }

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

    fn auto_complete(&mut self) {
        self.suggestions.clear();
        self.cursor_commnad_position = 0;
        let tags = search(&self.repo, self.input.clone()).unwrap();
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

pub fn search_by_input<T: TagDataRepository>(repo: T) -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::<T>::default();
    app.set_repo(repo);

    run_app(&mut terminal, app)?;

    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn run_app<B: Backend, T: TagDataRepository>(
    terminal: &mut Terminal<B>,
    mut app: App<T>,
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
                        app.auto_complete();
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                        app.auto_complete();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left(1);
                    }
                    KeyCode::Right => {
                        app.move_cursor_right(1);
                    }
                    KeyCode::Tab => {
                        app.auto_complete();
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

fn render<T: TagDataRepository>(f: &mut Frame, app: &App<T>) {
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
