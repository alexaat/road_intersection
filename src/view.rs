use super::Model;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::model::Line;
use sdl2::rect::{Point, Rect};
use crate::model::Car;
use crate::model::TrafficLight;



pub struct View {
    canvas: Canvas<Window>,
    bg_color: (u8, u8, u8),
}

impl View {
    pub fn new(canvas: Canvas<Window>, bg_color: (u8, u8, u8)) -> Self {
        Self { canvas, bg_color }
    }

    pub fn draw_model(&mut self, model: &mut Model) {
        let (r, g, b) = self.bg_color;
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.clear();      
        for marking in &model.road_marking {
            self.draw_line(&marking);
        }
        for car in &model.cars {
            car.draw(&mut self.canvas);
        }
        
        for lights in model.traffic_light_switch.traffic_lights.values() {
            lights.draw(&mut self.canvas);
        }
        self.canvas.present();
    }

    fn draw_line(&mut self, line: &Line) {
        let (r, g, b) = line.color;
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        let start = Point::new(line.start.x, line.start.y);
        let end = Point::new(line.end.x, line.end.y);
        self.canvas.draw_line(start, end).unwrap();
    }


}


impl Drawable for Car {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        let (r, g, b) = self.color;
        canvas.set_draw_color(Color::RGB(r, g, b));
        let x = self.position.x;
        let y = self.position.y;
        let width = self.size.width as u32;
        let length = self.size.length as u32;
        let rect = Rect::new(x, y, width, length);
        canvas.fill_rect(rect).unwrap();
    }
}

impl Drawable for TrafficLight {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        let (r, g, b) = match self.status {
            true => (0, 255, 0),
            false => (255, 0, 0),
        };
        canvas.set_draw_color(Color::RGB(r, g, b));
        let width = self.size.width as u32;
        let length = self.size.length as u32;
        let rect = Rect::new(self.position.x, self.position.y, width, length);
        canvas.fill_rect(rect).unwrap();
    }
}

trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
