/*
* Inspiration taken with great thanks from: https://github.com/dgreenheck/ez-tree
*/

use bevy::prelude::*;

#[cfg(feature="inspector")]
use bevy_inspector_egui::prelude::*;


#[cfg(feature="inspector")]
#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct TreeGenSettings {
    #[inspector(min = 0.1, max = 5.0)]
    pub trunk_height: f32,
    #[inspector(min = 0.05, max = 1.5)]
    pub trunk_radius: f32
}

#[cfg(not(feature="inspector"))]
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TreeGenSettings {
    trunk_height: f32
}

impl Default for TreeGenSettings {
    fn default() -> Self {
        Self { 
            trunk_height: 2.0,
            trunk_radius: 0.4,
        }
    }
}