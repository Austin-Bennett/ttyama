use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::ui::button::Button;
use crate::ui::Direction;
use crate::ui::ui_tree::UITree;
use crate::utils::Auxiliaries;

pub struct TTYama {
    log_server: Option<TcpStream>
}

impl TTYama {
    pub fn new(tree: &mut UITree, log_server: Option<&str>) -> Arc<Mutex<Self>> {
        tree.insert(
            Button::new(Rect::new(2, 2, 10, 3), "New Chat", |context| {
                context.log("New chat pressed!");
            })
        );

        Arc::new(
            Mutex::new(
                Self{
                    log_server: {
                        if let Some(s) = log_server {
                            Some(TcpStream::connect(s).unwrap())
                        } else {
                            None
                        }
                    }
                }
            )
        )
    }


    pub fn log(&mut self, msg: impl AsRef<str>) {
        if let Some(serv) = &mut self.log_server {
            serv.write(msg.as_ref().as_bytes()).ignore();
        }
    }


    pub fn handle_input(this: Arc<Mutex<Self>>, ui: &mut UITree, inp: KeyEvent) {
        {
            let mut lock = this.lock().unwrap();
            ui.with_current_mut(
                |o| {
                    o.handle_input(inp, lock.deref_mut())
                }
            )
        };
    }
}
