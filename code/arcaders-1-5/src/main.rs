extern crate sdl2;

mod phi;
mod views;


fn main() {
    ::phi::spawn("ArcadeRS Shooter", |_| {
        Box::new(::views::ViewA)
    });
}
