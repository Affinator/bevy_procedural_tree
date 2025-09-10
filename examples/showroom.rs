use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_procedural_tree::settings::TreeMeshSettings;
use bevy_procedural_tree::{Tree, TreeProceduralGenerationPlugin};
use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(EntityCountDiagnosticsPlugin)
    .add_plugins(SystemInformationDiagnosticsPlugin)
    .add_plugins(RenderDiagnosticsPlugin)
    .add_plugins(TreeProceduralGenerationPlugin)
    .add_plugins(PerfUiPlugin)
    .add_plugins(EguiPlugin::default())
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(ResourceInspectorPlugin::<TreeMeshSettings>::default())
    .add_systems(Startup, setup)
    .run();
}



/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // perf ui
    commands.spawn(PerfUiAllEntries::default());

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(7.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 2.0, 9.0).looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
        Tonemapping::None,
    ));

    // tree
    let bark_texture_color: Handle<Image> = asset_server.load("textures/bark_willow/color.png");
    let bark_texture_normal: Handle<Image> = asset_server.load("textures/bark_willow/normal_gl.png");
    let bark_texture_roughness: Handle<Image> = asset_server.load("textures/bark_willow/roughness.png");
    // let bark_texture_displacement: Handle<Image> = asset_server.load("textures/bark_willow/displacement.png");
    let bark_material = Some(MeshMaterial3d(materials.add(
        StandardMaterial {
            base_color_texture: Some(bark_texture_color),
            normal_map_texture: Some(bark_texture_normal),
            metallic_roughness_texture: Some(bark_texture_roughness),
            perceptual_roughness: 1.0,
            reflectance: 0.1,
            // depth_map: Some(bark_texture_displacement),
            ..Default::default()
        }
    )));

    let leaf_texture_color: Handle<Image> = asset_server.load("textures/deciduous_leaves/color.png");
    let leaf_texture_normal: Handle<Image> = asset_server.load("textures/deciduous_leaves/normal_gl.png");
    let leaf_texture_roughness: Handle<Image> = asset_server.load("textures/deciduous_leaves/roughness.png");
    let leaf_material = Some(MeshMaterial3d(materials.add(
        StandardMaterial {
            base_color_texture: Some(leaf_texture_color),
            normal_map_texture: Some(leaf_texture_normal),
            metallic_roughness_texture: Some(leaf_texture_roughness),
            perceptual_roughness: 1.0,
            reflectance: 0.1,
            cull_mode: None, // show leaves from both sides (makes the tree "fuller")
            double_sided: true,
            alpha_mode: AlphaMode::Mask(0.5),
            ..Default::default()
        }
    )));

    // tree with local settings
    commands.spawn((
        Tree {
            seed: 0,
            tree_mesh_settings: Some(TreeMeshSettings::default()), // set to None to fallback to the global resource
            bark_material: bark_material.clone(),
            leaf_material: leaf_material.clone(),
        },
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));

    // tree using global settings (Res<TreeMeshSettings>)
    commands.spawn((
        Tree {
            seed: 0,
            tree_mesh_settings: None, // set to None to fallback to the global resource
            bark_material,
            leaf_material,
        },
        Transform::from_xyz(4.0, 0.0, -3.0)
    ));
}
