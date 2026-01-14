use crossterm::{event::{Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers, read}};
use std::{fmt::format, io::Error};
use terminal::{Terminal, Size, Position};

mod terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {code: Char('q'), modifiers: KeyModifiers::CONTROL, ..}) = event
        {
            self.should_quit = true;
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x: 0, y: 0})?;
        }   
        
        Terminal::show_cursor()?;
        Terminal::execute()
    }
    
    // fn draw_welcome_message() -> Result<(), Error> { 
    //     let mut message = format!("{NAME} editor -- version {VERSION}");
    //     let width = Terminal::size()?.width as usize;
    //     let len = message.len();
    //     let padding = (width - len) / 2;
    //     let spaces = " ".repeat(padding - 1);
    //     message = format!("~{spaces}{message}");
    //     message.truncate(width);
    //     Terminal::print(&message)
    // } 
    
    fn draw_welcome_message() -> Result<(), Error> { 
        let message = format!("{NAME} editor -- version {VERSION}");
        Terminal::print(&message)
    } 
    
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }
    
    fn draw_rows() -> Result<(), Error> {
        let Size{height, width} = Terminal::size()?;
        let middle_height = height / 3;
        let center_row = width / 2;
        
        for current_row in 0..height {
            Terminal::clear_line()?;
            Self::draw_empty_row()?;
            
            if current_row ==  middle_height {
                Terminal::move_cursor_to(Position { x: center_row, y: middle_height })?;
                Self::draw_welcome_message()?;
            }
            
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
