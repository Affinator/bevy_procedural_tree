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

        app.add_systems(PostUpdate, update_all_tree_meshes_with_global_settings.run_if(resource_changed::<TreeMeshSettings>));
        app.add_systems(PostUpdate, update_all_tree_meshes_with_local_settings);
        app.add_systems(PostUpdate, update_all_tree_textures_when_changed);
    }
}



#[derive(Component, Reflect, Clone, Debug)]
#[component(on_add = new_tree_component_added)]
pub struct Tree {
    /// the seed for the rng (same seed and TreeMeshSettings = same tree mesh)
    /// the seed is always local to each tree instance (regardless if the tree is using global TreeMeshSettings)
    pub seed: u64,
    /// the settings to use for this tree; if set to none the settings from the global TreeMeshSettings resource are used
    pub tree_mesh_settings: Option<TreeMeshSettings>,
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
    let tree: Tree = (*world.entity(tree_entity).components::<&Tree>()).clone();
    let tree_mesh_settings = tree.tree_mesh_settings.or_else(
      || {
            world.get_resource::<TreeMeshSettings>().cloned()
      }  
    ).unwrap();

    let mut rng: Rng = Rng::with_seed(tree.seed);
    let (branches_mesh, leaves_mesh) = generate_tree(&tree_mesh_settings, &mut rng);
    
    // retrieve AssetServer
    let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

    // meshes
    let branches_mesh = Mesh3d(meshes.add(branches_mesh));
    let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

    let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
    // bark material
    let branch_material = tree.bark_material.clone().or_else(
        || {
            let mut material = StandardMaterial::from(Color::WHITE);
            material.cull_mode = None;
            Some(MeshMaterial3d(materials.add(material)))
        }
    ).unwrap();
    // leaf material
    let leaf_material = tree.leaf_material.clone().or_else(
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
        branch_material
    )).add_child(leaves_id);

}

fn update_all_tree_textures_when_changed() {

}

fn update_all_tree_meshes_with_local_settings(
    trees: Query<(Entity, &Tree, &Leaves), Changed<Tree>>,
    mut meshes: ResMut<Assets<Mesh>>,
    global_tree_settings: Res<TreeMeshSettings>,
    mut commands: Commands,
)
{
    // For now we are regenerating the whole tree mesh each time 
    // TODO: Try to modify in place (or at least only branch/leaf levels or textures that need modification)
    for (tree_entity, tree, leaves_entity) in trees.iter() {
        let tree_settings: &TreeMeshSettings = match tree.tree_mesh_settings {
            Some(ref tree_settings) => tree_settings,
            None => global_tree_settings.as_ref(),
        };        
        
        let mut rng: Rng = Rng::with_seed(tree.seed);
        let (branches_mesh, leaves_mesh) = generate_tree(tree_settings, &mut rng);
        let branches_mesh = Mesh3d(meshes.add(branches_mesh));
        let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

        commands.entity(tree_entity).insert(branches_mesh);
        commands.entity(leaves_entity.0).insert(leaves_mesh);        
    }
}

fn update_all_tree_meshes_with_global_settings(
    trees: Query<(Entity, &Tree, &Leaves)>,
    tree_settings: Res<TreeMeshSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    // For now we are regenerating the whole tree mesh each time 
    // TODO: Try to modify in place (or at least only branch/leaf levels or textures that need modification)

    for (tree_entity, tree, leaves_entity) in trees.iter() {
        if tree.tree_mesh_settings.is_none() {
            let mut rng: Rng = Rng::with_seed(tree.seed);
            let (branches_mesh, leaves_mesh) = generate_tree(&tree_settings, &mut rng);
            let branches_mesh = Mesh3d(meshes.add(branches_mesh));
            let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

            commands.entity(tree_entity).insert(branches_mesh);
            commands.entity(leaves_entity.0).insert(leaves_mesh);
        }
    }

}