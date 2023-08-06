extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("audioswirl", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let window_size = &window.size();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // Clear canvas
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Drawing logic
        canvas.set_draw_color(Color::WHITE);
        canvas
            .fill_rect(rect::Rect::new(
                (**&window_size).0 as i32 / 2 + i % 100,
                (**&window_size).1 as i32 / 2,
                10,
                10,
            ))
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
