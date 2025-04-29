use std::{
    io::{self, Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{self, MoveDown, MoveTo, MoveToNextLine, MoveUp, position},
    event::{Event, KeyCode, poll, read},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, SetUnderlineColor},
    terminal::{Clear, ClearType},
};

use crate::config::ShellItem;

/// Writes the shell list to `stdout`.
/// 
/// ## Arguments
/// 
/// * `stdout` - The standard output stream.
/// * `shell_list` - The list of shells to display.
pub fn write_shell_list(
    stdout: &mut Stdout,
    shell_list: &Vec<ShellItem>,
) -> Result<(), io::Error> {
    queue!(
        stdout,
        cursor::Show,
        cursor::EnableBlinking,
        MoveTo(0, 0),
        Clear(ClearType::FromCursorDown)
    )?;

    queue!(
        stdout,
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        SetUnderlineColor(Color::White),
        Print("Select a shell:"),
        ResetColor,
        MoveToNextLine(1)
    )?;

    for shell in shell_list {
        queue!(
            stdout,
            Print(format!("[ ] ")),
            SetForegroundColor(Color::Blue),
            Print(format!("{}", shell.name)),
            ResetColor,
            Print(format!(" - ")),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("{:?}", shell.path)),
            ResetColor,
            MoveToNextLine(1)
        )?;
    }

    queue!(stdout, MoveTo(1, 1))?;

    stdout.flush()?;

    Ok(())
}

/// Run the shell picker.
/// 
/// ## Arguments
/// 
/// * `stdout` - The standard output stream.
/// * `shell_list` - The list of shells to display.
pub fn run_shell_picker(
    stdout: &mut Stdout,
    shell_list: &Vec<ShellItem>,
) -> Result<usize, io::Error> {
    let mut selected_index = 0;
    let mut termination_requested = false;

    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Up => {
                        let cursor_position = position()?;
                        let current_row = cursor_position.1 as usize;

                        if current_row > 1 {
                            execute!(stdout, MoveUp(1),)?;
                        }
                        else {
                            let last_row = shell_list.len() - 1;
                            execute!(stdout, MoveDown(last_row as u16))?;
                        }
                    }

                    KeyCode::Down => {
                        let cursor_position = position()?;
                        let current_row = cursor_position.1 as usize;

                        if current_row - 1 < shell_list.len() - 1 {
                            execute!(stdout, MoveDown(1),)?;
                        }
                        else {
                            let first_row = shell_list.len() - 1;
                            execute!(stdout, MoveUp(first_row as u16))?;
                        }
                    }

                    KeyCode::Enter => {
                        let cursor_position = position()?;

                        selected_index = cursor_position.1 as usize;
                        break;
                    }

                    KeyCode::Esc => {
                        termination_requested = true;
                        break;
                    }

                    KeyCode::Char('q') => {
                        termination_requested = true;
                        break;
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }

    queue!(stdout, MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;

    stdout.flush()?;

    if termination_requested {
        std::process::exit(0);
    }

    Ok(selected_index)
}
