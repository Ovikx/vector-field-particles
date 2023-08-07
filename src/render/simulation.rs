use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::{event::Event, mouse::MouseButton};

use sdl2::keyboard::Keycode;

use crate::particle::particle::VectorField;
use crate::{Particle, ParticleCollection, Vector2};

pub struct Simulation<'a> {
    title: &'a str,
    width: u32,
    height: u32,
    framerate: u32,
    background_color: Color,
    particle_collection: ParticleCollection,
    vector_field: VectorField,
}

impl<'a> Simulation<'a> {
    pub fn new(
        title: &'a str,
        width: u32,
        height: u32,
        framerate: u32,
        background_color: Color,
    ) -> Self {
        Simulation {
            title,
            width,
            height,
            framerate,
            background_color,
            particle_collection: ParticleCollection::new(),
            vector_field: |point: (i32, i32)| -> (f32, f32) { (0.0, 1.0) },
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

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(self.background_color);
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut frame = 0;
        'running: loop {
            frame = (frame + 1) % 255;
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
                        mouse_btn,
                        clicks: _,
                        x,
                        y,
                    } => match mouse_btn {
                        MouseButton::Left => self.particle_collection.add_particle(Particle {
                            mass: 10,
                            x,
                            y,
                            size: 10,
                            color: Color::WHITE,
                            velocity: Vector2 { x: 0f32, y: -10.0 },
                        }),
                        MouseButton::Right => self.particle_collection.add_particle(Particle {
                            mass: 10,
                            x,
                            y,
                            size: 10,
                            color: Color::GREEN,
                            velocity: Vector2 { x: 0f32, y: -1.0 },
                        }),
                        _ => {}
                    },
                    _ => {}
                }
            }
            // Clear canvas
            canvas.set_draw_color(self.background_color);
            canvas.clear();

            // Removal storage
            let mut to_remove: Vec<&u32> = vec![];

            // Drawing logic
            for (key, particle) in self.particle_collection.particles.iter_mut() {
                particle.update_position(self.vector_field); // Apply the gradient before drawing
                canvas.set_draw_color(particle.color);
                if particle.x < (self.width as i32)
                    && particle.x > 0
                    && particle.y < (self.height as i32)
                    && particle.y > 0
                {
                    canvas
                        .fill_rect(Rect::new(
                            particle.x,
                            particle.y,
                            particle.size,
                            particle.size,
                        ))
                        .unwrap();
                } else {
                    to_remove.push(key);
                }
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.framerate));
        }
    }
}
