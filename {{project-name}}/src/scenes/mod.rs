pub mod loading_scene;
pub mod main_menu_scene;
pub mod level_select_scene;
pub mod playing_scene;
pub mod victory_scene;
pub mod game_over_scene;
pub mod quit_to_menu_scene;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    LoadingScene,
    MainMenuScene,
    LevelSelectScene,
    PlayingScene,
    VictoryScene,
    GameOverScene,
    QuitToMenuScene,
}
