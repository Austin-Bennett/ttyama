use std::mem::ManuallyDrop;
use std::ptr;

pub unsafe fn force_move<T>(data: &T) -> ManuallyDrop<T> {
    unsafe {
        ManuallyDrop::new( ptr::read(data as *const _) )
    }
}

pub trait Auxiliaries: Sized {
    fn ignore(self) {}
}

impl<T> Auxiliaries for T {}