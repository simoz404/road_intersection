extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Point;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    

    
    'running: loop {

        
        canvas.set_draw_color(Color::YELLOW);
        canvas.draw_point(Point::new(25, 325));
        canvas.draw_point(Point::new(425, 575));
        canvas.draw_point(Point::new(375, 25));
        canvas.draw_point(Point::new(775, 275));

        canvas.set_draw_color(Color::WHITE);
        let _ = canvas.draw_line(Point::new(1, 250), Point::new(800, 250));
        let _ = canvas.draw_line(Point::new(1, 350), Point::new(800, 350));
        let _ = canvas.draw_line(Point::new(1, 300), Point::new(800, 300));
        let _ = canvas.draw_line(Point::new(350, 1), Point::new(350, 600));
        let _ = canvas.draw_line(Point::new(450, 1), Point::new(450, 600));
        let _ = canvas.draw_line(Point::new(400, 1), Point::new(400, 600));

        canvas.set_draw_color(Color::YELLOW);
        canvas.draw_point(Point::new(450, 350));
        canvas.draw_point(Point::new(500, 400));

        canvas.draw_point(Point::new(300, 400));
        canvas.draw_point(Point::new(350, 350));




        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {},
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {println!("down"); },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {println!("down"); },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {println!("down"); },

                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

/* fn create_up(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::WHITE);
    let _ = canvas.draw_rect();
} */