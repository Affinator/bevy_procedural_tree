pub mod enums;
pub mod settings;
pub mod errors;

mod meshgen;

use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*};
use fastrand::Rng;

use crate::{meshgen::generate_tree, settings::TreeMeshSettings};


pub struct TreeProceduralGenerationPlugin;

impl Plugin for TreeProceduralGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeMeshSettings>();
        app.register_type::<TreeMeshSettings>();
        app.init_resource::<TreeDefaultMaterials>();
        app.register_type::<TreeDefaultMaterials>();
        app.register_type::<Tree>();
        app.register_type::<Leaves>();

        app.add_systems(PostUpdate, update_all_tree_meshes_with_global_settings.run_if(resource_changed::<TreeMeshSettings>));
        app.add_systems(PostUpdate, update_all_tree_meshes_with_local_settings);
    }
}



#[derive(Component, Reflect, Clone, Debug)]
#[component(on_add = new_tree_component_added)]
pub struct Tree {
    /// the seed for the rng (same seed and TreeMeshSettings = same tree mesh)
    /// the seed is always local to each tree instance (regardless if the tree is using global TreeMeshSettings)
    pub seed: u64,
    /// the settings to use for this tree; if set to none the settings from the global TreeMeshSettings resource are used
    pub tree_mesh_settings_override: Option<TreeMeshSettings>,
    /// defaults to Color::WHITE
    pub bark_material_override: Option<MeshMaterial3d<StandardMaterial>>,
    /// defaults to green -> Color::LinearRgba(LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }
    /// recommendation: AlphaMode::Mask(0.x) is recommend to be set for the leaves (depending on the texture used)
    pub leaf_material_override: Option<MeshMaterial3d<StandardMaterial>>,
}


#[derive(Resource, Reflect)]
struct TreeDefaultMaterials {
    /// defaults to Color::WHITE
    pub bark_material: MeshMaterial3d<StandardMaterial>,
    /// defaults to green -> Color::LinearRgba(LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }
    /// recommendation: AlphaMode::Mask(0.x) is recommend to be set for the leaves (depending on the texture used)
    pub leaf_material: MeshMaterial3d<StandardMaterial>,
}

impl FromWorld for TreeDefaultMaterials {
    fn from_world(world: &mut World) -> Self {        
        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
        Self {
            bark_material: MeshMaterial3d(materials.add(Color::WHITE)),
            leaf_material: MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 })))
        }
    }
}

#[derive(Component, Reflect)]
struct Leaves(Entity);

fn new_tree_component_added(mut world: DeferredWorld, context: HookContext) {
    let tree_entity = context.entity;

    // Generate meshes
    // TODO: remove unwrap
    let tree: Tree = (*world.entity(tree_entity).components::<&Tree>()).clone();
    let tree_mesh_settings = tree.tree_mesh_settings_override.or_else(
      || {
            world.get_resource::<TreeMeshSettings>().cloned()
      }  
    ).unwrap();

    let mut rng: Rng = Rng::with_seed(tree.seed);

    match generate_tree(&tree_mesh_settings, &mut rng) {
        Ok((branches_mesh, leaves_mesh)) => {
            // retrieve AssetServer
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

            // meshes
            let branches_mesh = Mesh3d(meshes.add(branches_mesh));
            let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

            let default_materials = world.get_resource::<TreeDefaultMaterials>().unwrap();
            // bark material
            let branch_material = tree.bark_material_override.clone().unwrap_or_else(|| default_materials.bark_material.clone());
            // leaf material
            let leaf_material = tree.leaf_material_override.clone().unwrap_or_else(|| default_materials.leaf_material.clone());

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
        },
        Err(err) => error!("Error during tree mesh generation: {}", err),
    }
}


fn update_all_tree_meshes_with_local_settings(
    trees: Query<(Entity, &Tree, &MeshMaterial3d<StandardMaterial>, &Leaves), Changed<Tree>>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    global_tree_settings: Res<TreeMeshSettings>,
    default_materials: Res<TreeDefaultMaterials>,
    mut commands: Commands,
)
{
    // For now we are regenerating the whole tree mesh each time 
    // TODO: Try to modify in place (or at least only branch/leaf levels or textures that need modification)
    for (tree_entity, tree, current_bark_material, leaves_entity) in trees.iter() {
        let tree_settings: &TreeMeshSettings = match tree.tree_mesh_settings_override {
            Some(ref tree_settings) => tree_settings,
            None => global_tree_settings.as_ref(),
        };        
        
        let mut rng: Rng = Rng::with_seed(tree.seed);

        match generate_tree(tree_settings, &mut rng) {
            Ok((branches_mesh, leaves_mesh)) => {
                let branches_mesh = Mesh3d(meshes.add(branches_mesh));
                let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

                commands.entity(tree_entity).insert(branches_mesh);
                commands.entity(leaves_entity.0).insert(leaves_mesh);        

                // check if the textures changed
                match tree.bark_material_override { // what is the target state of the bark material
                    Some(ref bark_material_from_local_settings) => {
                        if !current_bark_material.eq(bark_material_from_local_settings) {
                            commands.entity(tree_entity).insert(bark_material_from_local_settings.clone());
                        }
                    },
                    None => {
                        if !current_bark_material.eq(&default_materials.bark_material) {
                            commands.entity(tree_entity).insert(default_materials.bark_material.clone());
                        }
                    },
                }

                if let Ok(current_leaf_material) = mesh_materials.get(leaves_entity.0) {
                    match tree.leaf_material_override { // what is the target state of the leaf material
                        Some(ref leaf_material_from_local_settings) => {
                            if !current_leaf_material.eq(leaf_material_from_local_settings) {
                                commands.entity(leaves_entity.0).insert(leaf_material_from_local_settings.clone());
                            }
                        },
                        None => {
                            if !current_leaf_material.eq(&default_materials.leaf_material) {
                                commands.entity(leaves_entity.0).insert(default_materials.leaf_material.clone());
                            }
                        },
                    }
                }
            },
            Err(err) => error!("Error during tree mesh generation: {}", err),
        }
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
        if tree.tree_mesh_settings_override.is_none() {
            let mut rng: Rng = Rng::with_seed(tree.seed);

            match generate_tree(&tree_settings, &mut rng) {
                Ok((branches_mesh, leaves_mesh)) => {
                    let branches_mesh = Mesh3d(meshes.add(branches_mesh));
                    let leaves_mesh = Mesh3d(meshes.add(leaves_mesh));

                    commands.entity(tree_entity).insert(branches_mesh);
                    commands.entity(leaves_entity.0).insert(leaves_mesh);
                },
                Err(err) => error!("Error during tree mesh generation: {}", err),
            }
        }
    }

}