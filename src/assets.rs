use macroquad::prelude::*;

pub static SHADOWBLOB_SHADER_VERT: &str = include_str!("../assets/shaders/shadowblob.vert");
pub static SHADOWBLOB_SHADER_FRAG: &str = include_str!("../assets/shaders/shadowblob.frag");

pub struct Assets {
    pub player_walk: Texture2D,
    pub player_idle: Texture2D,
}

impl Assets {
    pub async fn new() -> Result<Assets, macroquad::Error> {
        let player_walk: Texture2D = load_texture("sprites/anims/walk.png").await?;
        player_walk.set_filter(FilterMode::Nearest);
        let player_idle: Texture2D = load_texture("sprites/anims/idle.png").await?;
        player_walk.set_filter(FilterMode::Nearest);
        build_textures_atlas();

        Ok(Assets {
            player_walk,
            player_idle,
        })
    }
}
