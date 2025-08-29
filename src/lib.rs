use bevy::prelude::*;

#[cfg(feature="inspector")]
use bevy_inspector_egui::prelude::*;


pub struct TreeProceduralGenerationPlugin;

impl Plugin for TreeProceduralGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeGenSettings>();
        app.register_type::<TreeGenSettings>();
    }
}

#[cfg(feature="inspector")]
#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct TreeGenSettings {
    #[inspector(min = 0.1, max = 10.0)]
    trunk_height: f32
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
            trunk_height: 2.0
        }
    }
}