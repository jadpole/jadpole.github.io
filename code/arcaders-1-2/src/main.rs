extern crate sdl2;
mod events;

use ::sdl2::pixels::Color;
use ::events::Events;

fn main() {
    // Initialize SDL2
    let mut sdl_context = sdl2::init().video()
        .build().unwrap();

    // Create the window
    let window = sdl_context.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    let mut events = Events::new(sdl_context.event_pump());

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
