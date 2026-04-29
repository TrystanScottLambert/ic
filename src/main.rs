use crossterm::cursor::{self, MoveLeft};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    self,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use yarer::session::{self, Session};

use std::collections::HashMap;
use std::io::{self, Cursor, Stdout, stdout};

mod parser;
use crate::parser::sanitize_string;

fn print_in_prompt(stdout: &mut Stdout, counter: i32) {
    execute!(stdout, Print(format!("In [{counter}]: ").green().bold())).unwrap()
}

fn print_out_prompt(stdout: &mut Stdout, counter: i32) {
    execute!(stdout, Print(format!("Out[{counter}]: ").red().bold())).unwrap()
}

fn print_color_string(stdout: &mut Stdout, buffer_string: String) {
    for c in buffer_string.chars() {
        if c.is_numeric() || c == '.' {
            execute!(stdout, Print(format!("{}", c.green()))).unwrap();
        } else {
            execute!(stdout, Print(c)).unwrap();
        }
    }
}

fn reset_prompt(stdout: &mut Stdout, counter: i32) {
    execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    execute!(stdout, Print("\r")).unwrap();
    print_in_prompt(stdout, counter);
}

fn main() {
    let mut stdout = std::io::stdout();
    let mut buffer = String::new();
    let start_cursor_position: usize = 5;
    let mut cursor_position: usize = start_cursor;
    let mut history = HashMap::new();
    let mut history_counter = 0;
    enable_raw_mode().unwrap();
    let mut counter = 1;
    execute!(stdout, Clear(ClearType::All)).unwrap();
    execute!(stdout, Print("\n\r")).unwrap();

    print_in_prompt(&mut stdout, counter);
    loop {
        let thing = event::read().unwrap();
        match thing {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => break,
                KeyCode::Char(c) => {
                    if c.is_numeric() || c == '.' {
                        execute!(stdout, Print(format!("{}", c.green()))).unwrap();
                    } else {
                        execute!(stdout, Print(c)).unwrap();
                    }
                    buffer.insert(cursor_position, c);
                    cursor_position += 1;
                }
                KeyCode::Enter => {
                    execute!(stdout, Print("\r\n")).unwrap();

                    if buffer.trim().is_empty() {
                        execute!(stdout, Print("\r\n")).unwrap();
                        print_in_prompt(&mut stdout, counter);
                    } else if buffer == "clear" {
                        execute!(stdout, Clear(ClearType::All)).unwrap();
                        print_in_prompt(&mut stdout, counter);
                        buffer.clear();
                    } else {
                        print_out_prompt(&mut stdout, counter);
                        let session = Session::init();
                        let sanitized_string = sanitize_string(&buffer);
                        let mut resolver = session.process(&sanitized_string);
                        // session.set("x", 1) // we can add variables in the future
                        let answer = resolver.resolve();
                        if let Ok(num) = answer {
                            execute!(stdout, Print(format!("{}", num))).unwrap();
                        } else {
                            execute!(stdout, Print("\n\rError reading. Try again.")).unwrap();
                        }
                        history.insert(counter, buffer.clone());
                        history_counter = 0;

                        buffer.clear();
                        execute!(stdout, Print("\r\n")).unwrap();
                        execute!(stdout, Print("\r\n")).unwrap();
                        counter += 1;
                        print_in_prompt(&mut stdout, counter);
                    }
                }
                KeyCode::Up => {
                    if (buffer.is_empty() || history_counter != 0)
                        && (history_counter != counter - 1)
                    {
                        history_counter += 1;
                        let index = counter - history_counter;
                        buffer = history.get(&(index)).unwrap().to_string();
                        reset_prompt(&mut stdout, counter);
                        print_color_string(&mut stdout, buffer.clone());
                    }
                }
                KeyCode::Down => {
                    if history_counter > 1 {
                        history_counter -= 1;
                        let index = counter - history_counter;
                        buffer = history.get(&(index)).unwrap().to_string();
                        reset_prompt(&mut stdout, counter);
                        print_color_string(&mut stdout, buffer.clone());
                    } else {
                        history_counter = 0;
                        buffer.clear();
                        reset_prompt(&mut stdout, counter);
                        execute!(stdout, Print("")).unwrap();
                    }
                }
                KeyCode::Backspace => {
                    buffer.pop();
                    reset_prompt(&mut stdout, counter);
                    print_color_string(&mut stdout, buffer.clone());
                }
                KeyCode::Home => {
                    let (_, current_column) = cursor::position().unwrap();
                    let offset = current_column - start_cursor_position;

                    execute!(stdout, Cursor(MoveLeft(offset))).unwrap();
                }
                _ => {}
            },
            _ => todo!(),
        }
    }
    disable_raw_mode().unwrap();
}
