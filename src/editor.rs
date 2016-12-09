use std;
use std::path::Path;
use std::collections::vec_deque::{VecDeque, Drain};

use vst2::editor::{Editor, KeyCode, KnobMode};
use libc::c_void;
use libc;

use conrod;
use conrod::{Ui};
use conrod::image::{Map, HashMap};
use conrod::backend::glium::{glium, Renderer};
use glium::{DisplayBuild, Surface, Display};

use glium::texture::Texture2d;
use glium::backend::glutin_backend::GlutinFacade;
use glutin::{WindowID};
use glium::glutin::WindowBuilder;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 640;

pub struct DelayEditor {
    ctx: Option<RenderingContext>,
    current_delay: f32,
    changed: bool,
}

impl DelayEditor {
    pub fn new() -> DelayEditor {
        DelayEditor {
            ctx: None,
            current_delay: 0.,
            changed: false
        }
    }

    pub fn get_current_delay(&mut self) -> Option<f32> {
        if self.changed {
            self.changed = false;
            Some(self.current_delay)
        } else {
            None
        }
    }

    pub fn set_delay(&mut self, value: f32) {
        self.current_delay = value;
    }
}

pub struct RenderingContext {
    display: GlutinFacade,
    renderer: Renderer,
    ui: Ui,
    ids: Ids,
    image_map: Map<Texture2d>
}


impl Editor for DelayEditor {
    fn size(&self) -> (i32, i32) {
        (WIDTH, HEIGHT)
    }
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }
    fn open(&mut self, window: *mut c_void) {
        let display = WindowBuilder::new()
            .with_parent(WindowID::new(window))
            .with_dimensions(WIDTH as u32, HEIGHT as u32)
            .build_glium().unwrap();
        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).theme(conrod::Theme {
            name: "Main Theme".to_string(),
            padding: conrod::Padding::none(),
            x_position: conrod::Position::Align(conrod::Align::Start, None),
            y_position: conrod::Position::Direction(conrod::Direction::Backwards, 20.0, None),
            background_color: conrod::color::DARK_CHARCOAL,
            shape_color: conrod::color::LIGHT_CHARCOAL,
            border_color: conrod::color::BLACK,
            border_width: 0.0,
            label_color: conrod::color::WHITE,
            font_id: None,
            font_size_large: 26,
            font_size_medium: 18,
            font_size_small: 12,
            widget_styling: std::collections::HashMap::new(),
            mouse_drag_threshold: 0.0,
            double_click_threshold: std::time::Duration::from_millis(500),
        }).build();
        let ids = Ids::new(ui.widget_id_generator());
        let path = Path::new(r"C:\Windows\Fonts\arial.ttf");
        ui.fonts.insert_from_file(path).unwrap();
        self.ctx = Some(RenderingContext {
            display: display,
            renderer: renderer,
            ui: ui,
            ids: ids,
            image_map: Map::new()
        });
    }
    fn is_open(&mut self) -> bool {
        self.ctx.is_some()
    }

    fn idle(&mut self) {
        if let Some(ref mut ctx) = self.ctx {
            for event in ctx.display.poll_events() {
                let window = ctx.display.get_window().unwrap();
                if let Some(event) = conrod::backend::glutin::convert(event.clone(), window) {
                    ctx.ui.handle_event(event);
                }
            }

            if let Some(win_rect) = ctx.ui.rect_of(ctx.ui.window) {
                let (win_w, win_h) = (win_rect.w() as u32, win_rect.h() as u32);
                let (w, h) = ctx.display.get_window().unwrap().get_inner_size_points().unwrap();
                if w != win_w || h != win_h {
                    let event = conrod::event::Input::Resize(w, h);
                    ctx.ui.handle_event(event);
                }
            }


            {
                // Instantiate a GUI demonstrating every widget type provided by conrod.
                let mut gui = ctx.ui.set_widgets();
                use conrod::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};

                widget::Canvas::new().pad(30.).set(ctx.ids.canvas, &mut gui);
                widget::Text::new("diLay").font_size(42).mid_top_of(ctx.ids.canvas).set(ctx.ids.text, &mut gui);
                for value in widget::Slider::new(self.current_delay, 0., 1.)
                    .mid_bottom_of(ctx.ids.canvas)
                    .set(ctx.ids.dial, &mut gui) {
                    self.current_delay = value;
                    self.changed = true;
                }
            }


            let primitives = ctx.ui.draw();
            ctx.renderer.fill(&ctx.display, primitives, &ctx.image_map);
            let mut target = ctx.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            ctx.renderer.draw(&ctx.display, &mut target, &ctx.image_map).unwrap();
            target.finish().unwrap();
        }
    }
    fn close(&mut self) {
        self.ctx = None;
    }
    fn set_knob_mode(&mut self, mode: KnobMode) -> bool {
        true
    }
    fn key_up(&mut self, keycode: KeyCode) -> bool {
        true
    }
    fn key_down(&mut self, keycode: KeyCode) -> bool {
        true
    }
}

widget_ids! {
    pub struct Ids {
        canvas,
        text,
        dial
    }
}
