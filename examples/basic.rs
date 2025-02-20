use std::time::Duration;
use bevy::{
    core_pipeline::tonemapping::Tonemapping, pbr::{NotShadowCaster, NotShadowReceiver}, prelude::*, time::common_conditions::on_timer
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
    .add_systems(Update, switch_animation.run_if(on_timer(Duration::from_secs(5))))
    .run();
}

const ANI_COUNT_GIRL: usize = 13;
const ANI_COUNT_MAN: usize = 5;

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
        Transform::from_xyz(0., 3., -8.).looking_at(Vec3::ZERO.with_y(1.), Vec3::Y),
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

    let girl_path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/assets/girl.glb");
    let man_path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/assets/man.glb");

    all_animations.add("Girl", girl_path, ANI_COUNT_GIRL, &mut graphs, &assets);
    all_animations.add("Man", man_path, ANI_COUNT_MAN, &mut graphs, &assets);

    let sh_girl = assets.load(GltfAssetLabel::Scene(0).from_asset(girl_path));
    let sh_man = assets.load(GltfAssetLabel::Scene(0).from_asset(man_path));

    let mut x = -5.;
    for i in 0 .. 6 {
        x += 1.2;
        if i % 2 == 0 {
            cmd.spawn((
                SceneRoot(sh_girl.clone()),
                NotShadowCaster,
                NotShadowReceiver,
                AniData::new("Girl", i % ANI_COUNT_GIRL),
                Transform::from_xyz(x, 0., 0.)
            ));
        } else {
            cmd.spawn((
                SceneRoot(sh_man.clone()),
                NotShadowCaster,
                NotShadowReceiver,
                AniData::new("Man", i % ANI_COUNT_MAN),
                Transform::from_xyz(x, 0., 0.)
            ));

        }
    }
}

// ---

fn switch_animation(
    mut q: Query<&mut AniData>,
) {
    for mut ad in &mut q {
        ad.animation_index = (ad.animation_index + 1) % (if ad.animation_key == "Man" {ANI_COUNT_MAN} else {ANI_COUNT_GIRL});
    }
}