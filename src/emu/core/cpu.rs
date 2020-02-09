use super::frame_buffer::*;

trait CPU<T> {
    fn load_rom(rom: &[u8]);
    fn reset(&mut self);
    fn frame_buffer(&self) -> &FrameBuffer<T>;
    fn execute(&mut self);
}
