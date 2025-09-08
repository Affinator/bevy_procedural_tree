pub mod enums;
pub mod settings;

mod meshgen;

use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*};
use fastrand::Rng;

use crate::{meshgen::generate_tree, settings::TreeSettings};


pub struct TreeProceduralGenerationPlugin;

impl Plugin for TreeProceduralGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeSettings>();
        app.register_type::<TreeSettings>();
        app.register_type::<Tree>();
        app.register_type::<TreeTextureSettings>();
        app.register_type::<Leaves>();

        app.add_systems(PostUpdate, update_tree.run_if(resource_changed::<TreeSettings>));
    }
}



#[derive(Component, Reflect)]
#[require(TreeTextureSettings)]
#[component(on_add = new_tree_component_added)]
pub struct Tree;

#[derive(Component, Reflect, Default)]
pub struct TreeTextureSettings {
    pub bark_texture: Option<MeshMaterial3d<StandardMaterial>>,
    pub leaf_texture: Option<MeshMaterial3d<StandardMaterial>>
}

#[derive(Component, Reflect)]
struct Leaves(Entity);

fn new_tree_component_added(mut world: DeferredWorld, context: HookContext) {
    info!("New tree component added to entity");
    let tree_entity = context.entity;

    // Generate meshes
    // TODO: remove unwrap
    let settings = world.get_resource::<TreeSettings>().unwrap();
    let settings: TreeSettings = settings.clone();
    let mut rng: Rng = Rng::with_seed(settings.seed);
    let (branches_mesh, leaves_mesh) = generate_tree(&settings, &mut rng);
    
    // retrieve AssetServer
    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

    // meshes
    let branches_mesh = Mesh3d(meshes.add(branches_mesh));
    let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

    // materials
    let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
    let mut branch_material = StandardMaterial::from(Color::WHITE);
    branch_material.cull_mode = None;
    let mut leaves_material = StandardMaterial::from(Color::LinearRgba(LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }));
    leaves_material.cull_mode = None;
    let branch_material = MeshMaterial3d(materials.add(branch_material));
    let leaves_material = MeshMaterial3d(materials.add(leaves_material));


    // Spawn/Insert
    let mut commands = world.commands();
    let leaves_id = commands.spawn((
        Name::new("ProcGenTreeLeaves"),
        leaves_mesh,
        leaves_material,
    )).id();

    let mut tree_commands = commands.entity(tree_entity);
        
    tree_commands.insert((
        Name::new("ProcGenTreeBranches"),
        Leaves(leaves_id),
        branches_mesh,
        branch_material,
        settings
    )).add_child(leaves_id);

}

fn update_tree(
    trees: Query<(Entity, &Leaves), With<Tree>>,
    tree_settings: Res<TreeSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    info!("Updating trees, due to changed TreeSettings...");
    // For now we are regenerating the tree each time 
    // TODO: Try to modify in place (or at least only branch/leaf levels that need modification)
    let mut rng: Rng = Rng::with_seed(tree_settings.seed);

    for (tree_entity, leaves_entity) in trees.iter() {
        let (branches_mesh, leaves_mesh) = generate_tree(&tree_settings, &mut rng);
        let branches_mesh = Mesh3d(meshes.add(branches_mesh));
        let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

        commands.entity(tree_entity).insert(branches_mesh);
        commands.entity(leaves_entity.0).insert(leaves_mesh);
    }

}