
/// Rotate a point around a given origin.
#[inline]
pub fn rotate_point(mut p: [f32; 2], o: &[f32; 2], angle: f32) -> [f32; 2] {
    // First, get the point in terms of the origin
    p[0] -= o[0];
    p[1] -= o[1];

    // Now, apply a rotation matrix, and add the origin back
    let cos = angle.cos();
    let sin = angle.sin();
    [p[0] * cos - p[1] * sin + o[0],
     p[0] * sin + p[1] * cos + o[1]]
}
