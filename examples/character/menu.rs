use avian3d::prelude::PhysicsGizmos;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::CursorGrabMode,
};
use bevy_egui::{egui::Slider, *};
use bevy_sliding_door::{RequestClose, RequestOpen};

use crate::{SlidingDoor};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_plugins((FrameTimeDiagnosticsPlugin::default(), EguiPlugin::default()))
            .add_systems(
                EguiPrimaryContextPass,
                egui_menu.run_if(in_state(MenuState::InMenu)),
            )
            .add_systems(Update, grab_mouse);
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, PartialOrd, Clone)]
pub enum MenuState {
    #[default]
    InMenu,
    Playing,
}

// grab on left click, release on escape
fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut menustate: ResMut<NextState<MenuState>>,
    mut contexts: EguiContexts,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(ctx) = contexts.ctx_mut()
            && ctx.is_pointer_over_area()
        {
            return;
        }

        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        menustate.set(MenuState::Playing);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
        menustate.set(MenuState::InMenu);
    }
}

fn egui_menu(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    window: Single<&mut Window>,
    doors: Query<Entity, With<SlidingDoor>>,
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    mut time: ResMut<Time<Virtual>>,
) -> Result {
    let fps_text = match diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        None => "N/A".into(),
        Some(value) => format!("{value:>4.0}"),
    };

    egui::Window::new("Debug")
        .resizable(true)
        .show(contexts.ctx_mut()?, |ui| {
            ui.label("Left click to grab mouse, ESC to ungrab");
            ui.label(format!("FPS: {fps_text}"));
            ui.label(format!("VSync: {:?}", window.present_mode));
            ui.checkbox(
                &mut config_store.config_mut::<PhysicsGizmos>().0.enabled,
                "Draw physics",
            );
            if ui.button("Open all doors").clicked() {
                for door in doors.iter() {
                    commands.trigger_targets(RequestOpen, door);
                }
            }
            if ui.button("Close all doors").clicked() {
                for door in doors.iter() {
                    commands.trigger_targets(RequestClose, door);
                }
            }
            let mut speed = time.relative_speed();
            ui.add(Slider::new(&mut speed, 0.0..=10.0).text("Time relative speed"));
            time.set_relative_speed(speed);
        });

    Ok(())
}
