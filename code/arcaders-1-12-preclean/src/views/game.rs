use ::phi::{Phi, View, ViewAction};
use ::phi::data::{MaybeAlive, Rectangle};
use ::phi::gfx::{AnimatedSprite, AnimatedSpriteDescr, CopySprite, Sprite};
use ::sdl2::pixels::Color;
use ::views::shared::BgSet;


/// Pixels traveled by the player's ship every second, when it is moving.
const PLAYER_SPEED: f64 = 180.0;

const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

const BULLET_SPEED: f64 = 240.0;
const BULLET_W: f64 = 8.0;
const BULLET_H: f64 = 4.0;

const ASTEROID_PATH: &'static str = "assets/asteroid.png";
const ASTEROIDS_WIDE: usize = 21;
const ASTEROIDS_HIGH: usize = 7;
const ASTEROIDS_TOTAL: usize = ASTEROIDS_WIDE * ASTEROIDS_HIGH - 4;
const ASTEROID_SIDE: f64 = 96.0;

const EXPLOSION_PATH: &'static str = "assets/explosion.png";
const EXPLOSIONS_WIDE: usize = 5;
const EXPLOSIONS_HIGH: usize = 4;
const EXPLOSIONS_TOTAL: usize = 17;
const EXPLOSION_SIDE: f64 = 96.0;
const EXPLOSION_FPS: f64 = 16.0;
const EXPLOSION_DURATION: f64 = 1.0 / EXPLOSION_FPS * EXPLOSIONS_TOTAL as f64;

const DEBUG: bool = false;


/// The different states our ship might be in. In the image, they're ordered
/// from left to right, then from top to bottom.
#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm   = 0,
    UpFast   = 1,
    UpSlow   = 2,
    MidNorm  = 3,
    MidFast  = 4,
    MidSlow  = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8
}


#[derive(Clone, Copy)]
enum CannonType {
    RectBullet,
    SineBullet { amplitude: f64, angular_vel: f64 },
    DivergentBullet { a: f64, b: f64 },
}


struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
    cannon: CannonType,
}

impl Ship {
    fn spawn_bullets(&self) -> Vec<Box<Bullet>> {
        let cannons_x = self.rect.x + 30.0;
        let cannon1_y = self.rect.y + 6.0;
        let cannon2_y = self.rect.y + SHIP_H - 10.0;

        match self.cannon {
            CannonType::RectBullet =>
                vec![
                    Box::new(RectBullet {
                        rect: Rectangle {
                            x: cannons_x,
                            y: cannon1_y,
                            w: BULLET_W,
                            h: BULLET_H,
                        }
                    }),
                    Box::new(RectBullet {
                        rect: Rectangle {
                            x: cannons_x,
                            y: cannon2_y,
                            w: BULLET_W,
                            h: BULLET_H,
                        }
                    }),
                ],

            CannonType::SineBullet { amplitude, angular_vel } =>
                vec![
                    Box::new(SineBullet {
                        pos_x: cannons_x,
                        origin_y: cannon1_y,
                        amplitude: amplitude,
                        angular_vel: angular_vel,
                        total_time: 0.0,
                    }),
                    Box::new(SineBullet {
                        pos_x: cannons_x,
                        origin_y: cannon2_y,
                        amplitude: amplitude,
                        angular_vel: angular_vel,
                        total_time: 0.0,
                    }),
                ],

            CannonType::DivergentBullet { a, b } =>
                vec![
                    // If a,b > 0, eventually goes upwards
                    Box::new(DivergentBullet {
                        pos_x: cannons_x,
                        origin_y: cannon1_y,
                        a: -a,
                        b: b,
                        total_time: 0.0,
                    }),
                    // If a,b > 0, eventually goes downwards
                    Box::new(DivergentBullet {
                        pos_x: cannons_x,
                        origin_y: cannon2_y,
                        a: a,
                        b: b,
                        total_time: 0.0,
                    }),
                ]
        }
    }
}


