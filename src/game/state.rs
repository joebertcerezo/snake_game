#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
    Won,
}
