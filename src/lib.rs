pub mod enums;
pub mod settings;

use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*, render::mesh::{CylinderAnchor, CylinderMeshBuilder}};

use crate::settings::TreeSettings;


pub struct TreeProceduralGenerationPlugin;

impl Plugin for TreeProceduralGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeSettings>();
        app.register_type::<TreeSettings>();

        app.add_systems(PostUpdate, update_tree.run_if(resource_changed::<TreeSettings>));
    }
}



#[derive(Component, Reflect)]
#[component(on_add = new_tree_component_added)]
pub struct Tree;

fn new_tree_component_added(mut world: DeferredWorld, context: HookContext) {
    info!("New tree component added to entity");
    let tree_entity = context.entity;
    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let mesh = Mesh3d(meshes.add(
        CylinderMeshBuilder::new(1.0, 1.0, 10).anchor(CylinderAnchor::Bottom)
    ));
    let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
    let material = MeshMaterial3d(materials.add(Color::WHITE));

    let mut commands = world.commands();
    let mut tree_commands = commands.entity(tree_entity);
    tree_commands.insert((
        Name::new("ProcGenTree"),
        mesh,
        material
    ));

}

fn update_tree(
    mut tree_transforms: Query<&mut Transform, With<Tree>>,
    tree_settings: Res<TreeSettings>
) {
    info!("Updating trees, due to changed TreeSettings...");
    for mut tree_transform in tree_transforms.iter_mut() {
        tree_transform.scale.y = tree_settings.branch.length[0];
        tree_transform.scale.x = tree_settings.branch.radius[0];
        tree_transform.scale.z = tree_settings.branch.radius[0];
    }
}