//! Module for framesets, a list of animations in a spritesheet.

use std::ops::{Deref, DerefMut};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_FRAME_SET_ID: AtomicUsize = AtomicUsize::new(0);

pub fn gen_new_frame_set_id() -> usize {
    NEXT_FRAME_SET_ID.fetch_add(1, Ordering::Relaxed)
}

/// A struct containing a list of frames that can be played. This will be a
/// different value depending on how the frames are ordered.
pub enum Frames {
    /// Frames are ordered from left to right like a book, wrapping at the
    /// edges. Contains (start, end) frames, end is inclusive.
    Ordered(usize, usize)
}

impl Frames {
    /// Get the index (read like a book) of the current frame in the whole
    /// spritesheet given a frame number in this animation.
    pub fn get_frame(&self, frame_num: usize) -> usize {
        match self {
            Frames::Ordered(start, end) => {
                debug_assert!(start + frame_num <= *end);
                start + frame_num
            }
        }
    }
}

pub struct FrameSet {
    pub frames: Vec<Frames>,
}

/// A map of framesets
pub struct FrameSetMap {
    map: BTreeMap<usize, FrameSet>,
}

impl FrameSetMap {
    pub fn new() -> FrameSetMap {
        FrameSetMap {
            map: BTreeMap::new(),
        }
    }
}


impl Deref for FrameSetMap {
    type Target = BTreeMap<usize, FrameSet>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for FrameSetMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
