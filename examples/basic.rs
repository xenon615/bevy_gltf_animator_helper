use std::time::Duration;
use bevy::{
    core_pipeline::tonemapping::Tonemapping, 
    prelude::*,
    time::common_conditions::on_timer
};

use bevy_gltf_animator_helper::{AllAnimations, AniData, AnimatorHelperPlugin}; // necessary things

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins.set(
            AssetPlugin {
                file_path: "examples/assets".into(),
                ..default()
            }
        ),
        AnimatorHelperPlugin // don't forget to add this
    ))
    .add_systems(Startup, startup)
    .add_systems(Update, switch_animation.run_if(on_timer(Duration::from_secs(5))))
    .run();
    
}

// in this version of the plugin we need to know the number of animations in the model
// yuu can use Gltf Viewer (https://gltf-viewer.donmccurdy.com/) to find out this number
// you can choose a lower number to avoid loading unnecessary animations.

const ANIMATION_COUNT_M1: usize = 2;   // count for first model
const ANIMATION_COUNT_M2: usize = 2;   // count for second model
const ANIMATION_COUNT_M3: usize = 1;   // count for third model

fn startup(
    mut cmd: Commands,
    mut al : ResMut<AmbientLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: ResMut<AssetServer>,
    mut all_animations: ResMut<AllAnimations>,  // necessary things
    mut graphs: ResMut<Assets<AnimationGraph>>, // necessary things

) {
    al.brightness = 200.;
    cmd.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 3., -5.).looking_at(Vec3::ZERO.with_y(1.), Vec3::Y),
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

    // path to your models 
    // in your case it will most likely be a path relative to the "assets" folder

    let m1_path = "m1.glb";
    let m2_path = "m2.glb";
    let m3_path = "m3.glb";

    // created two animation sets , m1, m2 and m3 are animation keys
    all_animations.add("m1", m1_path, ANIMATION_COUNT_M1, &mut graphs, &assets);
    all_animations.add("m2", m2_path, ANIMATION_COUNT_M2, &mut graphs, &assets);
    all_animations.add("m3", m3_path, ANIMATION_COUNT_M3, &mut graphs, &assets);

    let sh_m1 = assets.load(GltfAssetLabel::Scene(0).from_asset(m1_path));
    let sh_m2 = assets.load(GltfAssetLabel::Scene(0).from_asset(m2_path));
    let sh_m3 = assets.load(GltfAssetLabel::Scene(0).from_asset(m3_path));

    cmd.spawn((
        SceneRoot(sh_m1.clone()),
        AniData::new("m1", 0),  // Parameters: "m1"  - animation key to link with the animation set defined above, initial animation index
        Transform::from_xyz(0., 0., 0.)
    ));

    cmd.spawn((
        SceneRoot(sh_m2.clone()),
        AniData::new("m2", 1), // look "m1" above
        Transform::from_xyz(-2., 0., 0.)
    ));

    cmd.spawn((
        SceneRoot(sh_m3.clone()),
        AniData::new("m3", 0), // look "m1" above
        Transform::from_xyz(2., 0., 0.).with_scale(Vec3::splat(0.25))
    ));

}

// ---

fn switch_animation(
    mut q: Query<&mut AniData>,
) {
    for mut ad in &mut q {
        // set new value for AniData animation index to switch animation

        let div = match ad.animation_key {
            "m1" => ANIMATION_COUNT_M1,
            "m2" => ANIMATION_COUNT_M2,
            _ => 1
        };

        ad.animation_index = (ad.animation_index + 1) % div;
    }
}