trait Bullet {
    /// Update the bullet.
    /// If the bullet should be destroyed, e.g. because it has left the screen,
    /// then return `None`.
    /// Otherwise, return `Some(update_bullet)`.
    fn update(self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>>;

    /// Render the bullet to the screen.
    fn render(&self, phi: &mut Phi);

    /// Return the bullet's bounding box.
    fn rect(&self) -> Rectangle;
}


struct RectBullet {
    rect: Rectangle,
}

impl RectBullet {
    fn new(x: f64, y: f64) -> RectBullet {
        RectBullet {
            rect: Rectangle {
                x: x,
                y: y,
                w: BULLET_W,
                h: BULLET_H,
            }
        }
    }
}

impl Bullet for RectBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        let (w, _) = phi.output_size();
        self.rect.x += BULLET_SPEED * dt;

        // If the bullet has left the screen, then delete it.
        if self.rect.x > w {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        // We will render this kind of bullet in yellow.
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect.to_sdl().unwrap());
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }
}


struct SineBullet {
    pos_x: f64,
    origin_y: f64,
    amplitude: f64,
    angular_vel: f64,
    total_time: f64,
}

impl Bullet for SineBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        self.total_time += dt;
        self.pos_x += BULLET_SPEED * dt;

        // If the bullet has left the screen, then delete it.
        let (w, _) = phi.output_size();

        if self.rect().x > w {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        // We will render this kind of bullet in yellow.
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect().to_sdl().unwrap());
    }

    fn rect(&self) -> Rectangle {
        let dy = self.amplitude * f64::sin(self.angular_vel * self.total_time);
        Rectangle {
            x: self.pos_x,
            y: self.origin_y + dy,
            w: BULLET_W,
            h: BULLET_H,
        }
    }
}


/// Bullet which follows a vertical trajectory given by:
///     a * ((t / b)^3 - (t / b)^2)
struct DivergentBullet {
    pos_x: f64,
    origin_y: f64,
    a: f64, // Influences the bump's height
    b: f64, // Influences the bump's width
    total_time: f64,
}

impl Bullet for DivergentBullet {
    fn update(mut self: Box<Self>, phi: &mut Phi, dt: f64) -> Option<Box<Bullet>> {
        self.total_time += dt;
        self.pos_x += BULLET_SPEED * dt;

        // If the bullet has left the screen, then delete it.
        let (w, h) = phi.output_size();
        let rect = self.rect();

        if rect.x > w || rect.x < 0.0 ||
           rect.y > h || rect.y < 0.0 {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        // We will render this kind of bullet in yellow.
        phi.renderer.set_draw_color(Color::RGB(230, 230, 30));
        phi.renderer.fill_rect(self.rect().to_sdl().unwrap());
    }

    fn rect(&self) -> Rectangle {
        let dy = self.a *
                    ((self.total_time / self.b).powi(3) -
                     (self.total_time / self.b).powi(2));

        Rectangle {
            x: self.pos_x,
            y: self.origin_y + dy,
            w: BULLET_W,
            h: BULLET_H,
        }
    }
}


struct Asteroid {
    sprite: AnimatedSprite,
    rect: Rectangle,
    vel: f64,
}

impl Asteroid {
    fn factory(phi: &mut Phi) -> AsteroidFactory {
        AsteroidFactory {
            sprite: AnimatedSprite::with_fps(
                AnimatedSprite::load_frames(phi, AnimatedSpriteDescr {
                    image_path: ASTEROID_PATH,
                    total_frames: ASTEROIDS_TOTAL,
                    frames_high: ASTEROIDS_HIGH,
                    frames_wide: ASTEROIDS_WIDE,
                    frame_w: ASTEROID_SIDE,
                    frame_h: ASTEROID_SIDE,
                }), 1.0),
        }
    }

    fn update(mut self, phi: &mut Phi, dt: f64) -> Option<Asteroid> {
        self.rect.x -= dt * self.vel;
        self.sprite.add_time(dt);

        if self.rect.x <= -ASTEROID_SIDE {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.rect().to_sdl().unwrap());
        }

        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }
}


struct AsteroidFactory {
    sprite: AnimatedSprite,
}

impl AsteroidFactory {
    fn random(&self, phi: &mut Phi) -> Asteroid {
        let (w, h) = phi.output_size();

        // FPS in [10.0, 30.0)
        let mut sprite = self.sprite.clone();
        sprite.set_fps(::rand::random::<f64>().abs() * 20.0 + 10.0);

        Asteroid {
            sprite: sprite,

            // In the screen vertically, and over the right of the screen
            // horizontally.
            rect: Rectangle {
                w: ASTEROID_SIDE,
                h: ASTEROID_SIDE,
                x: w,
                y: ::rand::random::<f64>().abs() * (h - ASTEROID_SIDE),
            },

            // vel in [50.0, 150.0)
            vel: ::rand::random::<f64>().abs() * 100.0 + 50.0,
        }
    }
}


