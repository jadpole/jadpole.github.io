extern crate sdl2;

use sdl2::pixels::Color;
use std::thread;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().video()
        .build().unwrap();

    // Create the window
    let window = sdl_context.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    thread::sleep_ms(3000);
}
