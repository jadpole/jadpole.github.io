extern crate sdl2;

mod events;

use events::Events;
use sdl2::pixels::Color;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().expect("Could not initialize SDL2");
    let video = sdl_context.video().expect("Could not load the video component");

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().expect("Could not open the main window");

    let mut renderer = window.renderer()
        .accelerated()
        .build().expect("Could not create a renderer for the main window");

    // Prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
