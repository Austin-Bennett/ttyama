pub mod ui_tree;
pub mod button;

use std::sync::{Arc, Mutex};
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::ttyama_app::TTYama;
use crate::ui::Direction::{Down, Left, Right, Up};
use crate::ui::ui_tree::UITree;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn direction(a1: Rect, a2: Rect) -> Self {
        let c1 = (a1.x + a1.width / 2, a1.y + a1.height / 2);
        let c2 = (a2.x + a2.width / 2, a2.y + a2.height / 2);
        let dvec = (c2.0 as i32 - c1.0 as i32, c2.1 as i32 - c1.1 as i32);
        //first, we need to determine the larger component
        if dvec.0.abs() > dvec.1.abs() {
            if dvec.0 < 0 {
                Left
            } else {
                Right
            }
        } else {
            if dvec.1 < 0 {
                Up
            } else {
                Down
            }
        }
    }
}

//basically everything in the main loop
pub struct InputContext<'a> {
    pub(crate) ui_tree: &'a mut UITree
}

pub trait UIObject {
    fn get_area(&self) -> Rect;
    fn render(&self, frame: &mut Frame);

    fn set_focused(&mut self, v: bool);
    fn is_focused(&self) -> bool;

    fn relative_direction(&self, area: Rect) -> Direction {
        Direction::direction(self.get_area(), area)
    }

    fn handle_input(&mut self, event: KeyEvent, context: &mut TTYama);
}