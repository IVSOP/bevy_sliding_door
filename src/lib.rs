use bevy::prelude::*;

mod utils;
use bevy_gearbox::active::Active;
use utils::*;

mod plugin;
pub use plugin::*;

mod state_machine;
pub use state_machine::*;

#[derive(Component, Default)]
pub struct SlidingDoor {
    pub start_x: f32,
    pub end_x: f32,
    pub idle_secs: f32,
    /// how long the opening and closing should last
    pub target_duration_secs: f32,

    // internal data representing how far along the animation is
    pub current_duration_secs: f32,
}

pub fn handle_door_open(
    mut commands: Commands,
    mut opening_doors: Populated<(Entity, &mut Transform, &mut SlidingDoor), With<DoorOpening>>,
    time: Res<Time>
) {
    let delta_secs = time.delta_secs();

    for (entity, mut transform, mut door) in opening_doors.iter_mut() {
        door.current_duration_secs += delta_secs;

        // only the X component of the door moves. if the animation has finished, this makes the final position correspond exactly to our goal
        let mut x = door.end_x;

        if door.current_duration_secs >= door.target_duration_secs {
            // animation finished. t is not updated so that the position corresponds exactly to the target
            // use an event to change it to the DoorOpen state
            commands.trigger_targets(FinishedOpening, entity);
        } else {
            // the animation is still going
            // this util function computes the X position for the current animation time
            x = slide_interpolate(door.as_ref());
        }

        transform.translation.x = x;
    }
}

pub fn handle_door_close(
    mut commands: Commands,
    mut opening_doors: Populated<(Entity, &mut Transform, &mut SlidingDoor), With<DoorClosing>>,
    time: Res<Time>
) {
    let delta_secs = time.delta_secs();

    for (entity, mut transform, mut door) in opening_doors.iter_mut() {
        door.current_duration_secs -= delta_secs;

        // only the X component of the door moves. if the animation has finished, this makes the final position correspond exactly to our goal
        let mut x = door.start_x;

        if door.current_duration_secs <= 0.0 {
            // animation finished. t is not updated so that the position corresponds exactly to the target
            // use an event to change it to the DoorOpen state
            commands.trigger_targets(FinishedClosing, entity);
        } else {
            // the animation is still going
            // this util function computes the X position for the current animation time
            x = slide_interpolate(door.as_ref());
        }

        transform.translation.x = x;
    }
}
