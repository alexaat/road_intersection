use crate::model::{Destination, Location};
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
            } => self.model.spawn_car(Location::North, Destination::get_random()),
            Event::KeyDown {
                keycode: Some(Keycode::UP),
                ..
            } =>   self.model.spawn_car(Location::South, Destination::get_random()),
            Event::KeyDown {
                keycode: Some(Keycode::LEFT),
                ..
            } => self.model.spawn_car(Location::East, Destination::get_random()),
            Event::KeyDown {
                keycode: Some(Keycode::RIGHT),
                ..
            } =>  self.model.spawn_car(Location::West, Destination::get_random()),
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } =>  self.model.spawn_car(Location::get_random(), Destination::get_random()),
            _ => {}
        }
    }



}