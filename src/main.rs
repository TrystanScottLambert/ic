use crossterm::cursor::{Hide, MoveLeft, MoveRight, RestorePosition, SavePosition, Show, position};
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    self,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use yarer::session::Session;

use std::collections::HashMap;
use std::io::{self, Stdout};

mod parser;
use crate::parser::sanitize_string;

fn print_in_prompt(stdout: &mut Stdout, counter: i32) -> io::Result<()> {
    execute!(stdout, Print(format!("In [{counter}]: ").green().bold()))
}

fn print_out_prompt(stdout: &mut Stdout, counter: i32) -> io::Result<()> {
    execute!(stdout, Print(format!("Out[{counter}]: ").red().bold()))
}

fn print_color_string(stdout: &mut Stdout, buffer_string: String) -> io::Result<()> {
    for c in buffer_string.chars() {
        if c.is_numeric() || c == '.' {
            execute!(stdout, Print(format!("{}", c.green())))?
        } else {
            execute!(stdout, Print(c))?
        }
    }
    Ok(())
}

fn reset_prompt(stdout: &mut Stdout, counter: i32) -> io::Result<()> {
    execute!(stdout, Clear(ClearType::CurrentLine))?;
    execute!(stdout, Print("\r"))?;
    print_in_prompt(stdout, counter)
}

fn main() -> io::Result<()> {
    let mut stdout = std::io::stdout();
    let mut buffer = String::new();
    let mut history = HashMap::new();
    let mut history_counter = 0;
    let start_line_position = 8;
    enable_raw_mode()?;
    let mut counter = 1;
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, Print("\n\r"))?;
    print_in_prompt(&mut stdout, counter)?;
    loop {
        let thing = event::read()?;
        match thing {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc => break,
                KeyCode::Char(c) => {
                    let (column, _) = position()?;
                    let target_position = column - start_line_position;
                    let (pre, post) = buffer.split_at(target_position as usize);
                    execute!(stdout, SavePosition)?;
                    execute!(stdout, Hide)?;
                    execute!(stdout, Clear(ClearType::UntilNewLine))?;
                    if c.is_numeric() || c == '.' {
                        execute!(stdout, Print(format!("{}", c.green())))?;
                    } else {
                        execute!(stdout, Print(c))?;
                    }
                    for c in post.chars() {
                        if c.is_numeric() || c == '.' {
                            execute!(stdout, Print(format!("{}", c.green())))?;
                        } else {
                            execute!(stdout, Print(c))?;
                        }
                    }
                    execute!(stdout, RestorePosition)?;
                    execute!(stdout, MoveRight(1))?;
                    execute!(stdout, Show)?;
                    buffer = format!("{}{}{}", pre, c, post);
                }
                KeyCode::Enter => {
                    execute!(stdout, Print("\r\n"))?;

                    if buffer.trim().is_empty() {
                        execute!(stdout, Print("\r\n"))?;
                        print_in_prompt(&mut stdout, counter)?;
                    } else if buffer == "clear" {
                        execute!(stdout, Clear(ClearType::All))?;
                        print_in_prompt(&mut stdout, counter)?;
                        buffer.clear();
                    } else if buffer.trim() == "exit" {
                        break;
                    } else {
                        print_out_prompt(&mut stdout, counter)?;
                        let session = Session::init();
                        let sanitized_string = sanitize_string(&buffer);
                        let mut resolver = session.process(&sanitized_string);
                        // session.set("x", 1) // we can add variables in the future
                        let answer = resolver.resolve();
                        if let Ok(num) = answer {
                            execute!(stdout, Print(format!("{}", num)))?;
                        } else {
                            execute!(stdout, Print("\n\rError reading. Try again."))?;
                        }
                        history.insert(counter, buffer.clone());
                        history_counter = 0;

                        buffer.clear();
                        execute!(stdout, Print("\r\n"))?;
                        execute!(stdout, Print("\r\n"))?;
                        counter += 1;
                        print_in_prompt(&mut stdout, counter)?;
                    }
                }
                KeyCode::Up => {
                    if (buffer.is_empty() || history_counter != 0)
                        && (history_counter != counter - 1)
                    {
                        history_counter += 1;
                        let index = counter - history_counter;
                        buffer = history.get(&(index)).unwrap().to_string();
                        reset_prompt(&mut stdout, counter)?;
                        print_color_string(&mut stdout, buffer.clone())?;
                    }
                }
                KeyCode::Down => {
                    if history_counter > 1 {
                        history_counter -= 1;
                        let index = counter - history_counter;
                        buffer = history.get(&(index)).unwrap().to_string();
                        reset_prompt(&mut stdout, counter)?;
                        print_color_string(&mut stdout, buffer.clone())?;
                    } else {
                        history_counter = 0;
                        buffer.clear();
                        reset_prompt(&mut stdout, counter)?;
                        execute!(stdout, Print(""))?;
                    }
                }
                KeyCode::Backspace => {
                    let (column, _) = position()?;
                    if column > start_line_position {
                        let target_position = column - start_line_position - 1;
                        execute!(stdout, SavePosition)?;
                        buffer.remove(target_position as usize);

                        reset_prompt(&mut stdout, counter)?;
                        print_color_string(&mut stdout, buffer.clone())?;
                        execute!(stdout, RestorePosition)?;
                        execute!(stdout, MoveLeft(1))?;
                    }
                }
                KeyCode::Left => {
                    let (column, _) = position()?;
                    if column > start_line_position {
                        execute!(stdout, MoveLeft(1))?;
                    }
                }
                KeyCode::Right => {
                    let (column, _) = position()?;
                    if column < (buffer.len() as u16) + start_line_position {
                        execute!(stdout, MoveRight(1))?;
                    }
                }
                _ => {}
            },
            _ => todo!(),
        }
    }
    disable_raw_mode()?;
    Ok(())
}
