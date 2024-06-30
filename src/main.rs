// https://www.rust-lang.org/learn/get-started
// https://ratatui.rs/tutorials/hello-world/
// https://ratatui.rs/tutorials/counter-app/basic-app/

// use crossterm::{
//     event::{self, KeyCode, KeyEventKind},
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//     ExecutableCommand,
// };
// use ratatui::{
//     prelude::{CrosstermBackend, Stylize, Terminal},
//     widgets::Paragraph,
// };
// use std::io;


// fn main() -> io::Result<()> {
//     io::stdout().execute(EnterAlternateScreen)?;
//     enable_raw_mode()?;

//     let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
//     terminal.clear()?;

//     loop {
//         terminal.draw(|frame| {
//             let area = frame.size();
//             frame.render_widget(
//                 Paragraph::new("Hello Ratatui! (press 'q' to quit)")
//                     .white()
//                     .on_blue(),
//                 area,
//             );
//         })?;
        
//         if event::poll(std::time::Duration::from_millis(16))? {
//             if let event::Event::Key(key) = event::read()? {
//                 if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
//                     break;
//                 }
//             }
//         }
//     }

//     io::stdout().execute(LeaveAlternateScreen)?;
//     disable_raw_mode()?;
//     Ok(())
// }

mod tui;

use std::io;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
