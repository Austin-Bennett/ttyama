use crate::ui::button::Button;
use crate::ui::ui_tree::UITree;
use crate::utils::Auxiliaries;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::net::TcpStream;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

pub struct TTYama {
    log_server: Option<TcpStream>,
    log_file: BufWriter<File>,
}

impl TTYama {
    pub fn new(tree: &mut UITree, log_server: Option<&str>) -> Arc<Mutex<Self>> {
        tree.insert(
            Button::new(Rect::new(2, 2, 10, 3), "New Chat", |context| {
                context.log("New chat pressed!");
            })
        );

        let mut log_file = BufWriter::new(File::create(format!("/logs/log_{}.txt", chrono::Local::now().format("%m-%d-%y-%H:%M"))).unwrap());
        

        Arc::new(
            Mutex::new(
                Self{
                    log_server: {
                        if let Some(s) = log_server {
                            match TcpStream::connect(s) {
                                Ok(stream) => Some(stream),
                                Err(e) => {
                                    log_file.write(format!("Failed to connect to logging server: {:?}", e).as_bytes()).ignore();
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    },
                    log_file,
                }
            )
        )
    }


    pub fn log(&mut self, msg: impl AsRef<str>) {
        if let Some(serv) = &mut self.log_server {
            serv.write(msg.as_ref().as_bytes()).ignore();
        }
        self.log_file.write(msg.as_ref().as_bytes()).ignore();
        self.log_file.write("\n".as_bytes()).ignore();
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
