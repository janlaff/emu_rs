// Graphics buffer size
pub const GFX_BUF_WIDTH: usize = 64;
pub const GFX_BUF_HEIGHT: usize = 32;
pub const GFX_BUF_PIXELS: usize = GFX_BUF_WIDTH * GFX_BUF_HEIGHT;

pub type GPUref = std::cell::RefCell<std::boxed::Box<dyn GPU>>;

pub trait GPU {
    fn write(&mut self, x: usize, y: usize, pixel: u8) -> bool;
    fn read(&self, x: usize, y: usize) -> u8;
    fn clear_buf(&mut self, pixel: u8);
    fn get_output_buffer(&self) -> &[u8];
    fn buffer_width(&self) -> usize;
    fn buffer_height(&self) -> usize;
    fn process_frame(&mut self);
}

pub struct ArrayGPU {
    pixels: [u8; GFX_BUF_PIXELS],
}

impl ArrayGPU {
    pub fn new() -> Self {
        Self {
            pixels: [0; GFX_BUF_PIXELS],
        }
    }
}

impl GPU for ArrayGPU {
    fn write(&mut self, x: usize, y: usize, pixel: u8) -> bool {
        let idx = y * GFX_BUF_WIDTH + x;

        if pixel == 0xFF {
            if self.pixels[idx] == 0x00 {
                self.pixels[idx] = 0xFF;
                return true;
            } else {
                self.pixels[idx] = 0x00;
                return false;
            }
        }

        false
    }

    fn read(&self, x: usize, y: usize) -> u8 {
        self.pixels[y * GFX_BUF_WIDTH + x]
    }

    fn clear_buf(&mut self, pixel: u8) {
        self.pixels = [pixel; GFX_BUF_PIXELS];
    }

    fn process_frame(&mut self) {}

    fn get_output_buffer(&self) -> &[u8] {
        &self.pixels
    }

    fn buffer_width(&self) -> usize {
        GFX_BUF_WIDTH
    }

    fn buffer_height(&self) -> usize {
        GFX_BUF_HEIGHT
    }
}
