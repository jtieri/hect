use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use std::io::stdout;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_display()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_display() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        Self::draw_rows()?;
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {code: Char('q'), modifiers: KeyModifiers::CONTROL, ..}) = event
        {
            self.should_quit = true;
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_display()?;
            let mut stdout = stdout();
            execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
    
    fn draw_rows() -> Result<(), std::io::Error> {
        let size = crossterm::terminal::size()?;
        for _ in 0..=size.1 {
            println!("~\r");
        }
        println!("~\r");
        Ok(())
    }
}
