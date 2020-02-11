pub struct FrameBuffer<T> {
    buf: Vec<T>,
    width: u32,
    height: u32,
    completed: bool,
}

impl<T: Copy + Clone + Eq + PartialEq> FrameBuffer<T> {
    pub fn new(width: u32, height: u32, init: T) -> Self {
        Self {
            buf: vec![init; (width * height) as usize],
            width,
            height,
            completed: false,
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
        if self.completed {
            eprintln!("Warning: Writing to framebuffer that hasn't been handled yet");
        }

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.buf[(y * self.width + x) as usize] = val;
        } else {
            println!("Ignoring pixel out of bounds");
        }
    }

    pub fn clear(&mut self, init: T) {
        self.buf = vec![init; (self.width * self.height) as usize];
    }

    pub fn request_draw(&mut self) {
        self.completed = true;
    }

    pub fn handle_draw(&mut self) -> bool {
        if self.completed {
            self.completed = false;
            true
        } else {
            false
        }
    }
}
