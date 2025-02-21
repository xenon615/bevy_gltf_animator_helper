# Bevy Gltf Animator Helper
Made as attempt to simplify process of animation of gltf models. 

Currently 2 examples available (simple and basic)

```
cargo r --example basic
```

## Usage
Add to your dependencies
```
[dependencies]
bevy_gltf_animator_helper = {git = "https://github.com/xenon615/bevy_gltf_animator_helper"}
```  
Then 
```
use bevy_gltf_animator_helper::{AllAnimations, AniData, AnimatorHelperPlugin};
```
Then
```
.add_plugins(
    AnimatorHelperPlugin
)
```
Then please look into example ("simple" or "basic") 

Assume you have 
```
.add_systems(Startup, startup)
```

so 
```
const ANI_COUNT: usize = 13;   //  count  animations to use from glb

fn startup(
    mut cmd: Commands,
    assets: ResMut<AssetServer>,
    mut all_animations: ResMut<AllAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,

) {
    all_animations.add("Lady", "girl.glb", ANI_COUNT, &mut graphs, &assets);  // "Lady" in this case - is animation key
    let sh = assets.load(GltfAssetLabel::Scene(0).from_asset("girl.glb"));

    cmd.spawn((
        SceneRoot(sh.clone()),
        AniData::new("Lady", 0),   // "Lady" - animation key, "0" in this case - initial animation index
    ));
}
```
switch animation

```rust
fn switch_animation(
    mut q: Query<&mut AniData>,
) {
    for mut ad in &mut q {
        ad.animation_index = (ad.animation_index + 1) % ANI_COUNT;
    }
}
```




