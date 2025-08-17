use std::time::Duration;

use bevy::prelude::*;
use bevy_gearbox::prelude::*;
use bevy_gearbox::transitions::Source;

use crate::SlidingDoor;

// --- Events ---

// these events are to be used from outside of the plugin

#[derive(Event, Clone)]
pub struct RequestOpen;

#[derive(Event, Clone)]
pub struct RequestClose;

// these events are internal to the plugin, emited when animations/countdowns finish

#[derive(Event, Clone)]
pub struct FinishedOpening;

#[derive(Event, Clone)]
pub struct FinishedClosing;

#[derive(Event, Clone)]
pub struct FinishedWaiting;

// --- State Marker Components ---

/// Marker component for when the door is closed
#[derive(Component, Clone)]
pub struct DoorClosed;

/// Marker component for when the door is opening
#[derive(Component, Clone)]
pub struct DoorOpening;

/// Marker component for when the door is open
#[derive(Component, Clone)]
pub struct DoorOpen;

/// Marker component for when the door is closing
#[derive(Component, Clone)]
pub struct DoorClosing;

/// Marker component for when the door is waiting before starting to close
#[derive(Component, Clone, Default)]
pub struct DoorWaiting {
    pub waiting_for_secs: f32,
}

/// Hook to automatically create the state machine on sliding door entities
pub fn create_door_state_machine(trigger: Trigger<OnAdd, SlidingDoor>, mut commands: Commands) {
    let door_entity = trigger.target();

    commands.queue(move |world: &mut World| {
        // Create state entities - we need intermediate states to defer events
        // let machine_entity = world.spawn(()).id();
        let machine_entity = door_entity; // the entity already exists, no need to make a new one
        let closed = world.spawn(()).id();
        let opening = world.spawn(()).id();
        let open = world.spawn(()).id();
        let closing = world.spawn(()).id();
        let waiting = world.spawn(()).id();

        // Create transition entities
        let closed_to_opening = world.spawn(()).id();
        let opening_to_open = world.spawn(()).id();
        let open_to_waiting = world.spawn(()).id();
        let waiting_to_closing = world.spawn(()).id();
        let closing_to_closed = world.spawn(()).id();
        let closing_to_opening = world.spawn(()).id();
        let waiting_to_open = world.spawn(()).id();

        // Set up the machine root
        world.entity_mut(machine_entity).insert((
            Name::new("DoorStateMachine"),
            StateMachine::new(),
            InitialState(closed),
        ));

        // Set up states with marker components
        world.entity_mut(closed).insert((
            Name::new("Closed"),
            StateChildOf(machine_entity),
            StateComponent(DoorClosed),
        ));

        world.entity_mut(opening).insert((
            Name::new("Opening"),
            StateChildOf(machine_entity),
            StateComponent(DoorOpening), // With<DoorOpening> will tell you doors that are in the DoorOpening state
            DeferEvents::<RequestClose>::new(), // Defer RequestClose while opening. Once the door finishes opening, it will then start to close (or in this case, go to the Waiting state)
        ));

        world.entity_mut(open).insert((
            Name::new("Open"),
            StateChildOf(machine_entity),
            StateComponent(DoorOpen),
        ));

        world.entity_mut(closing).insert((
            Name::new("Closing"),
            StateChildOf(machine_entity),
            StateComponent(DoorClosing),
        ));

        world.entity_mut(waiting).insert((
            Name::new("Waiting"),
            StateChildOf(machine_entity),
            StateComponent(DoorWaiting::default()),
        ));

        // Set up transitions - immediate event-driven transitions, then After delays
        world.entity_mut(closed_to_opening).insert((
            Name::new("Closed -> Opening (RequestOpen)"),
            Target(opening),
            TransitionListener::<RequestOpen>::default(),
            TransitionKind::External,
            Source(closed),
        ));

        world.entity_mut(opening_to_open).insert((
            Name::new("Opening -> Open (FinishedOpening)"),
            Target(open),
            TransitionListener::<FinishedOpening>::default(),
            TransitionKind::External,
            Source(opening),
        ));

        world.entity_mut(open_to_waiting).insert((
            Name::new("Open -> Waiting (RequestClose)"),
            Target(waiting),
            TransitionListener::<RequestClose>::default(),
            TransitionKind::External,
            Source(open),
        ));

        world.entity_mut(closing_to_closed).insert((
            Name::new("Closing -> Closed (FinishedClosing)"),
            Target(closed),
            TransitionListener::<FinishedClosing>::default(),
            TransitionKind::External,
            Source(closing),
        ));

        world.entity_mut(closing_to_opening).insert((
            Name::new("Closing -> Opening (RequestOpen)"),
            Target(opening),
            TransitionListener::<RequestOpen>::default(),
            TransitionKind::External,
            Source(closing),
        ));

        world.entity_mut(waiting_to_closing).insert((
            Name::new("Waiting -> Closing (FinishedWaiting)"),
            Target(closing),
            TransitionListener::<FinishedWaiting>::default(),
            TransitionKind::External,
            Source(waiting),
        ));

        world.entity_mut(waiting_to_open).insert((
            Name::new("Waiting -> Open (RequestOpen)"),
            Target(open),
            TransitionListener::<RequestOpen>::default(),
            TransitionKind::External,
            Source(waiting),
        ));
    });
}
