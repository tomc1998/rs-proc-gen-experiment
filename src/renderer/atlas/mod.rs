#![allow(dead_code)]

mod bin_packer;
mod charset;

pub use self::charset::*;
pub use self::bin_packer::PackRectError;

use image;
use self::bin_packer::BinPackNode;
use gfx_device_gl::Resources;
use gfx::texture::{Kind, Mipmap, AaMode};
use gfx::format::{R8_G8_B8_A8, Srgb};
use gfx::{handle::ShaderResourceView, Factory};
use std;
use std::collections::BTreeMap;
use rusttype::{Font, PositionedGlyph, self};
use std::path::Path;
use std::fs::File;

/// Spacing in pixels between items in the atlas
const SPACING : u16 = 2;

#[derive(Fail, Debug)]
pub enum AtlasPackErr {
    #[fail(display = "Error loading the font file: {}", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "Error loading the font glyphs: {}", _0)]
    Rusttype(#[cause] rusttype::Error),
    #[fail(display = "Error packing texture into atlas: {}", _0)]
    PackRectErr(#[cause] PackRectError),
    #[fail(display = "Error loading the image file: {}", _0)]
    ImageErr(#[cause] image::ImageError),
}

impl From<std::io::Error> for AtlasPackErr {
    fn from(e: std::io::Error) -> Self { AtlasPackErr::Io(e) }
}

impl From<rusttype::Error> for AtlasPackErr {
    fn from(e: rusttype::Error) -> Self { AtlasPackErr::Rusttype(e) }
}

impl From<bin_packer::PackRectError> for AtlasPackErr {
    fn from(e: bin_packer::PackRectError) -> Self { AtlasPackErr::PackRectErr(e) }
}

impl From<image::ImageError> for AtlasPackErr {
    fn from(e: image::ImageError) -> Self { AtlasPackErr::ImageErr(e) }
}

#[derive(Debug, Clone)]
pub struct UvRect {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl UvRect {
    // X Y W H pixel rect
    pub fn from_pixel_rect(rect: &[u16; 4], w: u16, h: u16) -> UvRect {
        UvRect {
            left: (rect[0] as f32) / w as f32,
            top: (rect[1] as f32) / h as f32,
            right: ((rect[2] + rect[0]) as f32) / w as f32,
            bottom: ((rect[3] + rect[1]) as f32) / h as f32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tileset {
    /// The rect fot the whole tileset
    pub rect: UvRect,
    /// Width in tiles
    pub w: u32,
    /// Height in tiles
    pub h: u32,
}

impl Tileset {
    /// X Y W H pixel rect
    /// # Params
    /// Same as UvRect::from_pixel_rect, but with:
    /// * `tiles_x` - The amount of tiles in this tileset width-wise
    /// * `tiles_y` - The amount of tiles in this tileset width-wise
    pub fn from_pixel_rect(rect: &[u16; 4], w: u16, h: u16, tiles_x: u32, tiles_y: u32) -> Self {
        Tileset {
            rect: UvRect::from_pixel_rect(rect, w, h),
            w: tiles_x,
            h: tiles_y,
        }
    }

    /// Get the UvRect of a given tile in this set
    pub fn tile(&self, x: u32, y: u32) -> UvRect {
        let total_w = self.rect.right - self.rect.left;
        let total_h = self.rect.bottom - self.rect.top;
        UvRect {
            left: self.rect.left + total_w * x as f32 / self.w as f32,
            right: self.rect.left + total_w * (x+1) as f32 / self.w as f32,
            top: self.rect.top + total_h * y as f32 / self.h as f32,
            bottom: self.rect.top + total_h * (y+1) as f32 / self.h as f32,
        }
    }
}

/// An animated sprite.
pub struct AnimSprite {
    /// TODO: Make this a more efficient storage of data, this is pretty bad for
    /// memory with the indirect access
    frames: Vec<UvRect>
}

impl AnimSprite {
    pub fn frame(&self, frame_num: usize) -> &UvRect {
        &self.frames[frame_num]
    }
}

/// Used to build a texture atlas, and a matching texture.
/// K - The type of key used to map texture UVs.
pub struct AtlasBuilder<K : Ord> {
    width: u16,
    height: u16,
    buf: Vec<u8>,
    atlas: TextureAtlas<K>,
    /// Used to pack textures
    bin_pack_tree: BinPackNode,
}

impl<K : Ord> AtlasBuilder<K> {
    pub fn new(w: u16, h: u16) -> AtlasBuilder<K> {
        AtlasBuilder {
            width: w,
            height: h,
            buf: vec![0; w as usize * h as usize * 4],
            atlas: TextureAtlas::new(),
            bin_pack_tree: BinPackNode::new([0, 0, w, h]),
        }
    }

    /// Blit the buf into a given uv rect. Panics if the rect is oob. Rect is X
    /// Y W H - what is returned from pack_rect.
    fn blit(&mut self, buf: &[u8], rect: &[u16; 4]) {
        debug_assert!(rect[0] + rect[2] <= self.width);
        debug_assert!(rect[1] + rect[3] <= self.height);
        for y in rect[1]..(rect[1] + rect[3]) {
            let y = y as usize;
            let range_dst = ((y * self.width as usize + rect[0] as usize) * 4) ..
                ((y * self.width as usize + rect[0] as usize + rect[2] as usize) * 4);
            let range_src = (((y - rect[1] as usize) * rect[2] as usize) * 4) ..
                (((y - rect[1] as usize + 1) * rect[2] as usize) * 4);
            self.buf[range_dst].copy_from_slice(&buf[range_src]);
        }
    }

    /// # Params
    pub fn add_tex<P: AsRef<Path>>(mut self, key: K, img_path: P)
                                   -> Result<Self, AtlasPackErr> {
        // Load the texture
        let img = image::open(img_path)?.to_rgba();
        let (w, h) = img.dimensions();
        let img_buf = img.into_raw();
        let pixel_rect = self.bin_pack_tree.pack_rect(w as u16 + SPACING*2, h as u16 + SPACING*2)?;
        let pixel_rect_unpadded = [
            pixel_rect[0]+SPACING,
            pixel_rect[1]+SPACING,
            pixel_rect[2]-SPACING*2,
            pixel_rect[3]-SPACING*2];
        self.atlas.textures.insert(
            key,
            UvRect::from_pixel_rect(&pixel_rect_unpadded,
                                    self.width, self.height));
        self.blit(&img_buf[..], &pixel_rect_unpadded);
        Ok(self)
    }

    /// # Params
    /// * `tiles_x` - Amount of tiles width-wise
    /// * `tiles_y` - Amount of tiles height-wise
    pub fn add_tileset<P: AsRef<Path>>(mut self, key: K, img_path: P,
                                   tiles_x: u32, tiles_y: u32) -> Result<Self, AtlasPackErr> {
        // Load the texture
        let img = image::open(img_path)?.to_rgba();
        let (w, h) = img.dimensions();
        let img_buf = img.into_raw();
        let pixel_rect = self.bin_pack_tree.pack_rect(w as u16 + SPACING*2, h as u16 + SPACING*2)?;
        let pixel_rect_unpadded = [
            pixel_rect[0]+SPACING,
            pixel_rect[1]+SPACING,
            pixel_rect[2]-SPACING*2,
            pixel_rect[3]-SPACING*2];
        self.atlas.tilesets.insert(
            key,
            Tileset::from_pixel_rect(&pixel_rect_unpadded, self.width,
                                     self.height, tiles_x, tiles_y));
        self.blit(&img_buf[..], &pixel_rect_unpadded);
        Ok(self)
    }

    /// # Params
    /// * `anim_frames` - map of a list of frame numbers, where each sublist
    ///     contains a list of frames that make up an animation, and each key is an
    ///     animation name. Each frame is specified by an x and y pos (not a
    ///     pixel offset, but a frame offset, assuming the input image is
    ///     divided into tiles).
    /// * `frame_w` - The width of a frame
    /// * `frame_h` - The height of a frame
    /// # Example
    /// ```
    /// // Load a 32*32 image which is made up of 4 animations on each row,
    /// // where each frame is 8x8.
    /// let mut frame_map = BTreeMap::new();
    /// frame_map.insert(TextureKey::WalkLeft,  &[(0, 0), (1, 0), (2, 0), (3, 0)][..]);
    /// frame_map.insert(TextureKey::WalkRight, &[(0, 1), (1, 1), (2, 1), (3, 1)][..]);
    /// frame_map.insert(TextureKey::WalkUp,    &[(0, 2), (1, 2), (2, 2), (3, 2)][..]);
    /// frame_map.insert(TextureKey::WalkDown,  &[(0, 3), (1, 3), (2, 3), (3, 3)][..]);
    /// atlas.add_anim_sprite("spritesheet.png", frame_map, 8, 8);
    /// ```
    pub fn add_anim_sprite<P: AsRef<Path>>(mut self, img_path: P,
                                           anim_frames: BTreeMap<K, &[(u16, u16)]>,
                                           frame_w: u16, frame_h: u16) -> Result<Self, AtlasPackErr> {
        // Load the texture
        let img = image::open(img_path)?.to_rgba();
        let (img_w, img_h) = img.dimensions();
        let img_buf = img.into_raw();
        let pixel_rect = self.bin_pack_tree.pack_rect(img_w as u16 + SPACING*2, img_h as u16 + SPACING*2)?;
        let pixel_rect_unpadded = [
            pixel_rect[0]+SPACING,
            pixel_rect[1]+SPACING,
            pixel_rect[2]-SPACING*2,
            pixel_rect[3]-SPACING*2];
        self.blit(&img_buf[..], &pixel_rect_unpadded);
        // Loop over and assign frames
        for (k, v) in anim_frames.into_iter() {
            let anim_uvs = v.iter().map(|(x, y)| {
                UvRect::from_pixel_rect(&[
                    pixel_rect_unpadded[0] + x * frame_w,
                    pixel_rect_unpadded[1] + y * frame_h,
                    frame_w,
                    frame_h], self.width, self.height)
            }).collect();
            self.atlas.anim_sprites.insert(k, AnimSprite {frames : anim_uvs});
        }
        Ok(self)
    }

    /// Set the font to use, with the given charset. Duplicate chars will
    /// probably fuck shit up here.
    ///
    /// # Params
    /// * font_path - The path to the .ttf file
    /// * chars - This is an iterator through the chars to extract from the font
    /// * size - This is the size of the font - for example, 24.0.
    pub fn set_font<P, I>(mut self, font_path: P, chars: I, size: f32) -> Result<Self, AtlasPackErr> where
        P : AsRef<Path>, I : Iterator<Item=char> + Clone
    {
        use rusttype::{Scale, Point};
        use std::io::Read;
        let mut font_data = Vec::new();
        let mut f = File::open(font_path)?;
        f.read_to_end(&mut font_data)?;
        let font = Font::from_bytes(&font_data[..])?;

        // Render glyphs to buffers, then insert into the bin packing tree
        let glyphs : Result<Vec<_>, AtlasPackErr> = chars.clone().zip(
            font.glyphs_for(chars)
                .map(|g| g.scaled(Scale::uniform(size)))
                .map(|g| g.positioned(Point{x: 0.0, y: 0.0}))
                .map(|g| {
                    match g.pixel_bounding_box() {
                        Some(pbb) => {
                            let mut buf = vec![0u8; (pbb.width() * pbb.height() * 4) as usize];
                            g.draw(|x, y, v| {
                                let offset = ((y * pbb.width() as u32 + x) * 4) as usize;
                                let v = (v * 255.0) as u8;
                                buf[offset..offset+4].copy_from_slice(&[v, v, v, v]);
                            });
                            (g, buf, pbb.width() as u16, pbb.height() as u16)
                        }
                        None => {
                            // If we have no pbb, we're probably a whitespace
                            // char, so just allocate a 1x1 black transparent texture
                            (g, vec![0u8; 4], 1, 1)
                        }
                    }
                }))
            .map(|(c, (g, buf, w, h))| {
                let pixel_rect = self.bin_pack_tree.pack_rect(w + SPACING*2,
                                                              h + SPACING*2)?;
                let pixel_rect_unpadded = [
                    pixel_rect[0]+SPACING,
                    pixel_rect[1]+SPACING,
                    pixel_rect[2]-SPACING*2,
                    pixel_rect[3]-SPACING*2];
                self.atlas.glyphs.insert(
                    c, (UvRect::from_pixel_rect(&pixel_rect_unpadded,
                                                self.width, self.height),
                        g.standalone()));
                self.blit(&buf[..], &pixel_rect_unpadded);
                Ok(())
            }).collect();
        glyphs.map(|_| self)
    }

    pub fn build<F>(self, f: &mut F) ->
        (TextureAtlas<K>, ShaderResourceView<Resources, [f32; 4]>)
        where F : Factory<Resources>
    {
        let (_, view) =
            f.create_texture_immutable_u8::<(R8_G8_B8_A8, Srgb)>(
                Kind::D2(self.width, self.height, AaMode::Single),
                Mipmap::Provided,
                &[&self.buf[..]]).unwrap();
        (self.atlas, view)
    }
}

/// An atlas containin both images and font glyphs. Uses a type K to act as a
/// key for textures. Construct with an AtlasBuilder. This struct only actually
/// contains the UV rects - the actual texture data will be returned when using
/// AtlasBuilder.
/// Only 1 distinct font per atlas.
pub struct TextureAtlas<K : Ord> {
    /// Maps tex keys to UV rects
    textures: BTreeMap<K, UvRect>,
    /// Maps chars to UV rects
    glyphs: BTreeMap<char, (UvRect, PositionedGlyph<'static>)>,
    tilesets: BTreeMap<K, Tileset>,
    anim_sprites: BTreeMap<K, AnimSprite>,
}

impl<K : Ord> TextureAtlas<K> {
    fn new() -> TextureAtlas<K> {
        TextureAtlas {
            textures: BTreeMap::new(),
            glyphs: BTreeMap::new(),
            tilesets: BTreeMap::new(),
            anim_sprites: BTreeMap::new(),
        }
    }

    pub fn rect_for_char(&self, c: char) -> Option<&(UvRect, PositionedGlyph<'static>)> {
        self.glyphs.get(&c)
    }

    pub fn rect_for_key(&self, k: K) -> Option<&UvRect> {
        self.textures.get(&k)
    }

    pub fn rect_for_anim_sprite(&self, k: K) -> Option<&AnimSprite> {
        self.anim_sprites.get(&k)
    }

    pub fn rect_for_tileset(&self, k: K) -> Option<&Tileset> {
        self.tilesets.get(&k)
    }
}
