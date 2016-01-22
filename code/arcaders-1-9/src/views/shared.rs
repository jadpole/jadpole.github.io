use phi::data::Rectangle;
use phi::gfx::{CopySprite, Sprite};
use sdl2::render::Renderer;


#[derive(Clone)]
pub struct Background {
    pub pos: f64,
    // The amount of pixels moved to the left every second
    pub vel: f64,
    pub sprite: Sprite,
}

impl Background {
    pub fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        // We define a logical position as depending solely on the time and the
        // dimensions of the image, not on the screen's size.
        let size = self.sprite.size();
        self.pos += self.vel * elapsed;
        if self.pos > size.0 {
            self.pos -= size.0;
        }

        // We determine the scale ratio of the window to the sprite.
        let (win_w, win_h) = renderer.output_size().unwrap();
        let scale = win_h as f64 / size.1;

        // We render as many copies of the background as necessary to fill
        // the screen.
        let mut physical_left = -self.pos * scale;

        while physical_left < win_w as f64 {
            renderer.copy_sprite(&self.sprite, Rectangle {
                x: physical_left,
                y: 0.0,
                w: size.0 * scale,
                h: win_h as f64,
            });

            physical_left += size.0 * scale;
        }
    }
}


/// Utility structure to reduce the number of arguments certain methods accept,
/// and the amount of `clone` calls required when switching views.
#[derive(Clone)]
pub struct BgSet {
    pub back: Background,
    pub middle: Background,
    pub front: Background,
}

impl BgSet {
    pub fn new(renderer: &mut Renderer) -> BgSet {
        BgSet {
            back: Background {
                pos: 0.0,
                vel: 20.0,
                sprite: Sprite::load(renderer, "assets/starBG.png").unwrap(),
            },
            middle: Background {
                pos: 0.0,
                vel: 40.0,
                sprite: Sprite::load(renderer, "assets/starMG.png").unwrap(),
            },
            front: Background {
                pos: 0.0,
                vel: 80.0,
                sprite: Sprite::load(renderer, "assets/starFG.png").unwrap(),
            },
        }
    }
}
