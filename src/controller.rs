use crate::model::{Destination, Location};
use crate::Model;
use crate::View;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
const CAR_SIZE: i32 = 24;
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;


pub struct Controller {
    model: Model,
    view: View,
}

impl Controller {
    pub fn new(model: Model, view: View) -> Self {
        Self { model, view }
    }

    pub fn tick(&mut self) {
        let all_cars = self.model.cars.clone();
        for car in &mut self.model.cars {           
            car.drive(&all_cars);
        }
        //remove from list cars that are no longer on the screen
        self.remove_old_cars();

        self.view.draw_model(&mut self.model);
    }

    fn remove_old_cars(&mut self) {
        let clonned = self.model.cars.clone();
        for (index, car) in clonned.iter().enumerate() {
            if car.direction == Location::East && car.position.x > SCREEN_WIDTH {
                self.model.cars.remove(index);
                break;
            }
            if car.direction == Location::West && car.position.x < -CAR_SIZE {
                self.model.cars.remove(index);
                break;
            }
            if car.direction == Location::South && car.position.y > SCREEN_HEIGHT {
                self.model.cars.remove(index);
                break;
            }
            if car.direction == Location::North && car.position.y < -CAR_SIZE {
                self.model.cars.remove(index);
                break;
            }
        }
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