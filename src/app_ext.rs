use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
    reflect::GetPath,
};
use std::marker::{Copy, PhantomData};

use crate::{AssetState, LoadingAssets, ScheduleNextState};

pub struct AssetLoaderEvent<R: Resource> {
    pub handles: Vec<HandleId>,
    _marker: PhantomData<R>,
}

pub trait AssetLoaderAppExt {
    fn state_asset_loader<R, S>(&mut self) -> &mut Self
    where
        R: Resource + FromWorld + Struct,
        S: Copy + AssetState;

    fn track_loading_progress<R, S>(&mut self) -> &mut Self
    where
        R: Resource + FromWorld + Struct,
        S: Copy + AssetState;

    fn cleanup_assets_on_exit<R>(&mut self, state: impl AssetState) -> &mut Self
    where
        R: Resource;
}

/// A marker to determine if [`state_asset_loader`] has been called.
/// Mainly to prevent adding [`load_assets`] more than once.
#[derive(Default, Resource)]
struct AssetLoaderInitMarker;

impl AssetLoaderAppExt for App {
    fn state_asset_loader<R, S>(&mut self) -> &mut Self
    where
        R: Resource + FromWorld + Struct,
        S: Copy + AssetState,
    {
        // Only add [`load_assets`] once by detecting the existence of `AssetLoaderInitMarker`.
        // If it exists, it menas the `state_asset_loader` has been called before and
        // `load_assets` has been added too.
        if self.world.get_resource::<AssetLoaderInitMarker>().is_none() {
            self.add_system(load_assets::<S>.run_if(resource_changed::<ScheduleNextState<S>>()));
        }

        self.init_resource::<AssetLoaderInitMarker>()
            .init_resource::<ScheduleNextState<S>>()
            .track_loading_progress::<R, S>()
    }

    fn track_loading_progress<R, S>(&mut self) -> &mut Self
    where
        R: Resource + FromWorld + Struct,
        S: Copy + AssetState,
    {
        self.add_systems((check_assets::<R, S>.run_if(resource_exists::<LoadingAssets<R>>()),))
            .add_event::<AssetLoaderEvent<R>>()
    }

    fn cleanup_assets_on_exit<R>(&mut self, state: impl AssetState) -> &mut Self
    where
        R: Resource,
    {
        self.add_systems((
            trigger_cleanup::<R>
                .in_schedule(OnExit(state))
                .run_if(resource_exists::<R>()),
            handle_cleanup::<R>.run_if(resource_exists::<AssetCleanUpTask<R>>()),
        ))
    }
}

/// When a state is scheduled([`ScheduleNextState`] is changed), invoke [`AssetState::load_assets`]
fn load_assets<S: AssetState>(
    commands: Commands,
    schedule_state: Res<ScheduleNextState<S>>,
    next_state: ResMut<NextState<S>>,
) {
    if let ScheduleNextState(Some(state)) = &*schedule_state {
        debug!("try to load assets for state {:?}, if any", *schedule_state);
        state.load_assets(commands, next_state);
    }
}

/// When assets are loading, check the progress and go to the scheduled
/// state once everything is loaded.
fn check_assets<R: Resource + Reflect + Struct, S: States + AssetState>(
    asset_server: Res<AssetServer>,
    assets: Res<R>,
    mut asset_event: EventWriter<AssetLoaderEvent<R>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<S>>,
    mut schedule_state: ResMut<ScheduleNextState<S>>,
) {
    debug!("checking assets {:?}", PhantomData::<R>);
    let handles = (*assets)
        .iter_fields()
        .map(|field| {
            *field
                .path::<HandleId>("id")
                .expect("could not get the asset handle of {field}")
        })
        .collect::<Vec<_>>();

    let all_loaded = handles.iter().all(|handle_id| {
        let loading_state = &asset_server.get_load_state(*handle_id);

        match loading_state {
            LoadState::Loaded => true,
            LoadState::Failed => {
                error!(
                    "could not load asset {:?} in {:?}",
                    asset_server.get_handle_path(*handle_id),
                    PhantomData::<R>
                );
                commands.remove_resource::<LoadingAssets<R>>();
                false
            }
            _ => false,
        }
    });

    if all_loaded {
        debug!("assets for {:?} have ben loaded", PhantomData::<R>);
        if let ScheduleNextState(Some(scheduled_next_state)) = &*schedule_state {
            asset_event.send(AssetLoaderEvent {
                handles,
                _marker: PhantomData,
            });

            next_state.set(*scheduled_next_state);
            schedule_state.clear();
            commands.remove_resource::<LoadingAssets<R>>();
        };
    }
}

/// A temporary resource for holding the cleanup task. Removed when task is done.
#[derive(Resource)]
pub struct AssetCleanUpTask<R: Resource>(PhantomData<R>);

/// A global timer to delay the [`AssetCleanUpTask`]
#[derive(Deref, DerefMut)]
pub struct GlobalAssetCleanUpTimer(Timer);

impl Default for GlobalAssetCleanUpTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(5.0, TimerMode::Once))
    }
}

/// A temporary resource for holding the cleanup task. Removed when task is done.
#[derive(Debug, Resource)]
pub struct AssetCleanUpTimer<R: Resource>(pub Timer, pub PhantomData<R>);

pub fn trigger_cleanup<R: Resource>(mut commands: Commands) {
    debug!(
        "cleanup for AssetCleanUpTask<{:?}> is triggered",
        AssetCleanUpTask(PhantomData::<R>).0
    );
    commands.insert_resource(AssetCleanUpTask(PhantomData::<R>));
}

pub fn handle_cleanup<R: Resource>(
    mut commands: Commands,
    time: Res<Time>,
    mut global_timer: Local<GlobalAssetCleanUpTimer>,
    dedicated_timer: Option<ResMut<AssetCleanUpTimer<R>>>,
) {
    // FIXME: move the inner if out
    if let Some(mut timer) = dedicated_timer {
        let timer = &mut timer.0;

        if timer.tick(time.delta()).just_finished() {
            debug!("cleaning up {:?}", PhantomData::<R>);
            commands.remove_resource::<AssetCleanUpTask<R>>();
            commands.remove_resource::<R>();
            timer.reset();
        }
    } else if global_timer.tick(time.delta()).just_finished() {
        debug!("cleaning up {:?}", PhantomData::<R>);
        commands.remove_resource::<AssetCleanUpTask<R>>();
        commands.remove_resource::<R>();
        global_timer.reset();
    }
}
