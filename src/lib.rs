#[macro_use]
extern crate vst2;

mod parameter;
mod delay_line;
use delay_line::DelayLine;
use parameter::Parameter;

use vst2::plugin::{Info, Plugin};
use vst2::buffer::{AudioBuffer};

use std::cell::Cell;

#[derive(Default)]
struct DelayPlugin{
    parameters : Vec<Parameter>,
    delay_y : DelayLine<f32>,
    delay_x : DelayLine<f32>,
    samples: usize
}

impl Plugin for DelayPlugin {

    fn get_info(&self) -> Info {
        Info {
            name: "diLay".to_string(),
            unique_id: 13367, // Used by hosts to differentiate between plugins.
            inputs:2,
            outputs:2,
            parameters:1,
            ..Default::default()
        }
    }

    fn init(&mut self){
        self.parameters= vec![Parameter{
            name: "Delay Time".to_string(),
            label: "ms".to_string(),
            value: Cell::new(0.),
            automatable: true
        }];
        self.samples = DelayPlugin::get_samples(0.);
        self.delay_y = DelayLine::new(44100,0.);
        self.delay_x = DelayLine::new(44100,0.);
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (mut inputs, mut outputs) = buffer.split();
        let (input1,input2) =  inputs.split_at_mut(1);
        let (input1, input2) = (&mut input1[0], &mut input2[0]);

        let (output1,output2) = outputs.split_at_mut(1);
        let (output1,output2) = (&mut output1[0],&mut output2[0]);

        let parameter: &Parameter =   self.parameters.get(0).unwrap();
        let value: f32 = parameter.value.get();
        let samples = DelayPlugin::get_samples(value);
        if self.samples != samples {
            self.delay_x.resize(samples, 0.);
            self.delay_y.resize(samples, 0.);
        }
        self.samples = samples;

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

    fn get_parameter_text(&self, index: i32) -> String {
        let parameter : &Parameter =   self.parameters.get(index as usize).unwrap();
        parameter.value.get().to_string()
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        let parameter :  &Parameter =  self.parameters.get(index as usize).unwrap();
        parameter.value.set(value);
    }
}

impl DelayPlugin {
    fn get_ms(value: f32) -> f32 {
        (value * 500. + 100.)
    }

    fn get_samples(value: f32) -> usize {
        let ms = DelayPlugin::get_ms(value);
        let s = ms / 1000.;
        (44100. * s) as usize
    }
}

plugin_main!(DelayPlugin); // Important!

