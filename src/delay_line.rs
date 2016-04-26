use std::boxed::Box;
use std::vec::Vec;

pub struct DelayLine<T: Copy + Default> {
    line : Box<[T]>,
    position: usize,
    size: usize,
    sample_delay: usize,
    default: T
}

impl<T: Copy + Default> DelayLine<T> {

    pub fn new(size: usize, sample_delay: usize, default: T) -> DelayLine<T> {
        let mut v_line = Vec::with_capacity(size);
        for _ in 0..size {
            v_line.push(default);
        }
        DelayLine{
            line: v_line.into_boxed_slice(),
            position: 0,
            size: size,
            sample_delay: sample_delay,
            default: default
        }
    }

    pub fn add_and_read(&mut self, value: T) -> T{
        //read and write pos
        let read_pos = self.position;
        let write_pos = (self.position + self.sample_delay) % self.size;

        //read val
        let val = self.line[read_pos];
        self.line[read_pos] = self.default;

        //write val
        self.line[write_pos] = value;
        //save new position
        self.position = (self.position + 1) % self.size;

        val
    }


    pub fn set_sample_delay(&mut self, sample_delay: usize){
        self.sample_delay = sample_delay;
    }
}

impl <T: Copy + Default> Default for DelayLine<T> {
    fn default() -> DelayLine<T> {
        let v_line = Vec::with_capacity(1);

        DelayLine{
            line: v_line.into_boxed_slice(),
            position: 0,
            size: 1,
            sample_delay: 1,
            default: Default::default()
        }
    }
}

#[test]
fn test_default(){
    let mut delay_line : DelayLine<f32> = Default::default();
}

#[test]
fn test_delay(){
    let mut delay_line : DelayLine<f32> = DelayLine::new(12, 12, 0.);
    //println!("{}",delay_line.add_and_read(1.) );
    assert_eq!(delay_line.add_and_read(1.), 0.);
}