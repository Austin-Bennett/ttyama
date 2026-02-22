use std::cmp::PartialEq;
use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::ui::{Direction, UIObject};

pub struct UITreeNode {
    obj: Box<dyn UIObject>,

    up: Option<Arc<RwLock<UITreeNode>>>,
    down: Option<Arc<RwLock<UITreeNode>>>,
    left: Option<Arc<RwLock<UITreeNode>>>,
    right: Option<Arc<RwLock<UITreeNode>>>,
}

impl UITreeNode {
    pub fn new(node: Box<dyn UIObject>) -> Self {
        Self{
            obj: node,
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }
}


//keeps track of UI based on its position, allows you to move up, down, left, right, etc.
//automatically inserts UI based on its position in area
pub struct UITree {
    current: Option<Arc<RwLock<UITreeNode>>>
}

impl UITree {
    pub fn new() -> UITree {
        UITree{
            current: None
        }
    }

    pub fn with_current<F, R>(&self, f: F) -> Option<R>
        where F: FnOnce(&dyn UIObject) -> R {
        if let Some(node) = &self.current {
            Some(f(node.read().unwrap().obj.as_ref()))
        } else {
            None
        }
    }

    pub fn with_current_mut<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce(&mut dyn UIObject) -> R {
        if let Some(node) = &self.current {
            Some(f(node.write().unwrap().obj.as_mut()))
        } else {
            None
        }
    }

    pub fn insert<UI: UIObject + 'static>(&mut self, obj: UI) {


        //the force_move is safe since we immediately write back to the current node
        self.current = Self::_insert(self.current.clone()
        , obj)
    }



    fn _insert<UI: UIObject + 'static>(node: Option<Arc<RwLock<UITreeNode>>>, o: UI) -> Option<Arc<RwLock<UITreeNode>>> {
        if let Some(arc) = &node {
            {
                let mut node = arc.write().unwrap();

                let dir = node.obj.relative_direction(o.get_area());

                match dir {
                    Direction::Up => {
                        node.up = Self::_insert(node.up.clone(), o);
                        //guaranteed to be Some(v) after this
                        let up_opt = node.up.clone().unwrap();
                        let mut up = up_opt.write().unwrap();

                        up.down = Some(arc.clone());
                    }
                    Direction::Down => {
                        node.down = Self::_insert(node.down.clone(), o);

                        //guaranteed to be Some(v) after this
                        let down_opt = node.down.clone().unwrap();
                        let mut down = down_opt.write().unwrap();

                        down.up = Some(arc.clone());
                    }
                    Direction::Left => {
                        node.left = Self::_insert(node.left.clone(), o);

                        //guaranteed to be Some(v) after this
                        let left_opt = node.left.clone().unwrap();
                        let mut left = left_opt.write().unwrap();

                        left.right = Some(arc.clone());
                    }
                    Direction::Right => {
                        node.right = Self::_insert(node.right.clone(), o);

                        //guaranteed to be Some(v) after this
                        let right_opt = node.right.clone().unwrap();
                        let mut right = right_opt.write().unwrap();

                        right.left = Some(arc.clone());
                    }
                }
            }
            node
        } else {
            Some(Arc::new(RwLock::new(UITreeNode::new(
                Box::new(o)
            ))))
        }
    }
    
    pub fn with_relative<F, R>(&self, dir: Direction, f: F) -> Option<R>
    where F: FnOnce(&Arc<RwLock<UITreeNode>>) -> R {
        if let Some(n) = self.current.clone() {
            let readlock = n.read().unwrap();
            match dir {
                Direction::Up => {
                    if let Some(n) = &readlock.up {
                        Some(f(n))
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if let Some(n) = &readlock.down {
                        Some(f(n))
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if let Some(n) = &readlock.left {
                        Some(f(n))
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if let Some(n) = &readlock.right {
                        Some(f(n))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn move_dir(&mut self, dir: Direction) -> bool {
        if let Some(n) = self.current.clone() {
            let readlock = n.read().unwrap();
            match dir {
                Direction::Up => {
                    if readlock.up.is_some() {
                        self.current = readlock.up.clone();
                        true
                    } else {
                        false
                    }
                }
                Direction::Down => {
                    if readlock.down.is_some() {
                        self.current = readlock.down.clone();
                        true
                    } else {
                        false
                    }
                }
                Direction::Left => {
                    if readlock.left.is_some() {
                        self.current = readlock.left.clone();
                        true
                    } else {
                        false
                    }
                }
                Direction::Right => {
                    if readlock.right.is_some() {
                        self.current = readlock.right.clone();
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }

    }

    fn dfs<F>(tree: Option<Arc<RwLock<UITreeNode>>>, func: &F, prev: Option<Direction>)
    where F: Fn(Arc<RwLock<UITreeNode>>) -> ()
    {
        if let Some(n) = tree {
            {
                let r = n.read().unwrap();
                if let Some(d) = prev {

                    if d != Direction::Right {
                        Self::dfs(r.right.clone(), func, Some(Direction::Left));
                    }
                    if d != Direction::Down {
                        Self::dfs(r.down.clone(), func, Some(Direction::Up));
                    }
                    if d != Direction::Up {
                        Self::dfs(r.up.clone(), func, Some(Direction::Down));
                    }
                    if d != Direction::Left {
                        Self::dfs(r.left.clone(), func, Some(Direction::Right));
                    }

                } else {

                    Self::dfs(r.right.clone(), func, Some(Direction::Left));
                    Self::dfs(r.down.clone(), func, Some(Direction::Up));
                    Self::dfs(r.up.clone(), func, Some(Direction::Down));
                    Self::dfs(r.left.clone(), func, Some(Direction::Right));
                }
            }
            func(n);
        }
    }

    fn dfs_mut<F>(tree: Option<Arc<RwLock<UITreeNode>>>, func: &mut F, prev: Option<Direction>)
    where F: FnMut(Arc<RwLock<UITreeNode>>) -> ()
    {
        if let Some(n) = tree {
            {
                let r = n.read().unwrap();
                if let Some(d) = prev {

                    if d != Direction::Right {
                        Self::dfs_mut(r.right.clone(), func, Some(Direction::Left));
                    }
                    if d != Direction::Down {
                        Self::dfs_mut(r.down.clone(), func, Some(Direction::Up));
                    }
                    if d != Direction::Up {
                        Self::dfs_mut(r.up.clone(), func, Some(Direction::Down));
                    }
                    if d != Direction::Left {
                        Self::dfs_mut(r.left.clone(), func, Some(Direction::Right));
                    }

                } else {

                    Self::dfs_mut(r.right.clone(), func, Some(Direction::Left));
                    Self::dfs_mut(r.down.clone(), func, Some(Direction::Up));
                    Self::dfs_mut(r.up.clone(), func, Some(Direction::Down));
                    Self::dfs_mut(r.left.clone(), func, Some(Direction::Right));
                }
            }
            func(n);
        }
    }
    
    pub fn render(&self, frame: &mut Frame) {
        Self::dfs_mut(self.current.clone(), &mut |n| {
            
            let read_lock = n.read().unwrap();
            read_lock.obj.render(frame)
            
        }, None);
    }
}









