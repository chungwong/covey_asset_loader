use crate::{despawn_screen, AppState};
use bevy::prelude::*;
use covey_asset_loader::prelude::*;

use std::marker::PhantomData;

#[derive(AssetCollection, Debug, Resource, Reflect)]
pub(crate) struct SplashAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    font1: Handle<Font>,
    #[asset(path = "audio/breakout_collision.ogg")]
    audio1: Handle<AudioSource>,
    #[asset(path = "images/icon.png")]
    image1: Handle<Image>,
}

pub(crate) struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)))
            .state_asset_loader::<SplashAssets, AppState>()
            .cleanup_assets_on_exit::<SplashAssets>(AppState::Splash)
            .insert_resource(AssetCleanUpTimer::<SplashAssets>(
                Timer::from_seconds(2.0, TimerMode::Once),
                PhantomData,
            ))
            .add_systems((
                setup.in_schedule(OnEnter(AppState::Splash)),
                splash_countdown.in_set(OnUpdate(AppState::Splash)),
                despawn_screen::<OnSplashScreen>.in_schedule(OnExit(AppState::Splash)),
                read_event,
            ));
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn setup(mut commands: Commands, assets: Res<SplashAssets>, audio: Res<Audio>) {
    let text_style = TextStyle {
        font: assets.font1.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;

    audio.play(assets.audio1.clone());

    commands.spawn((
        Name::new("Logo"),
        OnSplashScreen,
        SpriteBundle {
            texture: assets.image1.clone(),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Splash"),
        OnSplashScreen,
        Text2dBundle {
            text: Text::from_section("Splash", text_style).with_alignment(text_alignment),
            transform: Transform::from_xyz(0.0, 200.0, 0.0),
            ..default()
        },
    ));
}

fn splash_countdown(
    mut schedule_state: ResMut<ScheduleNextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).just_finished() {
        debug!("Splash countdown finished");
        schedule_state.set(AppState::MainMenu);
    }
}

fn read_event(mut asset_events: EventReader<AssetLoaderEvent<SplashAssets>>) {
    for AssetLoaderEvent { .. } in asset_events.iter() {
        debug!("finished loading SplashAssets");
    }
}
