use bevy::prelude::*;

mod utils;
use utils::*;

mod plugin;
pub use plugin::*;

#[derive(Component, Default)]
pub struct SlidingDoor {
    pub start_x: f32,
    pub end_x: f32,
    pub idle_secs: f32,
    /// Duration of the animation.
    /// f32 since it never needs to be too accurate, seconds are needed for math, and negative values may appear
    pub target_duration_secs: f32,

    // internal data representing how far along the animation is
    pub current_duration_secs: f32,
}

/// Use this to request the door to open or close
#[derive(Component)]
pub enum SlideActionRequest {
    RequestOpen,
    RequestClose,
}

/// To be used internally. Determines what action the door is performing this frame.
#[derive(Component)]
pub enum SlideAction {
    Open,
    WaitBeforeClose { waited_for_secs: f32 },
    Close,
}

impl SlideAction {
    const WAIT: Self = Self::WaitBeforeClose {
        waited_for_secs: 0.0,
    };
}

/// Handles doors with [`SlideActionRequest`].
/// Requests might be ignored. Behaviour:
/// - Request close
///     - Ignored if opening, waiting or closing
///     - Only taken into consideration if no action is being performed, by going to the waiting state
/// - Request open
///     - Always makes the door go to the open state, no matter what
fn handle_requests(
    // We would have to build this query anyway so no need to make this a hook
    mut doors_with_requests: Populated<(Entity, Option<&mut SlideAction>, &SlideActionRequest)>,
    mut commands: Commands,
) {
    for (entity, action_option, request) in doors_with_requests.iter_mut() {
        if let Some(mut action) = action_option {
            match request {
                SlideActionRequest::RequestOpen => {
                    // requests to open always cause the door to open
                    *action = SlideAction::Open;
                    // remove the request
                    commands.entity(entity).remove::<SlideActionRequest>();
                }
                SlideActionRequest::RequestClose => {
                    match action.as_mut() {
                        SlideAction::Open => {
                            // ignore the close request, but do not delete it
                        }
                        _ => {
                            // the door is either already closing or waiting, so delete the request
                            commands.entity(entity).remove::<SlideActionRequest>();
                        }
                    }
                }
            }
        } else {
            // there is no currently active action
            // open requests make the door open
            // close requests put it in wait
            // request is removed
            let action = match request {
                SlideActionRequest::RequestOpen => SlideAction::Open,
                SlideActionRequest::RequestClose => SlideAction::WAIT,
            };

            commands
                .entity(entity)
                .remove::<SlideActionRequest>()
                .insert(action);
        }
    }
}

/// Performs an action on the door according to the [`SlideAction`].
/// May transition to other actions.
fn handle_door_action(
    mut commands: Commands,
    mut doors: Populated<(Entity, &mut Transform, &mut SlidingDoor, &mut SlideAction)>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();

    for (entity, mut transform, mut door, mut action) in doors.iter_mut() {
        match action.as_mut() {
            SlideAction::Open => {
                door.current_duration_secs += delta_secs;

                if door.current_duration_secs < door.target_duration_secs {
                    // door is opening
                    transform.translation.x = slide_interpolate(&door);
                } else {
                    // finished opening, remove the action component
                    commands.entity(entity).remove::<SlideAction>();
                }
            }
            SlideAction::WaitBeforeClose { waited_for_secs } => {
                *waited_for_secs += delta_secs;
                if *waited_for_secs > door.idle_secs {
                    // idle finished, move to close state
                    *action = SlideAction::Close;
                }
            }
            SlideAction::Close => {
                door.current_duration_secs -= delta_secs;

                if door.current_duration_secs > 0.0 {
                    // door is closing
                    transform.translation.x = slide_interpolate(&door);
                } else {
                    // finished closing, remove the action component
                    commands.entity(entity).remove::<SlideAction>();
                }
            }
        }
    }
}
