use bevy::math::FloatExt;
use interpolation::Ease;

use crate::SlidingDoor;

pub fn slide_interpolate(door: &SlidingDoor) -> f32 {
    let t = (door.current_duration_secs / door.target_duration_secs).clamp(0.0, 1.0);

    let eased_t = Ease::quadratic_in_out(t);

    door.start_x.lerp(door.end_x, eased_t)
}
