mod character;

use avian3d::{prelude::*, PhysicsPlugins};
use character::*;
mod menu;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_atmosphere::prelude::*;
use menu::*;

use bevy_sliding_door::*;

#[derive(Component)]
pub struct SensorDoor(Entity);

#[derive(Component)]
pub struct ActionText;

#[derive(Component)]
pub struct RequestText;

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
    .add_systems(Startup, setup_scene)
    .add_systems(
        Update,
        (detect_enter_sensor, detect_exit_sensor, change_text),
    );

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
                idle_secs: 3.0,
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
                Text::new("Door action: "),
                TextLayout::new_with_justify(JustifyText::Center),
            ))
            .with_child((TextSpan::default(), ActionText));

            node.spawn((
                Node {
                    width: Val::Percent(100.0),
                    ..default()
                },
                Text::new("Door request: "),
                TextLayout::new_with_justify(JustifyText::Center),
            ))
            .with_child((TextSpan::default(), RequestText));
        });
}

fn change_text(
    action_text: Single<&mut TextSpan, (With<ActionText>, Without<RequestText>)>,
    request_text: Single<&mut TextSpan, (With<RequestText>, Without<ActionText>)>,
    door: Single<(Option<&SlideAction>, Option<&SlideActionRequest>), With<SlidingDoor>>,
) {
    let (action_option, request_option) = door.into_inner();
    let mut action_textspan = action_text.into_inner();
    let mut request_textspan = request_text.into_inner();

    let action_text: String = if let Some(action) = action_option {
        match action {
            SlideAction::Open => "opening".into(),
            SlideAction::WaitBeforeClose { waited_for_secs } => {
                format!("waiting ({:.2} secs)", waited_for_secs)
            }
            SlideAction::Close => "closing".into(),
        }
    } else {
        "none".into()
    };

    let request_text: String = if let Some(request) = request_option {
        match request {
            SlideActionRequest::RequestOpen => "open".into(),
            SlideActionRequest::RequestClose => "close".into(),
        }
    } else {
        "none".into()
    };

    **action_textspan = action_text;
    **request_textspan = request_text;
}

fn detect_enter_sensor(
    mut collision_event_reader: EventReader<CollisionStarted>,
    player: Single<Entity, With<Player>>,
    sensors: Populated<&SensorDoor>,
    mut commands: Commands,
) {
    let player_entity = player.into_inner();
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        let entity1 = *entity1;
        let entity2 = *entity2;

        if entity1 == player_entity {
            if let Ok(door_sensor) = sensors.get(entity2) {
                commands
                    .entity(door_sensor.0)
                    .insert(SlideActionRequest::RequestOpen);
            }
        } else if entity2 == player_entity {
            if let Ok(door_sensor) = sensors.get(entity1) {
                commands
                    .entity(door_sensor.0)
                    .insert(SlideActionRequest::RequestOpen);
            }
        }
    }
}

fn detect_exit_sensor(
    mut collision_event_reader: EventReader<CollisionEnded>,
    player: Single<Entity, With<Player>>,
    sensors: Populated<&SensorDoor>,
    mut commands: Commands,
) {
    let player_entity = player.into_inner();
    for CollisionEnded(entity1, entity2) in collision_event_reader.read() {
        let entity1 = *entity1;
        let entity2 = *entity2;

        if entity1 == player_entity {
            if let Ok(door_sensor) = sensors.get(entity2) {
                commands
                    .entity(door_sensor.0)
                    .insert(SlideActionRequest::RequestClose);
            }
        } else if entity2 == player_entity {
            if let Ok(door_sensor) = sensors.get(entity1) {
                commands
                    .entity(door_sensor.0)
                    .insert(SlideActionRequest::RequestClose);
            }
        }
    }
}
