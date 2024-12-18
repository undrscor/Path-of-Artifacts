//animation.rs
use bevy::prelude::*;
use bevy::utils::*;

#[derive(Clone, Default, Bundle)]
pub struct AnimationBundle {
    pub animation_type: AnimationType,
    pub texture_atlas: TextureAtlas,
    pub sprite: SpriteBundle,
}

#[derive(Clone, Component, PartialEq, Eq, Copy, Debug, Hash, Default)]
pub enum AnimationType {
    #[default]
    Idle,
    Run,
    Jump,
    Grapple,
}

#[derive(Resource)]
pub struct AnimationAssets {
    layouts: HashMap<AnimationType, Handle<TextureAtlasLayout>>,
    textures: HashMap<AnimationType, Handle<Image>>,
    timers: HashMap<AnimationType, Timer>,
}
impl AnimationAssets {
    pub(crate) fn get_layout(&self, animation_type: AnimationType) -> Option<&Handle<TextureAtlasLayout>> {
        self.layouts.get(&animation_type)
    }

    pub(crate) fn get_texture(&self, animation_type: AnimationType) -> Option<&Handle<Image>> {
        self.textures.get(&animation_type)
    }

    pub(crate) fn get_timer_mut(&mut self, animation_type: AnimationType) -> Option<&mut Timer> {
        self.timers.get_mut(&animation_type)
    }
}

// This system should be run during startup to initialize the AnimationAtlases resource
fn setup_animation_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut layouts = HashMap::new();
    let mut textures = HashMap::new();
    let mut timers = HashMap::new();

    // Load textures and create layouts for each animation type
    let mut load_animation = |anim_type: AnimationType, path: String, columns: u32, rows: u32, frame_duration: f32| {
        let texture_handle: Handle<Image> = asset_server.load(path);
        textures.insert(anim_type, texture_handle);

        let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), columns, rows, None, None);
        let layout_handle = texture_atlases.add(layout);
        layouts.insert(anim_type, layout_handle);

        // Create and insert a timer for this animation typed
        let timer = Timer::from_seconds(frame_duration, TimerMode::Repeating);
        timers.insert(anim_type, timer);
    };
    load_animation(AnimationType::Jump, "man_jump_spritesheet.png".to_string(), 3, 3, 0.1);
    load_animation(AnimationType::Run, "man_walk_spritesheet.png".to_string(), 3, 2, 0.1);
    load_animation(AnimationType::Idle, "man_jump_spritesheet.png".to_string(), 1, 1, 0.0); // Use a single-frame texture for Idle.
    load_animation(AnimationType::Grapple, "man_grapple.png".to_string(), 3, 1, 0.0); // Use a single-frame texture for Idle.

    // Add more animations as needed
    commands.insert_resource(AnimationAssets { layouts, textures, timers });
}

pub struct PlayerAnimationPlugin;
impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_animation_assets);
    }
}