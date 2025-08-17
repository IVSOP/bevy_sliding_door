mod character;

use avian3d::{prelude::*, PhysicsPlugins};
use bevy_gearbox::{EnterState, ExitState, GearboxPlugin};
use character::*;
mod menu;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_atmosphere::prelude::*;
use bevy_sliding_door::*;
use menu::*;

#[derive(Component)]
pub struct SensorDoor(Entity);

#[derive(Component)]
pub struct EventEnteredText;

#[derive(Component)]
pub struct EventExitedText;

#[derive(Component)]
pub struct EventRequestText;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // title: "I am a window!".into(),
                // name: Some("bevy.app".into()),
                // resolution: (500., 300.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells Wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                // prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                // enabled_buttons: bevy::window::EnabledButtons {
                //     maximize: false,
                //     ..Default::default()
                // },
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                // visible: false,
                ..default()
            }),
            ..default()
        }),
        MenuPlugin,
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
        CharacterPlugin,
        AtmospherePlugin,
        GearboxPlugin,
        SlidingDoorPlugin,
    ))
    .insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            ..default()
        },
    )
    .insert_gizmo_config(
        DefaultGizmoConfigGroup::default(),
        GizmoConfig {
            enabled: false,
            ..default()
        },
    )
    .add_observer(change_event_enter_text)
    .add_observer(change_event_exit_text)
    .add_systems(Startup, setup_scene)
    .add_systems(Update, (detect_enter_sensor, detect_exit_sensor));

    app.run();
}

// spawns geometry, door, light, and UI
fn setup_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let ground_material = materials.add(StandardMaterial {
        base_color: Color::LinearRgba(LinearRgba::new(0.0, 2.0, 0.0, 1.0)),
        ..default()
    });

    let door_material = materials.add(StandardMaterial {
        base_color: Color::LinearRgba(LinearRgba::new(2.0, 0.0, 0.0, 1.0)),
        ..default()
    });

    let wall_material = materials.add(StandardMaterial {
        base_color: Color::LinearRgba(LinearRgba::new(0.0, 0.0, 2.0, 1.0)),
        ..default()
    });

    let plane = meshes.add(Plane3d::new(Vec3::NEG_Z, Vec2::splat(0.5)));

    let door = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // floor
    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(90.0_f32.to_radians()),
            scale: Vec3::new(100.0, 100.0, 1.0),
        },
        Mesh3d(plane.clone()),
        MeshMaterial3d(ground_material.clone()),
        RigidBody::Static,
        Collider::cuboid(1.0, 1.0, 0.01),
    ));

    // door
    let door_scale = Vec3::new(4.0, 4.0, 0.1);
    let door_start_x = 0.0;
    let door_end_x = 4.0;
    let door_init_pos = Vec3::new(door_start_x, 2.0, -10.0);
    let duration = 2.0;
    let door_entity = commands
        .spawn((
            Transform {
                translation: door_init_pos,
                scale: door_scale,
                ..default()
            },
            Mesh3d(door.clone()),
            MeshMaterial3d(door_material.clone()),
            SlidingDoor {
                start_x: door_start_x,
                end_x: door_end_x,
                target_duration_secs: duration,
                waiting_secs: 3.0,
                ..default()
            },
            Collider::cuboid(1.0, 1.0, 1.0),
            RigidBody::Kinematic,
        ))
        .id();

    // ceiling
    commands.spawn((
        Transform {
            translation: door_init_pos + Vec3::new(0.0, 2.0, 1.0),
            rotation: Quat::from_rotation_x(90.0_f32.to_radians()),
            scale: Vec3::new(4.0, 15.0, 0.1),
        },
        Mesh3d(door.clone()),
        MeshMaterial3d(wall_material.clone()),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Static,
    ));

    // right wall
    commands.spawn((
        Transform {
            translation: door_init_pos + Vec3::new(2.0, 0.0, 1.0),
            rotation: Quat::from_rotation_x(90.0_f32.to_radians())
                * Quat::from_rotation_y(90.0_f32.to_radians()),
            scale: Vec3::new(4.0, 15.0, 0.1),
        },
        Mesh3d(door.clone()),
        MeshMaterial3d(wall_material.clone()),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Static,
    ));

    // left wall
    commands.spawn((
        Transform {
            translation: door_init_pos + Vec3::new(-2.0, 0.0, 1.0),
            rotation: Quat::from_rotation_x(90.0_f32.to_radians())
                * Quat::from_rotation_y(90.0_f32.to_radians()),
            scale: Vec3::new(4.0, 15.0, 0.1),
        },
        Mesh3d(door.clone()),
        MeshMaterial3d(wall_material.clone()),
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Static,
    ));

    // light
    commands.spawn((
        Transform::default().looking_to(
            Dir3::new_unchecked(Vec3::new(-1.0, -1.0, -1.0).normalize()),
            Dir3::Y,
        ),
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 500.0,
            shadows_enabled: true,
            ..default()
        },
    ));

    // sensor
    commands.spawn((
        Transform {
            translation: door_init_pos,
            scale: Vec3::new(4.0, 4.0, 13.5),
            ..default()
        },
        SensorDoor(door_entity),
        Collider::cuboid(1.0, 1.0, 1.0),
        Sensor,
        CollisionEventsEnabled,
    ));

    // UI
    commands
        .spawn((Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(85.0),
            top: Val::Percent(15.0),
            ..default()
        },))
        .with_children(|node| {
            node.spawn((
                Node {
                    width: Val::Percent(100.0),
                    ..default()
                },
                Text::new("Last event entered: "),
                TextLayout::new_with_justify(JustifyText::Center),
            ))
            .with_child((TextSpan::default(), EventEnteredText));

            node.spawn((
                Node {
                    width: Val::Percent(100.0),
                    ..default()
                },
                Text::new("Last event exited: "),
                TextLayout::new_with_justify(JustifyText::Center),
            ))
            .with_child((TextSpan::default(), EventExitedText));

            node.spawn((
                Node {
                    width: Val::Percent(100.0),
                    ..default()
                },
                Text::new("Last door request: "),
                TextLayout::new_with_justify(JustifyText::Center),
            ))
            .with_child((TextSpan::default(), EventRequestText));
        });
}

