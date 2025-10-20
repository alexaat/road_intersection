use crate::Model;
use crate::View;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    pub fn new(model: Model, view: View) -> Self {
        Self { model, view }
    }

    pub fn tick(&mut self) {
        self.view.draw_model(&mut self.model);
    }

        pub fn key_down(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::DOWN),
                ..
            } => println!("DOWN"),
            Event::KeyDown {
                keycode: Some(Keycode::UP),
                ..
            } =>  println!("UP"),
            Event::KeyDown {
                keycode: Some(Keycode::LEFT),
                ..
            } => println!("LEFT"),
            Event::KeyDown {
                keycode: Some(Keycode::RIGHT),
                ..
            } => println!("RIGHT"),
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } =>  println!("RANDOM"),
            _ => {}
        }
    }

}