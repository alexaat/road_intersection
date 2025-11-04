use crate::constants::*;
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
        let all_cars = self.model.cars.clone();
        for car in &mut self.model.cars {
            car.drive(
                &all_cars,
                &self.model.traffic_light_switch.traffic_lights,
                &self.model.config,
            );
        }
        //remove from list cars that are no longer on the screen
        self.remove_old_cars();
        //change traffic lights
        self.control_traffic();
        //draw model
        self.view.draw_model(&mut self.model);
    }

    fn remove_old_cars(&mut self) {
        let clonned = self.model.cars.clone();
        for (index, car) in clonned.iter().enumerate() {
            if car.direction == Location::East && car.position.x > SCREEN_WIDTH_F64 {
                self.model.cars.remove(index);
                break;
            }
            if car.direction == Location::West && car.position.x < -CAR_SIZE_F64 {
                self.model.cars.remove(index);
                break;
            }
            if car.direction == Location::South && car.position.y > SCREEN_HEIGHT_F64 {
                self.model.cars.remove(index);
                break;
            }
            if car.direction == Location::North && car.position.y < -CAR_SIZE_F64 {
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
            } => self
                .model
                .spawn_car(Location::North, Destination::get_random()),
            Event::KeyDown {
                keycode: Some(Keycode::UP),
                ..
            } => self
                .model
                .spawn_car(Location::South, Destination::get_random()),
            Event::KeyDown {
                keycode: Some(Keycode::LEFT),
                ..
            } => self
                .model
                .spawn_car(Location::East, Destination::get_random()),
            Event::KeyDown {
                keycode: Some(Keycode::RIGHT),
                ..
            } => self
                .model
                .spawn_car(Location::West, Destination::get_random()),
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => self
                .model
                .spawn_car(Location::get_random(), Destination::get_random()),
            _ => {}
        }
    }

    pub fn control_traffic(&mut self) {
        //check for queues
        let mut west = (0, Location::West);
        let mut east = (0, Location::East);
        let mut south = (0, Location::South);
        let mut north = (0, Location::North);
        for car in &self.model.cars {
            if car.direction == Location::East
                && car.position.x + CAR_SIZE_F64 / 2.0 <= BREAK_POINT_WEST
            {
                west.0 += CAR_SPEED;
            }
            if car.direction == Location::West
                && car.position.x >= BREAK_POINT_EAST + CAR_SIZE_F64 / 2.0
            {
                east.0 += CAR_SPEED;
            }
            if car.direction == Location::South
                && car.position.y + CAR_SIZE_F64 / 2.0 <= BREAK_POINT_NORTH
            {
                north.0 += CAR_SPEED;
            }
            if car.direction == Location::North
                && car.position.y >= BREAK_POINT_SOUTH + CAR_SIZE_F64 / 2.0
            {
                south.0 += CAR_SPEED;
            }
        }

        if west.0 >= MAX_CARS_IN_QUEUE {
            self.model.traffic_light_switch.urgent_request(west.1);
            return;
        }
        if east.0 >= MAX_CARS_IN_QUEUE {
            self.model.traffic_light_switch.urgent_request(east.1);
            return;
        }
        if north.0 >= MAX_CARS_IN_QUEUE {
            self.model.traffic_light_switch.urgent_request(north.1);
            return;
        }
        if south.0 >= MAX_CARS_IN_QUEUE {
            self.model.traffic_light_switch.urgent_request(south.1);
            return;
        }
        //Check approach Break Points
        for car in &self.model.cars {
            if car.direction == Location::East
                && (car.position.x == BREAK_POINT_WEST - CAR_SIZE_F64 * 1.5
                    || car.position.x == BREAK_POINT_WEST - CAR_SIZE_F64 / 2.0)
            {
                //Send signal to T/L switch
                self.model.traffic_light_switch.request(Location::West);
                break;
            }
            if car.direction == Location::West
                && (car.position.x == BREAK_POINT_EAST + CAR_SIZE_F64 * 1.5
                    || car.position.x == BREAK_POINT_EAST + CAR_SIZE_F64 / 2.0)
            {
                //Send signal to T/L switch
                self.model.traffic_light_switch.request(Location::East);
                break;
            }
            if car.direction == Location::South
                && (car.position.y == BREAK_POINT_NORTH - CAR_SIZE_F64 * 1.5
                    || car.position.y == BREAK_POINT_NORTH - CAR_SIZE_F64 / 2.0)
            {
                //Send signal to T/L switch
                self.model.traffic_light_switch.request(Location::North);
                break;
            }
            if car.direction == Location::North
                && (car.position.y == BREAK_POINT_SOUTH + CAR_SIZE_F64 * 1.5
                    || car.position.y == BREAK_POINT_SOUTH + CAR_SIZE_F64 / 2.0)
            {
                //Send signal to T/L switch
                self.model.traffic_light_switch.request(Location::South);
                break;
            }
        }
    }
}
