mod app_ext;
mod commands_ext;

use bevy::prelude::{debug, Commands, Deref, DerefMut, NextState, ResMut, Resource, States};
use std::marker::PhantomData;

pub use crate::{app_ext::*, commands_ext::*};

/// A trait that is required to be derived on the state.
pub trait AssetState: Copy + States {
    /// Invoked in [`AssetLoaderAppExt::state_asset_loader`]
    fn load_assets(&self, commands: Commands, next_state: ResMut<NextState<Self>>);
}

/// A struct that replaces [`bevy::prelude::NextState`].
#[derive(Default, Debug, Deref, DerefMut, Resource)]
pub struct ScheduleNextState<S: States>(pub Option<S>);

impl<S: AssetState> ScheduleNextState<S> {
    pub fn set(&mut self, state: S) {
        debug!("setting {:?} to {:?}", self, state);
        self.0 = Some(state);
    }

    pub fn clear(&mut self) {
        debug!("resetting {:?} to None", self);
        self.0 = None;
    }
}

/// A temporary resource to holds the struct of assets being loaded. It is removed when the assets
/// are loaded.
#[derive(Resource)]
pub struct LoadingAssets<R: Resource>(pub PhantomData<R>);

impl<R: Resource> Default for LoadingAssets<R> {
    fn default() -> Self {
        Self(PhantomData::<R>)
    }
}

pub mod prelude {
    pub use super::*;
    pub use covey_asset_loader_macros::{AssetCollection, AssetState};
}
