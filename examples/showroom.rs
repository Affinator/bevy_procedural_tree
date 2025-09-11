use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::input::mouse::{AccumulatedMouseMotion, MouseWheel};
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
    .add_systems(Update, orbit)
    .run();
}

const TARGET_CAMERA_FOCUS: Vec3 = Vec3 { x: 0.0, y: 2.5, z: 0.0 };

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
        Transform::from_xyz(-2.5, 2.0, 9.0).looking_at(TARGET_CAMERA_FOCUS, Vec3::Y),
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

    // tree using global settings (Res<TreeMeshSettings>)
    commands.spawn((
        Tree {
            seed: 0,
            tree_mesh_settings: None, // set to None to fallback to the global resource
            bark_material: bark_material.clone(),
            leaf_material: leaf_material.clone(),
        },
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));

    // tree with local settings
    commands.spawn((
        Tree {
            seed: 0,
            tree_mesh_settings: Some(TreeMeshSettings::default()), // set to None to fallback to the global resource
            bark_material,
            leaf_material,
        },
        Transform::from_xyz(4.0, 0.0, -3.0)
    ));
}


fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    // time: Res<Time>,
) {
    let delta = mouse_motion.delta;
    
    // Limiting pitch stops some unexpected rotation past 90° up or down.
    const PITCH_LIMIT:f32 = core::f32::consts::FRAC_PI_2 - 0.0001;
    let mut orbit_distance: f32 = (camera.translation - TARGET_CAMERA_FOCUS).length();

    // rotation
    if mouse_buttons.pressed(MouseButton::Left) {
        // Mouse motion is one of the few inputs that should not be multiplied by delta time,
        // as we are already receiving the full movement since the last frame was rendered. Multiplying
        // by delta time here would make the movement slower that it should be.
        let delta_pitch = delta.y * 0.002;
        let delta_yaw = delta.x * 0.002;

        // Obtain the existing pitch, yaw, and roll values from the transform.
        let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

        // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        let yaw = yaw + delta_yaw;
        camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

        // Adjust the translation to maintain the correct orientation toward the orbit target.
        // In our example it's a static target, but this could easily be customized.
        camera.translation = TARGET_CAMERA_FOCUS - camera.forward() * orbit_distance;
    }

    // mouse wheel zoom
    for mouse_wheel_event in mouse_wheel_events.read() {
        orbit_distance = (orbit_distance + mouse_wheel_event.y).clamp(0.1, 25.0);
        
        // Adjust the translation to maintain the correct orientation toward the orbit target.
        // In our example it's a static target, but this could easily be customized.
        camera.translation = TARGET_CAMERA_FOCUS - camera.forward() * orbit_distance;
    }
}
