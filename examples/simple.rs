use bevy::{
    core_pipeline::tonemapping::Tonemapping, pbr::{NotShadowCaster, NotShadowReceiver}, prelude::*
};
use bevy_gltf_animator_helper::{AllAnimations, AniData, AnimatorHelperPlugin};

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins,
        AnimatorHelperPlugin
    ))
    .add_systems(Startup, startup)
    .run();
}

fn startup(
    mut cmd: Commands,
    mut al : ResMut<AmbientLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: ResMut<AssetServer>,
    mut all_animations: ResMut<AllAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,

) {
    al.brightness = 200.;
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(1., 2., -6.).looking_at(Vec3::ZERO.with_y(1.), Vec3::Y),
        Tonemapping::ReinhardLuminance
    ));
    cmd.spawn((
        DirectionalLight {
            illuminance: 5_000.,
            ..default()
        },
        Transform::IDENTITY.looking_at(Vec3::Z, Vec3::Y)
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(50.)))),
        MeshMaterial3d(materials.add(Color::BLACK)),
    ));

    let gltf_path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/assets/bv.glb");
    all_animations.add("BV", gltf_path, 1, &mut graphs, &assets);

    let sh = assets.load(GltfAssetLabel::Scene(0).from_asset(gltf_path));

    cmd.spawn((
        SceneRoot(sh.clone()),
        NotShadowCaster,
        NotShadowReceiver,
        AniData::new("BV", 0),
    ));
}

// ---

