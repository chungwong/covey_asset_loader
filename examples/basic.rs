mod main_menu;
mod splash;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use covey_asset_loader::prelude::*;
use std::env;

use crate::{
    main_menu::{MainMenuAssets, MainMenuPlugin},
    splash::{SplashAssets, SplashPlugin},
};

#[derive(AssetState, Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum AppState {
    #[default]
    Boot,
    #[assets(SplashAssets)]
    Splash,
    #[assets(MainMenuAssets)]
    MainMenu,
    InGame,
}

fn jump_to_state() -> ScheduleNextState<AppState> {
    let Ok(state) = env::var("APPSTATE") else {
        return ScheduleNextState(Some(AppState::Splash));
    };

    ScheduleNextState(match state.as_ref() {
        "MainMenu" => Some(AppState::MainMenu),
        "Splash" => Some(AppState::Splash),
        _ => panic!("unrecognised app state {state}"),
    })
}

fn main() {
    let next_state = jump_to_state();

    App::new()
        .insert_resource(next_state)
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(SplashPlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub(crate) fn despawn_screen<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
