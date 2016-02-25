use std::boxed::Box;
use std::vec::Vec;

pub struct DelayLine<T: Clone> {
    line : Box<[T]>,
    position: usize,
    size: usize
}

impl<T: Clone> DelayLine<T> {
    pub fn new(size: usize, default: T) -> DelayLine<T> {
        let mut v_line = Vec::with_capacity(size);
        for x in 0..size {
            v_line.push(default.clone());
        }
        DelayLine{
            line: v_line.into_boxed_slice(),
            position: 0,
            size: size
        }
    }
    pub fn add_and_read(&mut self, value: T) -> T{
        self.line[self.position] = value;
        let mut read_pos = self.position + 1;
        read_pos = read_pos % self.size;
        self.position = (self.position + 1) % self.size;
        self.line[read_pos].clone()
    }
}

impl <T: Clone> Default for DelayLine<T> {
    fn default() -> DelayLine<T> {
        let mut v_line = Vec::with_capacity(1);

        DelayLine{
            line: v_line.into_boxed_slice(),
            position: 0,
            size: 1
        }
    }
}