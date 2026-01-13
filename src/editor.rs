use crossterm::{event::{Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers, read}};
use std::io::Error;
use terminal::{Terminal, Size, Position};

mod terminal;

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
    
    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
