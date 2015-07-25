use ::sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates an SDL-compatible Rect equivalent to `self`.
    /// Panics if it could not be created, for example if a
    /// coordinate of a corner overflows an `i32`.
    pub fn to_sdl(self) -> Option<SdlRect> {
        // SdlRect::new : `(i32, i32, u32, u32) -> Result<Option<SdlRect>>`
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
            .unwrap()
    }

    /// Returns a (perhaps moved) rectangle which is contained by a `parent`
    /// rectangle. If it can indeed be moved to fit, return `Some(result)`;
    /// otherwise, return `None`.
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        // It must be smaller than the parent rectangle to fit in it.
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        Some(Rectangle {
            w: self.w,
            h: self.h,
            x: if self.x < parent.x { parent.x }
               else if self.x + self.w >= parent.x + parent.w { parent.x + parent.w - self.w }
               else { self.x },
            y: if self.y < parent.y { parent.y }
               else if self.y + self.h >= parent.y + parent.h { parent.y + parent.h - self.h }
               else { self.y },
        })
    }
}
