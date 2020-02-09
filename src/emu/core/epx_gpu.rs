use super::frame_buffer::*;
use super::gpu::*;

pub struct EpxGPU {
    pub enabled: bool,
}

impl EpxGPU {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl<T: Clone + Copy + Eq + PartialEq> GPU<T> for EpxGPU {
    fn process(&self, input: &FrameBuffer<T>, output: &mut FrameBuffer<T>) {
        let multiplier = output.width() / input.width();
        assert_eq!(multiplier, 2); // Multiplier must be 2x

        for y in (0..output.height()).step_by(multiplier as usize) {
            for x in (0..output.width()).step_by(multiplier as usize) {
                let (input_x, input_y) = (x / multiplier, y / multiplier);

                let x_min = x == 0;
                let x_max = x + multiplier >= output.width();
                let y_min = y == 0;
                let y_max = y + multiplier >= output.height();

                let origin = input.read(input_x, input_y);

                let top = if y_min {
                    origin.clone()
                } else {
                    input.read(input_x, input_y - 1)
                };

                let right = if x_max {
                    origin.clone()
                } else {
                    input.read(input_x + 1, input_y)
                };

                let left = if x_min {
                    origin.clone()
                } else {
                    input.read(input_x - 1, input_y)
                };

                let bottom = if y_max {
                    origin.clone()
                } else {
                    input.read(input_x, input_y + 1)
                };

                output.write(x, y, origin);
                output.write(x + 1, y, origin);
                output.write(x, y + 1, origin);
                output.write(x + 1, y + 1, origin);

                if self.enabled {
                    if left == top && left != bottom && top != right {
                        output.write(x, y, top);
                    }
                    if top == right && top != left && right != bottom {
                        output.write(x + 1, y, right);
                    }
                    if bottom == left && bottom != right && left != top {
                        output.write(x, y + 1, left);
                    }
                    if right == bottom && right != top && bottom != left {
                        output.write(x + 1, y + 1, bottom);
                    }
                }
            }
        }
    }
}
