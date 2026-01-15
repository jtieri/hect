use crossterm::{event::{Event::{self, Key}, KeyCode, KeyEvent, KeyModifiers, read}};
use std::{io::Error};
use terminal::{Terminal, Size, Position};

mod terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    position: Position
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false, position: Position { x: 0, y: 0 } }
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
        let Size{ height, width } = Terminal::size().unwrap(); 
    
        if let Key(KeyEvent { code, modifiers, kind: _kind, state: _state }) = event {
            match code {
                KeyCode::Left => {
                    // if we are at the left edge of the screen we don't update the position
                    if self.position.x > 0 {
                        self.position = Position { x: self.position.x-1, y: self.position.y };
                    } 
                }
                KeyCode::Right => {
                    // if we are at the right edge of the screen we don't update the position
                    if self.position.x < width-1 {
                        self.position = Position { x: self.position.x+1, y: self.position.y};
                    }
                }
                KeyCode::Up => {
                    // if we are at the top of the screen we don't update the position
                    if self.position.y > 0 {
                        self.position = Position { x: self.position.x, y: self.position.y-1 };
                    } 
                }
                KeyCode::Down => {
                    // if we are at the bottom of the screen we don't update the position
                    if self.position.y < height-1 {
                        self.position = Position { x: self.position.x, y: self.position.y+1 };
                    } 
                }
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::PageUp => {
                    self.position = Position { x: self.position.x, y: 0 };
                }
                KeyCode::PageDown => {
                    self.position = Position { x: self.position.x, y: height-1 };
                }
                KeyCode::Home => {
                    self.position = Position { x: 0, y: self.position.y };
                }
                KeyCode::End => {
                    self.position = Position { x: width-1, y: self.position.y };
                }
                _ => ()
            }
        } 
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(self.position)?;
        }   
        
        Terminal::show_cursor()?;
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
