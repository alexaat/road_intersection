use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod controller;
mod model;
mod view;
use controller::Controller;
use model::Model;
use view::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("road intersection", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    
    let view = View::new(canvas, (0, 0, 0));
    let model = Model::new();
    let mut controller = Controller::new(model, view);


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {},
            }
        }
        controller.tick();       
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
   
}
