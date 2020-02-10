use super::frame_buffer::*;

use std::sync::{Mutex, Arc};

trait CPU<T> {
    fn load_rom(rom: &[u8]);
    fn reset(&mut self);
    fn execute(&mut self);
}
