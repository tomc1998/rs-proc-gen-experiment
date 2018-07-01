//! This is a module which contains a binary tree implementation for the bin
//! packing algorithm.

#[derive(Clone, Copy, Debug, Fail)]
pub enum PackRectError {
    #[fail(display="Space too small to pack the rect")]
    /// This variant is returned when the space in the node is too small for the
    /// given rect you're attempting to pack into it.
    SpaceTooSmall,
}

/// A binary tree bin packing node
pub struct BinPackNode {
    l_child: Option<Box<BinPackNode>>,
    r_child: Option<Box<BinPackNode>>,

    /// The space contained in this node as a pixel rect - x y w h
    space: [u16; 4],

    pub is_leaf: bool,
}
impl BinPackNode {
    /// Create a new binary tree node with the given pixel rect as space.
    pub fn new(space: [u16; 4]) -> BinPackNode {
        BinPackNode {
            l_child: None, r_child: None,
            space: space,
            is_leaf: true,
        }
    }

    /// Pack a rect into this space. Change this node into a branch, and add both
    /// children. l_child will be the rect below this newly packed rect, and
    /// r_child will be the remaining space to the right on the same row as the
    /// given rect.
    ///
    /// Free space on the right is a rectangle of the same height as the given
    /// rectangle to pack in this function. Space below has the same width,
    /// taking up the rest of the available height. This means the rectangle
    /// below this takes the 'diagonal' rectangle.
    /// # Params
    /// * `w` - The width of the rectangle in pixel coordinates.
    /// * `h` - The height of the rectangle in pixel coordinates.
    /// # Returns
    /// The rect the texture was placed in.
    /// # Errors
    /// Returns an error if the given rect is too small for this space.
    /// # Notes
    /// If this node is not a leaf node, then this function will be recursively
    /// called on the child nodes of this node.
    pub fn pack_rect(&mut self, w: u16, h: u16) -> Result<[u16; 4], PackRectError> {
        if !self.is_leaf {
            // Recurse.
            debug_assert!(self.l_child.is_some() && self.r_child.is_some(), 
                          r#"A node in the binary tree is a leaf, but for some reason
                    either l_child or r_child is not set."#);
            let res = self.r_child.as_mut().unwrap().pack_rect(w, h);
            if res.is_err() {
                match res.err().unwrap() {
                    PackRectError::SpaceTooSmall => return self.l_child.as_mut().unwrap().pack_rect(w, h),
                }
            }
            else { return res; }
        }

        // Check the given w/h is small enough to fit
        if w > self.space[2] || h > self.space[3] {
            return Err(PackRectError::SpaceTooSmall);
        }

        // Calculate the space to the right and below once the rectangle has been
        // packed.
        let mut space_below = [0; 4];
        let mut space_right = [0; 4];
        space_below[0] = self.space[0];
        space_below[1] = self.space[1] + h;
        space_below[2] = self.space[2];
        space_below[3] = self.space[3] - h;
        space_right[0] = self.space[0] + w;
        space_right[1] = self.space[1];
        space_right[2] = self.space[2] - w;
        space_right[3] = h;

        // Create the child nodes
        self.l_child = Some(Box::new(BinPackNode::new(space_below)));
        self.r_child = Some(Box::new(BinPackNode::new(space_right)));

        // Set this node's space to the given rect
        self.space = [self.space[0], self.space[1], w, h];
        self.is_leaf = false;

        return Ok(self.space.clone());
    }
}
