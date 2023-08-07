use std::time::Duration;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::keyboard::Keycode;

use crate::{Particle, ParticleCollection, Vector2};

pub struct Renderer<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    framerate: u32,
    background_color: Color,
    particle_collection: ParticleCollection,
}

impl<'a> Renderer<'a> {
    pub fn new(
        title: &'a str,
        width: u32,
        height: u32,
        framerate: u32,
        background_color: Color,
    ) -> Self {
        Renderer {
            title,
            width,
            height,
            framerate,
            background_color,
            particle_collection: ParticleCollection::new(),
        }
    }

    pub fn start(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(self.title, self.width, self.height)
            .position_centered()
            .build()
            .unwrap();
        let window_size = &window.size();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(self.background_color);
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

                    // Handle mouse down
                    Event::MouseButtonDown {
                        timestamp: _,
                        window_id: _,
                        which: _,
                        mouse_btn: _,
                        clicks: _,
                        x,
                        y,
                    } => self.particle_collection.add_particle(Particle {
                        x,
                        y,
                        size: 10,
                        color: Color::WHITE,
                        gradient: Vector2 { x: 0f32, y: 1.0 },
                    }),
                    _ => {}
                }
            }
            // Clear canvas
            canvas.set_draw_color(self.background_color);
            canvas.clear();

            // Drawing logic
            for (_, particle) in &mut self.particle_collection.particles {
                particle.apply_gradient(); // Apply the gradient before drawing
                println!("{:?}", particle.y);
                canvas.set_draw_color(particle.color);
                canvas
                    .fill_rect(Rect::new(
                        particle.x,
                        particle.y,
                        particle.size,
                        particle.size,
                    ))
                    .unwrap();
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.framerate));
        }
    }
}
