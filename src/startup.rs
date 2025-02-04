use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::AudioSource;
use bevy_kira_audio::prelude::*;
use std::collections::HashMap;
use bevy::sprite::Anchor;

#[derive(Resource)]
pub struct LevelMusicMap {
    pub music_map: HashMap<String, Handle<AudioSource>>,
}

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>, _audio: Res<Audio>) {
    // Spawn a zoomed-in camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.5,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 999.9),
        ..Default::default()
    });

    // Spawn the world
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("LDTKmap.ldtk"),
        ..Default::default()
    });

    commands.spawn(
    Text2dBundle {
        text: Text::from_section(
            "Welcome!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                color: Color::WHITE,
            },
        ).with_justify(JustifyText::Center),
        visibility: Visibility::Visible,
        text_anchor: Anchor::Center,
        transform: Transform::from_xyz(470.0, -314.0, 5.0),

        ..default()
    });

    commands.spawn(
    Text2dBundle {
        text: Text::from_section(
            "WASD to move, space to jump, R to respawn",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                color: Color::WHITE,
            },
        ).with_justify(JustifyText::Center),
        visibility: Visibility::Visible,
        text_anchor: Anchor::Center,
        transform: Transform::from_xyz(900.0, -360.0, 5.0),

        ..default()
    });

    commands.spawn(
    Text2dBundle {
        text: Text::from_section(
            "Remember: you can always go back with R",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                color: Color::WHITE,
            },
        ).with_justify(JustifyText::Center),
        visibility: Visibility::Visible,
        text_anchor: Anchor::Center,
        transform: Transform::from_xyz(2270.0, -1010.0, 5.0),

        ..default()
    });

    commands.spawn(
    Text2dBundle {
        text: Text::from_section(
            "Dont fall!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                color: Color::WHITE,
            },
        ).with_justify(JustifyText::Center),
        visibility: Visibility::Visible,
        text_anchor: Anchor::Center,
        transform: Transform::from_xyz(-385.0, -100.0, 5.0),

        ..default()
    });

    commands.spawn(
    Text2dBundle {
        text: Text::from_section(
            "The Final Collectible!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.0,
                color: Color::WHITE,
            },
        ).with_justify(JustifyText::Center),
        visibility: Visibility::Visible,
        text_anchor: Anchor::Center,
        transform: Transform::from_xyz(3400.0, -225.0, 5.0),

        ..default()
    });

    // Create a HashMap for level-to-music mapping
    let mut level_music_map = HashMap::new();
    //Level_1
    level_music_map.insert(
        "78137f20-9b00-11ef-85d2-918c41126c86".to_string(),
        asset_server.load("sounds/grapple-Arena-dnb.ogg"),
    );
    //Level_2
    level_music_map.insert(
        "89f13410-9b00-11ef-85d2-030c58c7d34b".to_string(),
        asset_server.load("sounds/grapple-Arena-peaceful-sticatto.ogg"),
    );
    //Level_3
    level_music_map.insert(
        "aa737fe0-9b00-11ef-85d2-cd1f6eb084b1".to_string(),
        asset_server.load("sounds/CHRONO-grapple-arena.ogg"),
    );
    //Level_4
    level_music_map.insert(
        "f2f338f0-9b00-11ef-85d2-151712402bd4".to_string(),
        asset_server.load("sounds/DK-grapple-arena.ogg"),
    );
    //Level_0
    level_music_map.insert(
        "69cafc60-4ce0-11ef-ac02-af3d88f88f16".to_string(),
        asset_server.load("sounds/grapple.ogg"),
    );

    // Wrap the HashMap in LevelMusicMap and insert it as a resource
    commands.insert_resource(LevelMusicMap { music_map: level_music_map });

    // Play initial background music (optional)
    //audio.play(asset_server.load("grapple.ogg")).looped().with_volume(0.8);
}
//
// // Play background music
// let music: Handle<AudioSource> = asset_server.load("DK-grapple-arena.ogg");
// info!("Loaded music: {:?}", music);
// audio.play(music).looped().with_volume((0.8)); // Play the audio in a loop
//}
