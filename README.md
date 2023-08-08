# 2D vector field particle simulator

The idea is pretty simple: the visible screen is a vector field that exerts force on the particles you spawn. The particles have mass, so they don't follow the vector field *exactly*. The vector field you define is used for the space on-screen; if you choose to have "bounds", then everything off-screen has a vector field directed towards the center of the screen.

### Example simulation

Vector field $F(x, y) = \frac{(-x, -y)}{\sqrt{x^2 + y^2}}$ acting on 1500 particles, then 4500 after two more particle spawns  
(Apologies for the low framerate, it came from GIF compression)
![](gifs/sim1.gif)

# Usage
This project is kind of a stepping stone for another project I'm making, so I wouldn't call it user-friendly. If you do want to try it out, clone the repo and follow the [installation instructions for SDL2](https://github.com/Rust-SDL2/rust-sdl2#requirements). 

Once you have SDL2 installed, execute `cargo run` to open the simulation window. Left click to add a single particle, right click to add a couple more particles at once (3000 more, to be exact), `C` to clear the canvas, and `ESC` to close the window.

Main configuration options for the simulator (vector field, bounding, etc.) can be found in `src/main.rs`. `src/render/simulation.rs` contains additional hard-coded options, such as particle mass and color.
