use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;


pub struct ViewA;

impl View for ViewA {
    fn render(&mut self, phi: &mut Phi, _: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if phi.events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewB));
        }

        phi.renderer.set_draw_color(Color::RGB(255, 0, 0));
        phi.renderer.clear();

        ViewAction::None
    }
}


pub struct ViewB;

impl View for ViewB {
    fn render(&mut self, phi: &mut Phi, _: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if phi.events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(ViewA));
        }

        phi.renderer.set_draw_color(Color::RGB(0, 0, 255));
        phi.renderer.clear();

        ViewAction::None
    }
}