struct Explosion {
    sprite: AnimatedSprite,
    rect: Rectangle,
    alive_since: f64,
}

impl Explosion {
    fn factory(phi: &mut Phi) -> ExplosionFactory {
        ExplosionFactory {
            sprite: AnimatedSprite::with_fps(
                AnimatedSprite::load_frames(phi, AnimatedSpriteDescr {
                    image_path: EXPLOSION_PATH,
                    total_frames: EXPLOSIONS_TOTAL,
                    frames_high: EXPLOSIONS_HIGH,
                    frames_wide: EXPLOSIONS_WIDE,
                    frame_w: EXPLOSION_SIDE,
                    frame_h: EXPLOSION_SIDE,
                }), EXPLOSION_FPS),
        }
    }

    fn update(mut self, dt: f64) -> Option<Explosion> {
        self.alive_since += dt;
        self.sprite.add_time(dt);

        if self.alive_since >= EXPLOSION_DURATION {
            None
        } else {
            Some(self)
        }
    }

    fn render(&self, phi: &mut Phi) {
        phi.renderer.copy_sprite(&self.sprite, self.rect);
    }
}


struct ExplosionFactory {
    sprite: AnimatedSprite,
}

impl ExplosionFactory {
    fn at_center(&self, center: (f64, f64)) -> Explosion {
        // FPS in [10.0, 30.0)
        let sprite = self.sprite.clone();

        Explosion {
            sprite: sprite,

            // In the screen vertically, and over the right of the screen
            // horizontally.
            rect: Rectangle::with_size(EXPLOSION_SIDE, EXPLOSION_SIDE)
                    .center_at(center),

            alive_since: 0.0,
        }
    }
}



pub struct GameView {
    player: Ship,
    bullets: Vec<Box<Bullet>>,
    asteroids: Vec<Asteroid>,
    asteroid_factory: AsteroidFactory,
    explosions: Vec<Explosion>,
    explosion_factory: ExplosionFactory,
    bg: BgSet,
}

impl GameView {
    /// We temporarily keep this so that we can instanciate `GameView` in
    /// `main` while developing it further.
    #[allow(dead_code)]
    pub fn new(phi: &mut Phi) -> GameView {
        let bg = BgSet::new(&mut phi.renderer);
        GameView::with_backgrounds(phi, bg)
    }

    pub fn with_backgrounds(phi: &mut Phi, bg: BgSet) -> GameView {
        // Get the spaceship's sprites
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    w: SHIP_W,
                    h: SHIP_H,
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                }).unwrap());
            }
        }

        GameView {
            // Entities
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H,
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
                cannon: CannonType::RectBullet,
            },

            bullets: vec![],
            asteroids: vec![],
            asteroid_factory: Asteroid::factory(phi),
            explosions: vec![],
            explosion_factory: Explosion::factory(phi),

            // Scenery
            bg: bg,
        }
    }
}

