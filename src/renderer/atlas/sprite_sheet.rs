use super::UvRect;

/// A struct which contains 1 larger UVrect that is split into multiple smaller
/// frames. Used for animations, or tilesets.
pub struct SpriteSheet {
    /// The sprite sheet as a whole UV rect
    whole: UvRect,
    /// Number of columns in the sprite sheet
    columns: usize,
    /// Width of frames in UV units
    frame_w: f32,
    /// Height of frames in UV units
    frame_h: f32,
}

impl SpriteSheet {
    pub fn new(whole: UvRect, columns: usize,
               frame_w: f32, frame_h: f32) -> SpriteSheet {
        SpriteSheet {
            whole, columns, frame_w, frame_h
        }
    }

    /// Given a frame number, retrieve a UV rect for that frame. Panics if the
    /// frame number is out of bound. The sprite sheet is a grid of frames, and
    /// ordered like (english) words - from left to right, then top to bottom.
    pub fn uv_rect(&self, frame_num: usize) -> UvRect {
        let left = self.whole.left  + (frame_num % self.columns) as f32 * self.frame_w;
        let top = self.whole.top + (frame_num / self.columns) as f32 * self.frame_h;
        UvRect {
            left: left,
            top: top,
            right: left + self.frame_w,
            bottom: top + self.frame_h,
        }
    }
}
