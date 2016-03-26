extern crate sdl2;

#[macro_use]
mod events;

use sdl2::pixels::Color;


// Generate an `Events` structure to record SDL events.
struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    },
    else: {
        quit: Quit { .. }
    }
}


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

        if events.now.quit || events.now.key_escape == Some(true) {
            break;
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
