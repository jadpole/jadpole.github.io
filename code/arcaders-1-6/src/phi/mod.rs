use ::sdl2::render::Renderer;
use ::sdl2::timer;

pub mod data;


#[macro_use]
mod events;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    else: {
        quit: Quit { .. }
    }
}


/// Bundles the Phi abstractions in a single structure which
/// can be passed easily between functions.
///
/// Neither of the attributes should outlive the SDL context.
pub struct Phi<'a> {
    pub events: Events<'a>,
    pub renderer: Renderer<'a>,
}

impl<'a> Phi<'a> {
    pub fn output_size(&self) -> (u32, u32) {
        self.renderer.get_output_size().unwrap()
    }
}


/// A `ViewAction` is a way for the currently executed view to
/// communicate with the game loop. It specifies which action
/// should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    /// Called when `self` becomes the main, rendered view.
    fn resume(&mut self, _context: &mut Phi) {
    }

    /// Called when `self` stops being the main, rendered view.
    fn pause(&mut self, _context: &mut Phi) {
    }

    /// Called on every frame to take care of both the logic and
    /// the rendering of the current view.
    ///
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}


/// Create a window with name `title`, initialize the
/// underlying libraries and start the game with the `View`
/// returned by `init()`.
///
/// # Examples
///
/// Here, we simply show a window with color #ffff00 and exit
/// when escape is pressed or when the window is closed.
///
/// ```
/// struct MyView;
///
/// impl View for MyView {
///     fn resume(&mut self, _: &mut Phi) {}
///     fn pause(&mut self, _: &mut Phi) {}
///     fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
///         if context.events.temp.exit {
///             return ViewAction::Exit;
///         }
///
///         context.renderer.set_draw_color(Color::RGB(255, 255, 0));
///         context.renderer.clear();
///         ViewAction::None
///     }
/// }
///
/// spawn("Example", |_| {
///     Box::new(MyView)
/// });
/// ```
pub fn spawn<F>(title: &str, init: F)
where F: Fn(&mut Phi) -> Box<View> {
    // Initialize SDL2

    let mut sdl_context = ::sdl2::init().timer().video()
        .build().unwrap();

    let window = sdl_context.window(title, 800, 600)
        .position_centered().opengl().resizable()
        .build().unwrap();

    let mut context = Phi {
        events: Events::new(sdl_context.event_pump()),
        renderer: window.renderer()
            .accelerated()
            .build().unwrap(),
    };

    // Create and show the default view

    let mut current_view = init(&mut context);
    current_view.resume(&mut context);

    // Frame timing

    let interval = 1_000 / 60;
    let mut before = timer::get_ticks();
    let mut last_second = timer::get_ticks();
    let mut fps = 0u16;

    loop {
        // Frame timing (bis)

        let now = timer::get_ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        if dt < interval {
            timer::delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }


        // Logic & rendering

        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => {
                current_view.pause(&mut context);
                break;
            },
            ViewAction::ChangeView(new_view) => {
                current_view.pause(&mut context);
                current_view = new_view;
                current_view.resume(&mut context);
            }
        }
    }
}
