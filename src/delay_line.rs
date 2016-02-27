use std::boxed::Box;
use std::vec::Vec;

pub struct DelayLine<T: Copy> {
    line : Box<[T]>,
    position: usize,
    size: usize
}

impl<T: Copy> DelayLine<T> {

    pub fn new(size: usize, default: T) -> DelayLine<T> {
        let mut v_line = Vec::with_capacity(size);
        for _ in 0..size {
            v_line.push(default);
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
        self.line[read_pos]
    }

    ///
    /// resizes the buffer
    /// is very ineffective, especially when automating delay time in daw
    ///
    pub fn resize(&mut self, size: usize, default: T){
        let mut v_line = Vec::with_capacity(size);
        let mut read_pos :usize;

        for i in 0..size {
            if i >= self.size {
                break;
            }
            read_pos = (self.position + 1 + i) % self.size;
            v_line.push(self.line[read_pos]);
        }

        if size > self.size {
            for _ in self.size..size {
                v_line.push(default);
            }
        }

        self.line = v_line.into_boxed_slice();
        self.position = 0;
        self.size = size;
    }
}

impl <T: Copy> Default for DelayLine<T> {
    fn default() -> DelayLine<T> {
        let v_line = Vec::with_capacity(1);

        DelayLine{
            line: v_line.into_boxed_slice(),
            position: 0,
            size: 1
        }
    }
}

#[test]
fn test_length(){
    let mut delay_line = DelayLine::new(1,2);
    assert_eq!(*delay_line.line, [2;1]);
}
#[test]
fn resize() {
    let mut delay_line = DelayLine::new(1,2);
    assert_eq!(*delay_line.line, [2,]);
    delay_line.resize(3,3);
    assert_eq!(*delay_line.line, [2,3,3]);
}