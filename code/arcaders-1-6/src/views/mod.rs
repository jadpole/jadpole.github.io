use phi::{Phi, View, ViewAction};
use phi::data::Rectangle;
use sdl2::pixels::Color;


/// Pixels traveled by the player's ship every second, when it is moving.
const PLAYER_SPEED: f64 = 180.0;


/// The player's ship, currently represented by its bounding-box.
struct Ship {
    rect: Rectangle,
}


/// The main view of our game, in which the player is able to control his ship,
/// which is currently represented by a rectangle.
pub struct GameView {
    player: Ship,
}

impl GameView {
    /// Create a new `GameView` with the ship on the point (64, 64), which is
    /// arbitrary, but does the job for now.
    pub fn new(phi: &mut Phi) -> GameView {
        GameView {
            player: Ship {
                rect: Rectangle { x: 64.0, y: 64.0, w: 32.0, h: 32.0 },
            }
        }
    }
}

impl View for GameView {
    /// Render and move the player's ship according to keyboard events.
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        // Quit the game if asked to
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if phi.events.now.resize {
            println!("{:?}", phi.output_size());
        }

        // Move the player's ship
        let diagonal =
            (phi.events.key_up ^ phi.events.key_down) &&
            (phi.events.key_left ^ phi.events.key_right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };

        // If the player resizes the screen so that the ship can't fit in it
        // anymore, then there is a problem and the game should be aborted.
        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();


        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the scene
        phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
        phi.renderer.fill_rect(self.player.rect.to_sdl()).unwrap();

        ViewAction::None
    }
}
