pub struct GameSettings {
    pub player_move_speed_x: f32,
    pub player_move_speed_y: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            player_move_speed_x: 1.0,
            player_move_speed_y: 1.0,
        }
    }
}

pub static ASSET_DIR: &'static str = "assets";
