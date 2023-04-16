# Covey Asset Loader

This Bevy plugin tries to tackle the problem of asset dependencies for states. For example, you have
```rs
enum AppState {
    Splash,
    ..
}

struct SplashAssets { .. }
```
and you want to make sure `SplashAssets` is available before `Splash` is entered and also get cleaned up `SplashAssets` when `Splash` is exited.

# Usage
See the [`basic example`](/examples/basic.rs) for a complete usage. But in general, it is like

## Requirements on your `State`
```rs
use covey_asset_loader::prelude::*; 

// implement AssetState
#[derive(AssetState, States, ..)]
enum AppState {     
    #[default]                 
    Boot,
    #[assets(SplashAssets)]
    Splash,
    #[assets(MainMenuAssets)]
    MainMenu,
    // no it doesn't support multiple assets
    // #[assets(InGameAssets1, InGameAssets2)]
    InGame,
}
```

## Requirements on your `Assets`
```rs
// implement AssetCollection , Resource and Reflect
#[derive(AssetCollection, Resource, Reflect, ..)]
struct SplashAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    font1: Handle<Font>,
    #[asset(path = "audio/breakout_collision.ogg")]
    audio1: Handle<AudioSource>,
    #[asset(path = "images/icon.png")]
    image1: Handle<Image>,
}
```

## Available methods
```rs
impl Plugin for SplashPlugin { 
    fn build(&self, app: &mut App) {
        app
            // Required, load SplashAssets for AppState
            .state_asset_loader::<SplashAssets, AppState>()
            // Optional, release SplashAssets when it exits AppState::Splash after `GlobalAssetCleanUpTimer` has finished, default is 5 seconds.
            .cleanup_assets_on_exit::<SplashAssets>(AppState::Splash)
            // Optional, if specified, will override `GlobalAssetCleanUpTimer` for this specific Resrouce, in this case, SplashAssets will be released after 2 seconds.
            .insert_resource(AssetCleanUpTimer::<SplashAssets>(
                Timer::from_seconds(2.0, TimerMode::Once),
                PhantomData,
            ))
    }
}
```

## Changing state

Unfortunately to make it work, you have to use `ScheduleNextState` instead of `NextState` to change state. It is not a feature but a workaround of the limitation of Bevy

```rs
fn system(
    mut schedule_state: ResMut<ScheduleNextState<AppState>>,
) {
    schedule_state.set(AppState::MainMenu);
}
```
