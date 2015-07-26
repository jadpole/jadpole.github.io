use ::sdl2::event::EventPump;

pub struct Events<'p> {
    pump: EventPump<'p>,

    pub quit: bool,
    pub key_escape: bool,
}

impl<'p> Events<'p> {
    pub fn new(pump: EventPump<'p>) -> Events<'p> {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
        }
    }
    
    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {
            use ::sdl2::event::Event::*;
            use ::sdl2::keyboard::Keycode::*;

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
