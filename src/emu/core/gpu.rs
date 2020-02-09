use super::frame_buffer::*;

pub trait GPU<T: Clone + Copy + Eq + PartialEq> {
    fn process(&self, input: &FrameBuffer<T>, output: &mut FrameBuffer<T>);
}
