use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy::utils::HashMap;
use std::time::Duration;

// ---
// ubused at the moment

// #[derive(Event)]
// pub struct AnimationChanged;


#[derive(Component)]
pub struct AniData {
    pub animation_index: usize,
    /// current animation index 
    pub player_entity: Entity,
    // entity of AnimatiionPlayer component (for convenience)
    pub animation_key: &'static str
    // key for link with animation set
}

impl AniData {
    pub fn new(key: &'static str, index: usize) -> Self {
        Self {
            animation_key: key,
            animation_index: index,
            ..default()
        }
    }
}

impl Default for AniData {
    fn default() -> Self {
        Self{animation_key: "", animation_index: 0, player_entity: Entity::PLACEHOLDER}
    }
}

struct AnimationSet {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

#[derive(Resource)]
pub struct AllAnimations(HashMap<&'static str, AnimationSet>);

impl AllAnimations {
    pub fn add(&mut self, key: &'static str, path: &'static str, count: usize, graphs: &mut ResMut<Assets<AnimationGraph>>, assets: &ResMut<AssetServer>) {
        if self.0.contains_key(&key) {
            return;
        }
        let mut graph = AnimationGraph::new();
        self.0.insert(
            key, 
            AnimationSet {
                animations: graph
                    .add_clips((0..count).map(|i| {assets.load(GltfAssetLabel::Animation(i).from_asset(
                        path
                    ))}), 1.0, graph.root)
                    .collect(),
                graph: graphs.add(graph),
            }
        );
    }
}

// ---

pub struct AnimatorHelperPlugin;
impl Plugin for AnimatorHelperPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AllAnimations(HashMap::new()))
        .add_observer(setup)
        // I haven't decided yet which method to use
        .add_systems(Update, switch)  // this ?
        // .add_observer(switch)     // or that ? 
        ;
    }
}

// ---

fn setup(
    tr: Trigger<SceneInstanceReady>,
    mut cmd: Commands,
    all_animations: Res<AllAnimations>,
    mut akq: Query<&mut AniData>,
    mut player_q: Query<&mut AnimationPlayer>,
    children: Query<&Children> 
) {
    let parent_e = tr.entity();

    let Ok(mut ani_data) = akq.get_mut(parent_e) else {
        return;
    };

    let Some(ani_set) = all_animations.0.get(ani_data.animation_key) else {
        return;
    };

    for c in children.iter_descendants(parent_e) {
        if let Ok(mut player) = player_q.get_mut(c) {
            // let initial_animation = ani_set.animations.len() - 1;
            let initial_animation = ani_data.animation_index;
            let mut transitions = AnimationTransitions::new();
            transitions
                .play(&mut player, ani_set.animations[initial_animation] , Duration::ZERO)
                .repeat()
            ;
            cmd
                .entity(c)
                .insert(AnimationGraphHandle(ani_set.graph.clone()))
                .insert(transitions)
            ;
            ani_data.player_entity = c;
            ani_data.animation_index = initial_animation;
            break;
        }
    }    
}

// ---

pub fn switch(
    // _tr: Trigger<AnimationChanged>,  // this or that ?
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    objects_q: Query<&AniData, Changed<AniData>>,
    all_animations: Res<AllAnimations>,
) {
    for adata in objects_q.iter() {
        if let Ok((mut player, mut transitions)) = animation_players.get_mut(adata.player_entity) {
            let ani_set = all_animations.0.get(adata.animation_key).unwrap();
            transitions
            .play(
                &mut player,
                ani_set.animations[adata.animation_index],
                Duration::from_millis(250),
            )
            .repeat();            
        }
    }
}
