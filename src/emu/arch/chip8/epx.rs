use super::gpu::*;

pub const EPX_BUF_WIDTH: usize = GFX_BUF_WIDTH * 2;
pub const EPX_BUF_HEIGHT: usize = GFX_BUF_HEIGHT * 2;
pub const EPX_BUF_PIXELS: usize = EPX_BUF_WIDTH * EPX_BUF_HEIGHT;

pub struct EpxGPU {
    input: [u8; GFX_BUF_PIXELS],
    output: [u8; EPX_BUF_PIXELS],
}

impl EpxGPU {
    pub fn new() -> Self {
        Self {
            input: [0; GFX_BUF_PIXELS],
            output: [0; EPX_BUF_PIXELS],
        }
    }
}

impl GPU for EpxGPU {
    fn write(&mut self, x: usize, y: usize, pixel: u8) -> bool {
        let idx = y * GFX_BUF_WIDTH + x;

        if pixel == 0xFF {
            if self.input[idx] == 0x00 {
                self.input[idx] = 0xFF;
                return true;
            } else {
                self.input[idx] = 0x00;
                return false;
            }
        }

        false
    }

    fn read(&self, x: usize, y: usize) -> u8 {
        self.input[y * GFX_BUF_WIDTH + x]
    }

    fn clear_buf(&mut self, pixel: u8) {
        self.input = [pixel; GFX_BUF_PIXELS];
        self.output = [pixel; EPX_BUF_PIXELS];
    }

    fn process_frame(&mut self) {
        for y in (0..EPX_BUF_HEIGHT).step_by(2) {
            for x in (0..EPX_BUF_WIDTH).step_by(2) {
                let (src_x, src_y) = (x / 2, y / 2);

                let top = y == 0;
                let bottom = y + 2 >= EPX_BUF_HEIGHT;
                let left = x == 0;
                let right = x + 2 >= EPX_BUF_WIDTH;

                let p = self.input[src_y * GFX_BUF_WIDTH + src_x];
                let a = if top {
                    p
                } else {
                    self.input[(src_y - 1) * GFX_BUF_WIDTH + src_x]
                };
                let b = if right {
                    p
                } else {
                    self.input[src_y * GFX_BUF_WIDTH + src_x + 1]
                };
                let c = if left {
                    p
                } else {
                    self.input[src_y * GFX_BUF_WIDTH + src_x - 1]
                };
                let d = if bottom {
                    p
                } else {
                    self.input[(src_y + 1) * GFX_BUF_WIDTH + src_x]
                };

                self.output[y * EPX_BUF_WIDTH + x] = p;
                self.output[y * EPX_BUF_WIDTH + x + 1] = p;
                self.output[(y + 1) * EPX_BUF_WIDTH + x] = p;
                self.output[(y + 1) * EPX_BUF_WIDTH + x + 1] = p;

                if c == a && c != d && a != b {
                    self.output[y * EPX_BUF_WIDTH + x] = a;
                }
                if a == b && a != c && b != d {
                    self.output[y * EPX_BUF_WIDTH + x + 1] = b;
                }
                if d == c && d != b && c != a {
                    self.output[(y + 1) * EPX_BUF_WIDTH + x] = c;
                }
                if b == d && b != a && d != c {
                    self.output[(y + 1) * EPX_BUF_WIDTH + x + 1] = d;
                }
            }
        }
    }

    fn get_output_buffer(&self) -> &[u8] {
        &self.output
    }

    fn buffer_width(&self) -> usize {
        EPX_BUF_WIDTH
    }

    fn buffer_height(&self) -> usize {
        EPX_BUF_HEIGHT
    }
}
