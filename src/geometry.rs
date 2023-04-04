use ggez::glam::Vec2;

/// Returns (x, y, w, h)
pub fn intersection(
    pos_a: Vec2,
    dim_a_w: f32,
    dim_a_h: f32,
    pos_b: Vec2,
    dim_b_w: f32,
    dim_b_h: f32,
) -> Option<(f32, f32, f32, f32)> {
    let left = (pos_a.x - dim_a_w).max(pos_b.x - dim_b_w);
    let top = (pos_a.y - dim_a_h).max(pos_b.y - dim_b_h);

    let right = (pos_a.x + dim_a_w).min(pos_b.x + dim_b_w);
    let bottom = (pos_a.y + dim_a_w).min(pos_b.y + dim_b_h);

    // (x, y, w, h)
    let intersection = (left, top, right - left, bottom - top);

    if right < left || bottom < top {
        return None;
    }

    Some(intersection)
}