impl View for GameView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit {
            return ViewAction::Quit;
        }

        if phi.events.now.key_escape == Some(true) {
            return ViewAction::ChangeView(Box::new(
                ::views::main_menu::MainMenuView::with_backgrounds(
                    phi, self.bg.clone())));
        }


        // Change the player's cannons

        if phi.events.now.key_1 == Some(true) {
            self.player.cannon = CannonType::RectBullet;
        }

        if phi.events.now.key_2 == Some(true) {
            self.player.cannon = CannonType::SineBullet {
                amplitude: 10.0,
                angular_vel: 15.0,
            };
        }

        if phi.events.now.key_3 == Some(true) {
            self.player.cannon = CannonType::DivergentBullet {
                a: 100.0,
                b: 1.2,
            };
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

        // The movable region spans the entire height of the window and 70% of its
        // width. This way, the player cannot get to the far right of the screen, where
        // we will spawn the asteroids, and get immediately eliminated.
        //
        // We restrain the width because most screens are wider than they are high.
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 as f64 * 0.70,
            h: phi.output_size().1 as f64,
        };

        // If the player cannot fit in the screen, then there is a problem and
        // the game should be promptly aborted.
        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        // Select the appropriate sprite of the ship to show.
        self.player.current =
            if dx == 0.0 && dy < 0.0       { ShipFrame::UpNorm }
            else if dx > 0.0 && dy < 0.0   { ShipFrame::UpFast }
            else if dx < 0.0 && dy < 0.0   { ShipFrame::UpSlow }
            else if dx == 0.0 && dy == 0.0 { ShipFrame::MidNorm }
            else if dx > 0.0 && dy == 0.0  { ShipFrame::MidFast }
            else if dx < 0.0 && dy == 0.0  { ShipFrame::MidSlow }
            else if dx == 0.0 && dy > 0.0  { ShipFrame::DownNorm }
            else if dx > 0.0 && dy > 0.0   { ShipFrame::DownFast }
            else if dx < 0.0 && dy > 0.0   { ShipFrame::DownSlow }
            else { unreachable!() };


        // Update the bullets
        let old_bullets = ::std::mem::replace(&mut self.bullets, vec![]);

        self.bullets =
            old_bullets.into_iter()
            .filter_map(|bullet| bullet.update(phi, elapsed))
            .collect();

        // Update the asteroids
        self.asteroids =
            ::std::mem::replace(&mut self.asteroids, vec![])
            .into_iter()
            .filter_map(|asteroid| asteroid.update(phi, elapsed))
            .collect();

        // Update the explosions
        self.explosions =
            ::std::mem::replace(&mut self.explosions, vec![])
            .into_iter()
            .filter_map(|explosion| explosion.update(elapsed))
            .collect();


        // Collision detection

        let mut player_alive = true;

        let mut transition_bullets: Vec<_> =
            ::std::mem::replace(&mut self.bullets, vec![])
            .into_iter()
            .map(|bullet| MaybeAlive { alive: true, value: bullet })
            .collect();

        self.asteroids =
            ::std::mem::replace(&mut self.asteroids, vec![])
            .into_iter()
            .filter_map(|asteroid| {
                // By default, the asteroid has not been in a collision.
                let mut asteroid_alive = true;

                for bullet in &mut transition_bullets {
                    if asteroid.rect().overlaps(bullet.value.rect()) {
                        asteroid_alive = false;
                        bullet.alive = false;
                    }
                }

                // The player's ship is destroyed if it is hit by an asteroid.
                // In which case, the asteroid is also destroyed.
                if asteroid.rect().overlaps(self.player.rect) {
                    asteroid_alive = false;
                    player_alive = false;
                }

                if asteroid_alive {
                    Some(asteroid)
                } else {
                    // Spawn an explosive wherever an asteroid was destroyed.
                    self.explosions.push(
                        self.explosion_factory.at_center(
                            asteroid.rect().center()));
                    None
                }
            })
            .collect();

        self.bullets = transition_bullets.into_iter()
            .filter_map(MaybeAlive::as_option)
            .collect();


        // TODO
        // For the moment, we won't do anything about the player dying. This will be
        // the subject of a future episode.
        if !player_alive {
            println!("The player's ship has been destroyed.");
        }


        // Allow the player to shoot after the bullets are updated, so that,
        // when rendered for the first time, they are drawn wherever they
        // spawned.
        if phi.events.now.key_space == Some(true) {
            self.bullets.append(&mut self.player.spawn_bullets());
        }

        // Randomly create an asteroid about once every 100 frames, that is,
        // a bit more often than once every two seconds.
        if ::rand::random::<usize>() % 100 == 0 {
            self.asteroids.push(self.asteroid_factory.random(phi));
        }


        // Clear the scene
        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        // Render the Backgrounds
        self.bg.back.render(&mut phi.renderer, elapsed);
        self.bg.middle.render(&mut phi.renderer, elapsed);

        // Render the bounding box (for debugging purposes)
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(200, 200, 50));
            phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());
        }

        // Render the entities
        phi.renderer.copy_sprite(
            &self.player.sprites[self.player.current as usize],
            self.player.rect);

        for bullet in &self.bullets {
            bullet.render(phi);
        }

        for asteroid in &self.asteroids {
            asteroid.render(phi);
        }

        for explosion in &self.explosions {
            explosion.render(phi);
        }

        // Render the foreground
        self.bg.front.render(&mut phi.renderer, elapsed);


        ViewAction::None
    }
}
