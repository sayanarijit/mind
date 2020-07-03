pub mod local;

use crate::Mind;
use std::io;

pub trait Storage {
    fn init() -> io::Result<Self>
    where
        Self: Sized;
    fn load(&self) -> io::Result<Mind>;
    fn save(&self, mind: Mind) -> io::Result<()>;
}
