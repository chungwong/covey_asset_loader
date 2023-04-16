use bevy::prelude::{debug, Commands, FromWorld, Resource};
use std::marker::PhantomData;

use super::LoadingAssets;

pub trait AssetLoaderCommandsExt {
    fn init_assets<R: Resource + FromWorld>(&mut self);
}

impl<'w, 's> AssetLoaderCommandsExt for Commands<'w, 's> {
    fn init_assets<R: Resource + FromWorld>(&mut self) {
        debug!("initializing {:?}", PhantomData::<R>);
        self.init_resource::<R>();
        self.init_resource::<LoadingAssets<R>>();
    }
}
