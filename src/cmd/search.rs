use crate::repo::{hashmap_repository::HashMapRepository, tag_data_repository::TagDataRepository};
use anyhow::Result;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};
use std::io;
use std::io::stdout;

pub fn search<T: TagDataRepository>(repo: T, search_str: String) -> Result<()> {
    println!("Searching for tag {}", search_str);
    let all_tags = repo.get_all_tags();

    for tag in all_tags {
        if tag.starts_with(&search_str) {
            match repo.get_tag_data(&tag) {
                Some(cmd) => {
                    println!("{}: {}", tag, cmd);
                }
                None => println!("Command not found"),
            }
        }
    }
    Ok(())
}

struct App<T>
where
    T: TagDataRepository,
{
    input: String,
    cursor_position: usize,
    messages: Vec<String>,
    repo: T,
}

impl<T: TagDataRepository> Default for App<T>
where
    T: TagDataRepository,
{
    fn default() -> App<T> {
        App {
            input: String::new(),
            messages: Vec::new(),
            cursor_position: 0,
            repo: TagDataRepository::new(),
        }
    }
}

impl<T: TagDataRepository> App<T> {
    fn set_repo(&mut self, repo: T) {
        self.repo = repo;
    }

    fn move_cursor_left(&mut self, size: usize) {
        let cursor_moved_left = self.cursor_position.saturating_sub(size);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self, size: usize) {
        let cursor_moved_right = self.cursor_position.saturating_add(size);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right(1);
    }

    fn enter_str(&mut self, new_str: &str) {
        self.input.insert_str(self.cursor_position, new_str);
        self.move_cursor_right(new_str.len());
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left(1);
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    fn auto_complete(&mut self) {
        self.messages.push(self.repo.get_all_tags().concat());
    }
}

pub fn search_by_input<T: TagDataRepository>(repo: T) -> Result<()> {
    let all_tags = repo.get_all_tags();
    for tag in all_tags {
        match repo.get_tag_data(&tag) {
            Some(cmd) => {
                println!("{}: {}", tag, cmd);
            }
            None => println!("Command not found"),
        }
    }

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::<T>::default();
    app.set_repo(repo);

    let res = run_app(&mut terminal, app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

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
                    KeyCode::Enter => app.submit_message(),
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
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
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}

fn render<T: TagDataRepository>(f: &mut Frame, app: &App<T>) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);
    let [help_area, input_area, messages_area] = vertical.areas(f.size());

    let msg = vec![
        "Press ".into(),
        "tab".bold(),
        " to auto complete tag and command, ".into(),
        "esc".bold(),
        " to exit seatch mode.".bold(),
    ];
    let text = Text::from(Line::from(msg)).patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, input_area);    
    f.set_cursor(
        input_area.x + app.cursor_position as u16 + 1,
        input_area.y + 1,
    );

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{i}: {m}")));
            ListItem::new(content)
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, messages_area);
}
