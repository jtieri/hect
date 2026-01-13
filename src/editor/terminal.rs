use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::terminal::size as raw_size;
use std::io::{Error, Write, stdout};

pub struct Terminal;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16
}

#[derive(Copy, Clone)]
pub struct Position {
   pub x: u16,
   pub y: u16
}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()
    }
    
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }
    
    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.x, pos.y))
    }
    
    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)
    }
    
    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)
    }
    
    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = raw_size()?;
        Ok(Size { width, height})
    }
    
    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }
}
