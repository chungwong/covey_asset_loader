use bevy::prelude::*;
use covey_asset_loader::prelude::*;

use crate::{despawn_screen, AppState};

#[derive(AssetCollection, Debug, Resource, Reflect)]
pub(crate) struct MainMenuAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    font1: Handle<Font>,
    #[asset(path = "audio/Windless Slopes.ogg")]
    audio1: Handle<AudioSource>,
    #[asset(path = "images/banner.png")]
    image1: Handle<Image>,
}

#[derive(Component)]
struct OnMainMenuScreen;

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.state_asset_loader::<MainMenuAssets, AppState>()
            .cleanup_assets_on_exit::<MainMenuAssets>(AppState::MainMenu)
            .add_systems((
                playn_button.in_set(OnUpdate(AppState::MainMenu)),
                setup.in_schedule(OnEnter(AppState::MainMenu)),
                despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(AppState::MainMenu)),
            ));
    }
}

#[derive(Component)]
struct PlayButton;

fn setup(mut commands: Commands, assets: Res<MainMenuAssets>, audio: Res<Audio>) {
    let text_style = TextStyle {
        font: assets.font1.clone(),
        font_size: 60.0,
        color: Color::BLUE,
    };

    audio.play(assets.audio1.clone());

    commands.spawn((
        Name::new("Logo"),
        OnMainMenuScreen,
        SpriteBundle {
            texture: assets.image1.clone(),
            ..default()
        },
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Main Menu", text_style.clone()));

            parent
                .spawn((PlayButton, ButtonBundle::default()))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("New Game", text_style));
                });
        });
}

fn playn_button(
    mut schedule_state: ResMut<ScheduleNextState<AppState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for interaction in &mut interaction_query {
        if Interaction::Clicked == *interaction {
            schedule_state.set(AppState::InGame);
        }
    }
}
