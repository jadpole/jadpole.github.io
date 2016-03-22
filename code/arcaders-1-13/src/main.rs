extern crate rand;
extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_mixer;
extern crate sdl2_ttf;

mod phi;
mod views;

fn main() {
    ::phi::spawn("ArcadeRS Shooter", |phi| {
        Box::new(::views::main_menu::MainMenuView::new(phi))
    });
}
