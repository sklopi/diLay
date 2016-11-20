#[macro_use] extern crate vst2;
#[macro_use] extern crate conrod;
extern crate libc;
extern crate winapi;
extern crate user32;
extern crate glutin;
extern crate glium;


mod editor;
mod parameter;
mod delay_line;
use delay_line::DelayLine;
use parameter::Parameter;

use vst2::plugin::{Info, Plugin};
use vst2::buffer::{AudioBuffer};
use vst2::editor::Editor;
use editor::DelayEditor;

use std::cell::Cell;


struct DelayPlugin{
    parameters : Vec<Parameter>,
    delay_y : DelayLine<f32>,
    delay_x : DelayLine<f32>,
    sample_delay: usize,
    sample_rate: f32,
    init: bool,
    editor: DelayEditor
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
            automatable: true,
        }];
        self.sample_delay = self.get_samples(self.parameters[0].value.get());
        self.delay_y = DelayLine::new((self.sample_rate * 2.) as usize, self.sample_delay , 0.);
        self.delay_x = DelayLine::new((self.sample_rate * 2.) as usize, self.sample_delay, 0.);
        self.init = true;
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (mut inputs, mut outputs) = buffer.split();
        let (input1,input2) =  inputs.split_at_mut(1);
        let (input1, input2) = (&mut input1[0], &mut input2[0]);

        let (output1,output2) = outputs.split_at_mut(1);
        let (output1,output2) = (&mut output1[0],&mut output2[0]);

        let parameter: &Parameter =   self.parameters.get(0).unwrap();
        let value: f32 = parameter.value.get();
        let samples = self.get_samples(value);
        if self.sample_delay != samples {
            self.delay_x.set_sample_delay(samples);
            self.delay_y.set_sample_delay(samples);
        }
        self.sample_delay = samples;

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

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
        if self.init {
            let sample_delay = self.get_samples(self.parameters[0].value.get());
            self.delay_y = DelayLine::new((self.sample_rate * 2.) as usize, sample_delay, 0.);
            self.delay_x = DelayLine::new((self.sample_rate * 2.) as usize, sample_delay, 0.);
            self.sample_delay = sample_delay;
        }
    }

    fn get_editor(&mut self) -> Option<&mut Editor> {
        Some(&mut self.editor)
    }
}

impl DelayPlugin {
    ///
    /// Generates a number representing the amount of miliseconds the sound should be delayed.
    /// The value is expected to be a value from 0. to 1.
    /// Generates vales between 100. and 600.
    ///
    fn get_ms(value: f32) -> f32 {
        (value * 500. + 100.)
    }

    ///
    /// Generates a number representing the amount of samples the sound should be delayed.
    ///
    fn get_samples(&self, value: f32) -> usize {
        let ms = DelayPlugin::get_ms(value);
        let s = ms / 1000.;

        (self.sample_rate * s) as usize
    }
}

impl Default for DelayPlugin{
    fn default() -> DelayPlugin{
        DelayPlugin{
            parameters : vec![],
            delay_y : DelayLine::new(44100*2, 1, 0.),
            delay_x : DelayLine::new(44100*2, 1, 0.),
            sample_delay: 1,
            sample_rate: 44100.,
            init: false,
            editor: DelayEditor::new()
        }
    }
}

plugin_main!(DelayPlugin); // Important!

#[test]
fn test(){
    let mut plugin : DelayPlugin = Default::default();
    plugin.init();
    plugin.set_sample_rate(44100.);
    let mut i1 = [0. as f32;512];
    let mut i2 = [0. as f32;512];
    let mut o1 = [0. as f32;512];
    let mut o2 = [0. as f32;512];
    let mut buffer: AudioBuffer<f32> = AudioBuffer::new(
        vec![
            &mut i1,
            &mut i2
        ],
        vec![
            &mut o1,
            &mut o2
        ]
    );
    plugin.process(buffer);
}

