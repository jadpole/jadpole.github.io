extern crate sdl2;

mod phi;
mod views;


fn main() {
    ::phi::spawn("ArcadeRS Shooter", (800, 600), (480, 240), |phi| {
        Box::new(::views::GameView::new(phi))
    });
}
