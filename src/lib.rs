pub mod enums;
pub mod settings;

mod meshgen;

use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*};
use fastrand::Rng;

use crate::{meshgen::generate_tree, settings::TreeMeshSettings};


pub struct TreeProceduralGenerationPlugin;

impl Plugin for TreeProceduralGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeMeshSettings>();
        app.register_type::<TreeMeshSettings>();
        app.register_type::<Tree>();
        app.register_type::<Leaves>();

        app.add_systems(PostUpdate, update_tree.run_if(resource_changed::<TreeMeshSettings>));
    }
}



#[derive(Component, Reflect, Clone, Debug)]
#[component(on_add = new_tree_component_added)]
pub struct Tree {
    /// defaults to Color::WHITE
    pub bark_material: Option<MeshMaterial3d<StandardMaterial>>,
    /// defaults to green -> Color::LinearRgba(LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }
    /// recommendation: AlphaMode::Mask(0.x) is recommend to be set for the leaves (depending on the texture used)
    pub leaf_material: Option<MeshMaterial3d<StandardMaterial>>,
}


#[derive(Component, Reflect)]
struct Leaves(Entity);

fn new_tree_component_added(mut world: DeferredWorld, context: HookContext) {
    info!("New tree component added to entity");
    let tree_entity = context.entity;

    // Generate meshes
    // TODO: remove unwrap
    let tree_settings = world.get_resource::<TreeMeshSettings>().unwrap();
    let tree_settings: TreeMeshSettings = tree_settings.clone();
    let texture_settings: Tree = (*world.entity(tree_entity).components::<&Tree>()).clone();

    let mut rng: Rng = Rng::with_seed(tree_settings.seed);
    let (branches_mesh, leaves_mesh) = generate_tree(&tree_settings, &mut rng);
    
    // retrieve AssetServer
    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

    // meshes
    let branches_mesh = Mesh3d(meshes.add(branches_mesh));
    let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

    let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
    // bark material
    let branch_material = texture_settings.bark_material.clone().or_else(
        || {
            let mut material = StandardMaterial::from(Color::WHITE);
            material.cull_mode = None;
            Some(MeshMaterial3d(materials.add(material)))
        }
    ).unwrap();
    // leaf material
    let leaf_material = texture_settings.leaf_material.clone().or_else(
        || {
            let mut material = StandardMaterial::from(Color::LinearRgba(LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }));
            material.cull_mode = None;
            material.alpha_mode = AlphaMode::Mask(0.5);
            Some(MeshMaterial3d(materials.add(material)))
        }
    ).unwrap();

    // Spawn/Insert
    let mut commands = world.commands();
    let leaves_id = commands.spawn((
        Name::new("ProcGenTreeLeaves"),
        leaves_mesh,
        leaf_material,
    )).id();

    let mut tree_commands = commands.entity(tree_entity);
        
    tree_commands.insert((
        Name::new("ProcGenTreeBranches"),
        Leaves(leaves_id),
        branches_mesh,
        branch_material,
        tree_settings
    )).add_child(leaves_id);

}

fn update_tree(
    trees: Query<(Entity, &Leaves), With<Tree>>,
    tree_settings: Res<TreeMeshSettings>,
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