use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

mod startup;
mod player;
mod physics;
mod walls;
mod ground_detection;
mod wall_climb;

use startup::setup;
use crate::player::PlayerPlugin;
//use crate::player::{react_to_player_changing, reader, PlayerPlugin}; //for testing

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            LdtkPlugin,
            RapierPhysicsPlugin::<()>::default(),
            RapierDebugRenderPlugin::default(), //for debugging colliders
        ))

        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))

        //implement player plugin
        .add_plugins(PlayerPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(ground_detection::GroundDetectionPlugin)
        .add_plugins(wall_climb::WallClimbPlugin)


        //player tests:
        //.add_systems(Update, player::reader)
        //.add_systems(Update, player::react_to_player_changing)

        .run();
}


