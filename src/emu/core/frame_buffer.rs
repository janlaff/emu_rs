use std::sync::mpsc::Sender;
use std::sync::{Condvar, Mutex, Arc};

pub struct FrameBuffer<T> {
    buf: Vec<T>,
    width: u32,
    height: u32,
    output: Sender<()>,
}

impl<T: Copy + Clone + Eq + PartialEq> FrameBuffer<T> {
    pub fn new(width: u32, height: u32, init: T, output: Sender<()>) -> Self {
        Self {
            buf: vec![init; (width * height) as usize],
            width,
            height,
            output
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn frame(&self) -> &[T] {
        &self.buf[..]
    }

    pub fn read(&self, x: u32, y: u32) -> T {
        self.buf[(y * self.width + x) as usize]
    }

    pub fn write(&mut self, x: u32, y: u32, val: T) {
        self.buf[(y * self.width + x) as usize] = val;
    }

    pub fn clear(&mut self, init: T) {
        self.buf = vec![init; (self.width * self.height) as usize];
    }

    pub fn request_draw(&mut self) {
        self.output.send(()).unwrap();
    }
}