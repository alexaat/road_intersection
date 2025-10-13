use super::Model;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::model::Line;
use sdl2::rect::{Point};


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
        self.canvas.present();
    }

    pub fn draw_line(&mut self, line: &Line) {
        let (r, g, b) = line.color;
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        let start = Point::new(line.start.x, line.start.y);
        let end = Point::new(line.end.x, line.end.y);
        self.canvas.draw_line(start, end).unwrap();
    }


}
