use bevy_gearbox::prelude::{replay_deferred_event, transition_listener, StateComponentAppExt};

use super::*;

pub struct SlidingDoorPlugin;

impl Plugin for SlidingDoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(create_door_state_machine)
            .add_observer(transition_listener::<RequestOpen>)
            .add_observer(transition_listener::<RequestClose>)
            .add_observer(transition_listener::<FinishedOpening>)
            .add_observer(transition_listener::<FinishedClosing>)
            .add_observer(transition_listener::<FinishedWaiting>)
            .add_observer(replay_deferred_event::<RequestClose>)
            .add_state_component::<DoorClosed>()
            .add_state_component::<DoorOpen>()
            .add_state_component::<DoorClosing>()
            .add_state_component::<DoorOpening>()
            .add_state_component::<DoorWaiting>()
            .add_systems(
                Update,
                (handle_door_open, handle_door_close, handle_door_waiting),
            );
    }
}
