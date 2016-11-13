use phi::Phi;
use phi::data::Rectangle;
use sdl2::pixels::Color;


/// Pixels traveled horizontally by a bullet every second.
pub const BULLET_SPEED: f64 = 240.0;
pub const BULLET_W: f64 = 8.0;
pub const BULLET_H: f64 = 4.0;


pub trait Bullet {
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


pub struct RectBullet {
    rect: Rectangle,
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
        phi.renderer.fill_rect(self.rect.to_sdl().unwrap()).unwrap();
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }
}


pub struct SineBullet {
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
        phi.renderer.fill_rect(self.rect().to_sdl().unwrap()).unwrap();
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
pub struct DivergentBullet {
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
        phi.renderer.fill_rect(self.rect().to_sdl().unwrap()).unwrap();
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



#[derive(Clone, Copy)]
pub enum CannonType {
    RectBullet,
    SineBullet { amplitude: f64, angular_vel: f64 },
    DivergentBullet { a: f64, b: f64 },
}


pub fn spawn_bullets(cannon: CannonType,
                     cannons_x: f64,
                     cannon1_y: f64,
                     cannon2_y: f64) -> Vec<Box<Bullet>>
{
    match cannon {
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
