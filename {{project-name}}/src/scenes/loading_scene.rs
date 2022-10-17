use bevy::prelude::*;

use crate::config::*;
use crate::resources::{
    Game,
    GameLevel,
    CarSpawnerData,
};
use crate::events::CollisionEvent;
use crate::scenes::SceneState;


pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Game>();
        app.add_event::<CollisionEvent>();
        app.add_system_set(
            SystemSet::on_enter(SceneState::LoadingScene).with_system(setup_game_data),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::LoadingScene).with_system(update_loader),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::LoadingScene).with_system(cleanup));
        app.add_system(bevy::window::close_on_esc);
    }
}



fn setup_game_data(
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let game_music = asset_server.load("sounds/game_music.ogg");
    audio.play_with_settings(
        game_music,
        PlaybackSettings::LOOP.with_volume(0.25),
    );

    // Setup Levels
    game.levels = Vec::new();

    game.levels.push(GameLevel {
        // level_id: 1,
        playable_area: Vec2::new(1280.0, 720.0),
        background_image: "sprites/level1.png".to_string(),
        num_cultists: 10,
        cultists_center: Vec2::new(0.0, -330.0),
        cultists_box_size: Vec2::new(800.0, 60.0),
        car_spawners: vec![
            CarSpawnerData {
                spawn_rate: 1.5,
                position: Vec2::new(-MAP_WIDTH/2.0,-188.0),
                spawn_direction: Vec2::new(520.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.32,
                position: Vec2::new(-MAP_WIDTH/2.0,-88.0),
                spawn_direction: Vec2::new(500.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.44,
                position: Vec2::new(MAP_WIDTH/2.0,112.0),
                spawn_direction: Vec2::new(-550.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.27,
                position: Vec2::new(MAP_WIDTH/2.0,212.0),
                spawn_direction: Vec2::new(-480.0,0.0),
            },
            ],
            ritual_sites: vec![
                Vec2::new(-330.0, -330.0),
                Vec2::new(330.0, -330.0),
                Vec2::new(0.0, 0.0),
            ],
            koolaid_stand: Vec2::new(50.0, 420.0),
    });

    game.levels.push(GameLevel {
        // level_id: 1,
        playable_area: Vec2::new(1280.0, 720.0),
        background_image: "sprites/level1.png".to_string(),
        num_cultists: 10,
        cultists_center: Vec2::new(0.0, -330.0),
        cultists_box_size: Vec2::new(800.0, 60.0),
        car_spawners: vec![
            CarSpawnerData {
                spawn_rate: 0.85,
                position: Vec2::new(-MAP_WIDTH/2.0,-188.0),
                spawn_direction: Vec2::new(520.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.0,
                position: Vec2::new(-MAP_WIDTH/2.0,-88.0),
                spawn_direction: Vec2::new(500.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.0,
                position: Vec2::new(MAP_WIDTH/2.0,112.0),
                spawn_direction: Vec2::new(-550.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.9,
                position: Vec2::new(MAP_WIDTH/2.0,212.0),
                spawn_direction: Vec2::new(-480.0,0.0),
            },
            ],
            ritual_sites: vec![
                Vec2::new(-330.0, -330.0),
                Vec2::new(330.0, -330.0),
                Vec2::new(0.0, -330.0),
                Vec2::new(-100.0, 0.0),
                Vec2::new(100.0, 0.0),
            ],
            koolaid_stand: Vec2::new(50.0, 420.0),
    });

    game.levels.push(GameLevel {
        // level_id: 2,
        playable_area: Vec2::new(1920.0, 1080.0),
        background_image: "sprites/level2.png".to_string(),
        num_cultists: 10,
        cultists_center: Vec2::new(0.0, -520.0),
        cultists_box_size: Vec2::new(800.0, 60.0),
        car_spawners: vec![
            CarSpawnerData {
                spawn_rate: 0.85,
                position: Vec2::new(-MAP_WIDTH/2.0,-400.0),
                spawn_direction: Vec2::new(550.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.77,
                position: Vec2::new(-MAP_WIDTH/2.0,-300.0),
                spawn_direction: Vec2::new(530.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.85,
                position: Vec2::new(-MAP_WIDTH/2.0,-200.0),
                spawn_direction: Vec2::new(520.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.0,
                position: Vec2::new(-MAP_WIDTH/2.0,-100.0),
                spawn_direction: Vec2::new(500.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.88,
                position: Vec2::new(MAP_WIDTH/2.0,100.0),
                spawn_direction: Vec2::new(-550.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.95,
                position: Vec2::new(MAP_WIDTH/2.0,200.0),
                spawn_direction: Vec2::new(-600.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.8,
                position: Vec2::new(MAP_WIDTH/2.0,300.0),
                spawn_direction: Vec2::new(-400.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.1,
                position: Vec2::new(MAP_WIDTH/2.0,400.0),
                spawn_direction: Vec2::new(-700.0,0.0),
            },
            ],
            ritual_sites: vec![
                Vec2::new(-200.0, -500.0),
                Vec2::new(200.0, -500.0),
                Vec2::new(-330.0, 0.0),
                Vec2::new(330.0, 0.0),
                Vec2::new(0.0, 0.0),
                Vec2::new(-200.0, 500.0),
                Vec2::new(200.0, 500.0),
            ],
            koolaid_stand: Vec2::new(0.0, 650.0),
    });

    game.levels.push(GameLevel {
        // level_id: 2,
        playable_area: Vec2::new(1920.0, 1080.0),
        background_image: "sprites/level2.png".to_string(),
        num_cultists: 10,
        cultists_center: Vec2::new(0.0, -520.0),
        cultists_box_size: Vec2::new(800.0, 60.0),
        car_spawners: vec![
            CarSpawnerData {
                spawn_rate: 0.85,
                position: Vec2::new(-MAP_WIDTH/2.0,-400.0),
                spawn_direction: Vec2::new(550.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.77,
                position: Vec2::new(-MAP_WIDTH/2.0,-340.0),
                spawn_direction: Vec2::new(530.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.85,
                position: Vec2::new(-MAP_WIDTH/2.0,-280.0),
                spawn_direction: Vec2::new(520.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.0,
                position: Vec2::new(-MAP_WIDTH/2.0,-220.0),
                spawn_direction: Vec2::new(500.0,0.0),
            },

            CarSpawnerData {
                spawn_rate: 0.88,
                position: Vec2::new(MAP_WIDTH/2.0,-100.0),
                spawn_direction: Vec2::new(-550.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.95,
                position: Vec2::new(MAP_WIDTH/2.0,-40.0),
                spawn_direction: Vec2::new(-600.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.8,
                position: Vec2::new(MAP_WIDTH/2.0,20.0),
                spawn_direction: Vec2::new(-400.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.1,
                position: Vec2::new(MAP_WIDTH/2.0,80.0),
                spawn_direction: Vec2::new(-700.0,0.0),
            },

            CarSpawnerData {
                spawn_rate: 0.85,
                position: Vec2::new(-MAP_WIDTH/2.0,200.0),
                spawn_direction: Vec2::new(550.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.77,
                position: Vec2::new(-MAP_WIDTH/2.0,260.0),
                spawn_direction: Vec2::new(530.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 0.85,
                position: Vec2::new(-MAP_WIDTH/2.0,320.0),
                spawn_direction: Vec2::new(520.0,0.0),
            },
            CarSpawnerData {
                spawn_rate: 1.0,
                position: Vec2::new(-MAP_WIDTH/2.0,380.0),
                spawn_direction: Vec2::new(500.0,0.0),
            },
            ],
            ritual_sites: vec![
                Vec2::new(-100.0, -500.0),
            ],
            koolaid_stand: Vec2::new(50.0, 500.0),
    });
}

fn update_loader(
    mut state: ResMut<State<SceneState>>,
) {
    state
        .set(SceneState::MainMenuScene)
        .expect("Couldn't switch state to Main Menu Scene");
}

fn cleanup() {
    println!("Cleanup LoadingScene");
}
