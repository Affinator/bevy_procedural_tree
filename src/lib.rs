use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*};

#[cfg(feature="inspector")]
use bevy_inspector_egui::prelude::*;


pub struct TreeProceduralGenerationPlugin;

impl Plugin for TreeProceduralGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeGenSettings>();
        app.register_type::<TreeGenSettings>();

        app.add_systems(PostUpdate, update_tree.run_if(resource_changed::<TreeGenSettings>));
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


#[derive(Component, Reflect)]
#[component(on_add = new_tree_component_added)]
pub struct Tree;

fn new_tree_component_added(mut world: DeferredWorld, context: HookContext) {
    info!("New tree component added to entity");
    let entity = context.entity;
    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let mesh = Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0)));
    let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
    let material = MeshMaterial3d(materials.add(Color::WHITE));

    let mut commands = world.commands();
    let trunk_id = commands.spawn((
        mesh,
        material,
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
    )).id();

    let mut tree_commands = commands.entity(entity);
    tree_commands.add_child(trunk_id);

}

fn update_tree() {
    info!("Updating tree, due to changed TreeGenSettings...");
}