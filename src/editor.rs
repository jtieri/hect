use crossterm::{event::{Event::{self, Key}, KeyCode, KeyEvent, KeyModifiers, read}};
use std::{io::Error};
use terminal::{Terminal, Size, Position};

mod terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    position: Position
}

impl Editor {
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
        let Size{ height, width } = Terminal::size().unwrap(); 
    
        if let Key(KeyEvent { code, modifiers, kind: _kind, state: _state }) = event {
            match code {
                KeyCode::Left => {
                    // if we are at the left edge of the screen we don't update the position
                    if self.position.column > 0 {
                        self.position = Position { column: self.position.column-1, row: self.position.row };
                    } 
                }
                KeyCode::Right => {
                    // if we are at the right edge of the screen we don't update the position
                    if self.position.column < width-1 {
                        self.position = Position { column: self.position.column+1, row: self.position.row};
                    }
                }
                KeyCode::Up => {
                    // if we are at the top of the screen we don't update the position
                    if self.position.row > 0 {
                        self.position = Position { column: self.position.column, row: self.position.row-1 };
                    } 
                }
                KeyCode::Down => {
                    // if we are at the bottom of the screen we don't update the position
                    if self.position.row < height-1 {
                        self.position = Position { column: self.position.column, row: self.position.row+1 };
                    } 
                }
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::PageUp => {
                    self.position = Position { column: self.position.column, row: 0 };
                }
                KeyCode::PageDown => {
                    self.position = Position { column: self.position.column, row: height-1 };
                }
                KeyCode::Home => {
                    self.position = Position { column: 0, row: self.position.row };
                }
                KeyCode::End => {
                    self.position = Position { column: width-1, row: self.position.row };
                }
                _ => ()
            }
        } 
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(self.position)?;
        }   
        
        Terminal::show_caret()?;
        Terminal::execute()
    }
    
    fn draw_welcome_message() -> Result<(), Error> {
            let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
            let width = Terminal::size()?.width;
            let len = welcome_message.len();
            
            // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
            // it's allowed to be a bit to the left or right.
            #[allow(clippy::integer_division)]
            let padding = (width.saturating_sub(len)) / 2;
            let spaces = " ".repeat(padding.saturating_sub(1));
            welcome_message = format!("~{spaces}{welcome_message}");
            welcome_message.truncate(width);
            Terminal::print(welcome_message)?;
            Ok(())
        }
    
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }
    
    fn draw_rows() -> Result<(), Error> {
            let Size { height, .. } = Terminal::size()?;
            for current_row in 0..height {
                Terminal::clear_line()?;
                // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
                // it's allowed to be a bit up or down
                #[allow(clippy::integer_division)]
                if current_row == height / 3 {
                    Self::draw_welcome_message()?;
                } else {
                    Self::draw_empty_row()?;
                }
                if current_row.saturating_add(1) < height {
                    Terminal::print("\r\n")?;
                }
            }
            Ok(())
        }
}
