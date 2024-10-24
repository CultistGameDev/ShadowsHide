use animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;

use crate::assets::asset_path;

pub struct Player {
    pub pos: Vec2,
    pub dims: Vec2,
    pub offset: Vec2,
    pub idle_sprite: Texture2D,
    pub anim: AnimatedSprite,
}

impl Player {
    pub async fn new(pos: Vec2, dims: Vec2, offset: Vec2) -> Self {
        Player {
            pos,
            dims,
            offset,
            idle_sprite: load_texture(
                asset_path()
                    .join("sprites/anims/idle.png")
                    .to_str()
                    .unwrap(),
            )
            .await
            .expect("Failed to load texture"),
            anim: AnimatedSprite::new(
                64,
                64,
                &[Animation {
                    name: "idle".to_string(),
                    row: 0,
                    frames: 3,
                    fps: 12,
                }],
                true,
            ),
        }
    }

    pub fn movement(&mut self, dir: Vec2) {
        self.pos += dir;
    }

    pub fn draw(&mut self) {
        let frame = self.anim.frame();
        draw_texture_ex(
            &self.idle_sprite,
            self.pos.x - self.dims.x / 2.0,
            self.pos.y - self.dims.x / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.dims),
                source: Some(frame.source_rect),
                ..Default::default()
            },
        );
        self.anim.update();
    }
}
