macro_rules! struct_events {
    (
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },
        else: { $( $e_alias:ident : $e_sdl:pat ),* }
    )
    => {
        use ::sdl2::event::EventPump;


        pub struct ImmediateEvents {
            resize: Option<(u32, u32)>,
            $( pub $k_alias : Option<bool> , )*
            $( pub $e_alias : bool ),*
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    resize: None,
                    $( $k_alias: None , )*
                    $( $e_alias: false ),*
                }
            }
        }


        pub struct Events<'p> {
            pump: EventPump<'p>,
            pub now: ImmediateEvents,

            $( pub $k_alias: bool ),*
        }

        impl<'p> Events<'p> {
            pub fn new(pump: EventPump<'p>) -> Events<'p> {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    $( $k_alias: false ),*
                }
            }

            pub fn pump(&mut self, renderer: &mut ::sdl2::render::Renderer) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use ::sdl2::event::Event::*;
                    use ::sdl2::event::WindowEventId::Resized;
                    use ::sdl2::keyboard::Keycode::*;

                    match event {
                        Window { win_event_id: Resized, .. } => {
                            self.now.resize = Some(renderer.get_output_size().unwrap());
                        },

                        KeyDown { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // Prevent multiple presses when keeping a key down
                                    // Was previously not pressed?
                                    if !self.$k_alias {
                                        // Key pressed
                                        self.now.$k_alias = Some(true);
                                    }

                                    self.$k_alias = true;
                                }
                            ),*
                            _ => {}
                        },

                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    // Key released
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                            ),*
                            _ => {}
                        },

                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            }
                        )*,

                        _ => {}
                    }
                }
            }
        }
    }
}
