use super::*;

pub struct SlidingDoorPlugin;

impl Plugin for SlidingDoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_door_action, handle_requests));
    }
}
