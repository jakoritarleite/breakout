use ggez::glam::vec2;
use ggez::glam::Vec2;

pub fn aabb(
    pos_a: Vec2,
    dim_a_w: f32,
    dim_a_h: f32,
    vel_a: Vec2,
    pos_b: Vec2,
    dim_b_w: f32,
    dim_b_h: f32,
    intersection_w: f32,
    intersection_h: f32,
) -> (Vec2, Vec2) {
    let center_a = vec2(pos_a.x + dim_a_w * 0.5, pos_a.y + dim_a_h * 0.5);
    let center_b = vec2(pos_b.x + dim_b_w * 0.5, pos_b.y + dim_b_h * 0.5);

    let to_signum = (center_b - center_a).signum();

    let mut pos = vec2(pos_a.x, pos_a.y);
    let mut vel = vec2(vel_a.x, vel_a.y);

    if intersection_w > intersection_h {
        pos.y -= to_signum.y * intersection_h;
        vel.y = -to_signum.y * vel.y.abs();
    } else {
        pos.x -= to_signum.x * intersection_w;
        vel.x = -to_signum.x * vel.x.abs();
    }

    (pos, vel)
}