fn change_event_enter_text(
    trigger: Trigger<EnterState>,
    text: Single<&mut TextSpan, With<EventEnteredText>>,
    names: Query<&Name>,
) {
    let mut textspan = text.into_inner();

    if let Ok(name) = names.get(trigger.target()) {
        **textspan = name.into();
    }
}

fn change_event_exit_text(
    trigger: Trigger<ExitState>,
    text: Single<&mut TextSpan, With<EventExitedText>>,
    names: Query<&Name>,
) {
    let mut textspan = text.into_inner();

    if let Ok(name) = names.get(trigger.target()) {
        **textspan = name.into();
    }
}

fn detect_enter_sensor(
    mut collision_event_reader: EventReader<CollisionStarted>,
    player: Single<Entity, With<Player>>,
    sensors: Populated<&SensorDoor>,
    mut commands: Commands,
    text: Single<&mut TextSpan, With<EventRequestText>>,
) {
    let mut textspan = text.into_inner();

    let player_entity = player.into_inner();
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let entity1 = *entity1;
        let entity2 = *entity2;

        if entity1 == player_entity {
            if let Ok(door_sensor) = sensors.get(entity2) {
                commands.trigger_targets(RequestOpen, door_sensor.0);
                **textspan = "RequestOpen".into();
            }
        } else if entity2 == player_entity {
            if let Ok(door_sensor) = sensors.get(entity1) {
                commands.trigger_targets(RequestOpen, door_sensor.0);
                **textspan = "RequestOpen".into();
            }
        }
    }
}

fn detect_exit_sensor(
    mut collision_event_reader: EventReader<CollisionEnded>,
    player: Single<Entity, With<Player>>,
    sensors: Populated<&SensorDoor>,
    mut commands: Commands,
    text: Single<&mut TextSpan, With<EventRequestText>>,
) {
    let mut textspan = text.into_inner();

    let player_entity = player.into_inner();
    for CollisionEnded(entity1, entity2) in collision_event_reader.read() {
        let entity1 = *entity1;
        let entity2 = *entity2;

        if entity1 == player_entity {
            if let Ok(door_sensor) = sensors.get(entity2) {
                commands.trigger_targets(RequestClose, door_sensor.0);
                **textspan = "RequestClose".into();
            }
        } else if entity2 == player_entity {
            if let Ok(door_sensor) = sensors.get(entity1) {
                commands.trigger_targets(RequestClose, door_sensor.0);
                **textspan = "RequestClose".into();
            }
        }
    }
}
