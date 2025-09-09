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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // perf ui
    commands.spawn(PerfUiAllEntries::default());

    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
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

    // cube for comparison
    let height: f32 = 5.0;
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(height/10.0, height))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(3.0, height/2.0, 0.0)
    ));

    // tree
    let bark_material = Some(MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba { red: 0.5, green: 1.0, blue: 0.5, alpha: 1.0 }))));
    let leaf_material = Some(MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba { red: 0.2, green: 0.8, blue: 0.2, alpha: 0.9 }))));
    commands.spawn((
        Tree {
            bark_material,
            leaf_material,
        },
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}
