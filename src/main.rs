extern crate core;

mod ui;
pub mod utils;
pub mod ttyama_app;

use std::{env, io};
use tokio::time::{sleep, Duration};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use ui::*;
use crate::ttyama_app::{TTYama};
use crate::ui::button::Button;
use crate::ui::ui_tree::UITree;
use crate::utils::Auxiliaries;

#[tokio::main]
async fn main() -> io::Result<()> {

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;





    let mut ui = UITree::new();
    let app = TTYama::new(&mut ui, Some("localhost:8080"));

    // Main loop
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                Paragraph::new("")
                    .block(Block::default().borders(Borders::ALL).title("ttyama")),
                area
            );
            ui.render(frame);
        })?;

        // Handle input
        if let Event::Key(KeyEvent{ code, modifiers, kind, state }) = event::read()? {
            if code == KeyCode::Char('q') && modifiers.contains(KeyModifiers::CONTROL) {
                break;
            } else if (code == KeyCode::Up || code == KeyCode::Down || code == KeyCode::Right || code == KeyCode::Left) && modifiers.is_empty() {

                let d = match code {
                    KeyCode::Up => ui::Direction::Up,
                    KeyCode::Down => ui::Direction::Down,
                    KeyCode::Left => ui::Direction::Left,
                    KeyCode::Right => ui::Direction::Right,
                    _ => panic!("ummmmm wtf?")
                };

                if ui.with_relative(d, |o| {  }).is_some() {
                    ui.with_current_mut(|o| {
                        o.set_focused(false);
                    });

                    ui.move_dir(d);
                }
                ui.with_current_mut(|o| {
                    o.set_focused(true);
                });

            } else {
                TTYama::handle_input(app.clone(), &mut ui, KeyEvent{ code, modifiers, kind, state });
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
