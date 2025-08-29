use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_procedural_tree::{TreeGenSettings, TreeProceduralGenerationPlugin};
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
    .add_plugins(ResourceInspectorPlugin::<TreeGenSettings>::default())
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
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Tonemapping::None,
    ));
}
