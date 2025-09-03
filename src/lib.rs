pub mod enums;
pub mod settings;

mod meshgen;

use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*};
use fastrand::Rng;

use crate::{meshgen::generate_branches, settings::TreeSettings};


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

    // Generate meshes
    // TODO: remove unwrap
    let settings = world.get_resource::<TreeSettings>().unwrap();
    let settings: TreeSettings = settings.clone();
    let mut rng: Rng = Rng::with_seed(settings.seed);
    let branches_mesh = generate_branches(&settings, &mut rng);
    
    // Add to AssetServer
    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
    let mesh = Mesh3d(meshes.add(branches_mesh));
    let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
    let mut material = StandardMaterial::from(Color::WHITE);
    material.cull_mode = None;
    let mesh_material = MeshMaterial3d(materials.add(material));

    // Spawn/Insert
    let mut commands = world.commands();
    let mut tree_commands = commands.entity(tree_entity);
    tree_commands.insert((
        Name::new("ProcGenTree"),
        mesh,
        mesh_material,
        settings
    ));

}

fn update_tree(
    trees: Query<Entity, With<Tree>>,
    tree_settings: Res<TreeSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    info!("Updating trees, due to changed TreeSettings...");
    // For now we are regenerating the tree each time 
    // TODO: Try to modify in place (or at least only branch/leaf levels that need modification)
    let mut rng: Rng = Rng::with_seed(tree_settings.seed);
    let branches_mesh = generate_branches(&tree_settings, &mut rng);

    let mesh = Mesh3d(meshes.add(branches_mesh));

    commands.entity(trees.single().unwrap()).insert(mesh);
}