#[macro_use]
extern crate vst2;

mod parameter;
mod delay_line;
use delay_line::DelayLine;
use parameter::Parameter;

use vst2::plugin::{Info, Plugin};
use vst2::buffer::{AudioBuffer};

#[derive(Default)]
struct DelayPlugin{
    parameters : Vec<Parameter>,
    delay_y : DelayLine<f32>,
    delay_x : DelayLine<f32>,
}

impl Plugin for DelayPlugin {

    fn get_info(&self) -> Info {
        Info {
            name: "Delay_Lama".to_string(),
            unique_id: 13367, // Used by hosts to differentiate between plugins.
            inputs:2,
            outputs:2,
            parameters:0,
            ..Default::default()
        }
    }

    fn init(&mut self){
        self.parameters= vec![];
        self.delay_y = DelayLine::new(10240,0.);
        self.delay_x = DelayLine::new(10240,0.);
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (mut inputs, mut outputs) = buffer.split();
        let (input1,input2)=  inputs.split_at_mut(1);
        let (input1, input2) = (&mut input1[0], &mut input2[0]);

        let (output1,output2) = outputs.split_at_mut(1);
        let (output1,output2) = (&mut output1[0],&mut output2[0]);

        for (i,j) in input1.into_iter().enumerate(){
            let mut y = input2[i];
            let mut x = *j;

            let delay_y = self.delay_y.add_and_read(y);
            let delay_x = self.delay_x.add_and_read(x);
            y = y + delay_y;
            x = x + delay_x;

            output1[i] = x;
            output2[i] = y;
        }
    }
}

plugin_main!(DelayPlugin); // Important!


#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
