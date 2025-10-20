use super::Model;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::model::Location;
use crate::model::Line;
use sdl2::rect::{Point, Rect};
use crate::model::Car;
use crate::model::TrafficLight;
use sdl2::image::LoadTexture;
use crate::constants::*;

pub struct View {
    canvas: Canvas<Window>,
    bg_color: (u8, u8, u8)
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

        //init textures  
        let field_width = (SCREEN_WIDTH/2 - CAR_SIZE - MARGIN) as u32;
        let field_heigth = (SCREEN_HEIGHT/2 - CAR_SIZE - MARGIN) as u32;
        let url = "assets/images/corner.png";
        let texture_creator = self.canvas.texture_creator();        
        match texture_creator.load_texture(url) {
            Ok(texture) => {
                let query = texture.query();
                let src = Rect::new(0, 0, query.width, query.height);
        
                //draw background top-left
                let dst = Rect::new(0, 0, field_width, field_heigth);
                let center = Point::new((field_width / 2) as i32, (field_heigth / 2) as i32);
                if let Err(e) = self.canvas
                    .copy_ex(&texture, src, dst, 0.0, center, false, false) {
                        println!("Cannot copy texture: {:?}", e)
                    }

                //draw background top-right
                let x = field_width as i32 + CAR_SIZE * 2 + MARGIN * 2;
                let dst = Rect::new(x, 0, field_width, field_heigth);
                let center = Point::new(x + (field_width / 2) as i32, (field_heigth / 2) as i32);
                if let Err(e) = self.canvas
                    .copy_ex(&texture, src, dst, 0.0, center, true, false){
                        println!("Cannot copy texture: {:?}", e)
                    }

                //draw background bottom-left
                let y = field_heigth as i32 + CAR_SIZE * 2 + MARGIN * 2;
                let dst = Rect::new(0, y, field_width, field_heigth);
                let center = Point::new((field_width / 2) as i32,  y + (field_heigth / 2) as i32);
                if let Err(e) = self.canvas
                    .copy_ex(&texture, src, dst, 0.0, center, false, true){
                        println!("Cannot copy texture: {:?}", e)
                    }

                //draw background bottom-right
                let dst = Rect::new(x, y, field_width, field_heigth);
                let center = Point::new(x + (field_width / 2) as i32,  y + (field_heigth / 2) as i32);
                if let Err(e) = self.canvas
                    .copy_ex(&texture, src, dst, 0.0, center, true, true){
                        println!("Cannot copy texture: {:?}", e)
                    }

            },
            Err(e) => println!("Cannot load texture: {:?}", e)
        }      
  

        //draw traffic lights
        model.traffic_light_switch.update(model.cars.clone());
        for lights in model.traffic_light_switch.traffic_lights.values() {
            lights.draw(&mut self.canvas);
        }

        for car in &model.cars {
            car.draw(&mut self.canvas);
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


        let x = self.position.x;
        let y = self.position.y;

        //draw image
        // let url = match  self.destination{
        //    Destination::Left => "assets/images/blue.png",
        //    Destination::Right => "assets/images/orange.png",
        //    _ => "assets/images/white.png"
        // };
        //
        let texture_creator = canvas.texture_creator(); 
       
       //calculate angle
        let angle = match self.direction {
            Location::South => 180.0,
            Location::East => 90.0,
            Location::North => 0.0,
            Location::West => 270.0,            
        };
          
        match texture_creator.load_texture(self.color.url.clone()) {
            Ok(texture) => {
                let query = texture.query();
                let src = Rect::new(0, 0, query.width, query.height);
                let dst = Rect::new(x, y , query.width, query.height);
                let center = Point::new(CAR_SIZE/2, CAR_SIZE/2);
                if let Err(e) = canvas
                    .copy_ex(&texture, src, dst, angle, center, false, false) {
                        println!("Cannot copy texture: {:?}", e);
                        let (r, g, b) = self.color.color;
                        canvas.set_draw_color(Color::RGB(r, g, b));
                        let width = self.size.width as u32;
                        let length = self.size.length as u32;
                        let rect = Rect::new(x, y, width, length);
                        if let Err(e) = canvas.fill_rect(rect){
                            println!("Could not draw on canvas: {:?}", e);
                        }
                    }
            }, 
            Err(e) => {
                println!("Could not loat texture: {:?}", e);
                let (r, g, b) = self.color.color;
                canvas.set_draw_color(Color::RGB(r, g, b));
                let width = self.size.width as u32;
                let length = self.size.length as u32;
                let rect = Rect::new(x, y, width, length);
                if let Err(e) = canvas.fill_rect(rect){
                    println!("Could not draw on canvas: {:?}", e);
                }
            }
        };


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
