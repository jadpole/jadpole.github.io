use sdl2::EventPump;

/// An abstraction over Rust-SDL2's EventPump that also acts as a record of the
/// application's events as-of the last time `Events::pump` was called.
pub struct Events {
    pump: EventPump,

    pub quit: bool,
    pub key_escape: bool,
}

impl Events {
    /// Create a new event record based on some SDL event pump.
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
        }
    }

    /// Pump the events that happened since the last frame and update the events
    /// record accordingly.
    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit { .. } => self.quit = true,

                KeyDown { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = true,
                    _ => {}
                },

                KeyUp { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = false,
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
